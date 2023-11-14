use std::collections::HashMap;

use anyhow::Result;
use btleplug::api::{Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, PeripheralId};
use flutter_rust_bridge::StreamSink;
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::{self, mpsc, Mutex};

mod setup;
pub use setup::*;
pub mod device;
mod error;
pub mod event;
pub mod scan;
pub use error::Error;
use tokio::time;

use self::device::BleService;

enum Command {
    Scan {
        sink: StreamSink<Vec<scan::BleDevice>>,
        filter: Vec<String>,
    },
    Event {
        sink: StreamSink<event::BleEvent>,
    },
    Connect {
        id: String,
    },
    Disconnect {
        id: String,
    },
    DiscoverService {
        id: String,
        sink: StreamSink<Vec<BleService>>,
    },
}

impl std::fmt::Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scan").finish()
    }
}

/// Wrapper struct around btleplug::platform::Peripheral that adds the last_seen variable.
///
#[derive(Debug, Clone)]
struct Peripheral {
    peripheral: btleplug::platform::Peripheral,
    last_seen: time::Instant,
    is_connected: bool,
}

impl Peripheral {
    fn new(peripheral: btleplug::platform::Peripheral) -> Self {
        Self {
            peripheral,
            last_seen: time::Instant::now(),
            is_connected: false,
        }
    }

    fn id(&self) -> PeripheralId {
        self.peripheral.id()
    }

    async fn name(&self) -> Option<String> {
        if let Ok(Some(properties)) = self.peripheral.properties().await {
            properties.local_name
        } else {
            None
        }
    }

    async fn connect(&self) -> Result<()> {
        self.peripheral.connect().await?;
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        self.peripheral.disconnect().await?;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.is_connected
    }
}

static DEVICES: Lazy<Mutex<HashMap<String, Peripheral>>> = Lazy::new(|| Mutex::new(HashMap::new()));

static TX: OnceCell<mpsc::UnboundedSender<Command>> = OnceCell::new();

/// Internal send function to send [Command]s into the message loop.
fn send(command: Command) -> Result<()> {
    let tx = TX.get().ok_or(Error::TxNotInitialized)?;
    tx.send(command)?;
    Ok(())
}

/// The init() function must be called before anything else.
/// At the moment the developer has to make sure it is only called once.
pub fn init() -> Result<()> {
    create_runtime()?;
    let runtime = RUNTIME.get().ok_or(Error::RuntimeNotInitialized)?;

    let (tx, mut rx) = mpsc::unbounded_channel::<Command>();
    TX.set(tx).map_err(|_| Error::TxAlreadySet)?;

    runtime.spawn(async move {
        while let Some(msg) = rx.recv().await {
            match msg {
                Command::Scan { sink, filter } => {
                    tokio::spawn(async { scan::inner_scan(sink, filter).await.unwrap() });
                }
                Command::Event { sink } => {
                    tokio::spawn(async { event::inner_events(sink).await.unwrap() });
                }
                Command::Connect { id } => device::inner_connect(id).await.unwrap(),
                Command::Disconnect { id } => device::inner_disconnect(id).await.unwrap(),
                Command::DiscoverService { id, sink } => {
                    device::inner_discover_services(id, sink).await.unwrap()
                }
            }
        }
    });
    Ok(())
}

static CENTRAL: sync::OnceCell<Adapter> = sync::OnceCell::const_new();

async fn init_adapter() -> Adapter {
    let manager = Manager::new().await.expect("Init manager failed!");
    let adapters = manager.adapters().await.expect("Get adapters failed!");
    adapters.into_iter().next().expect("cannot fail")
}

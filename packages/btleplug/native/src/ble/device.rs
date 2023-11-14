use std::collections::BTreeSet;

use crate::logger::log;
use anyhow::Result;
use btleplug::api::{CharPropFlags, Peripheral, Service, Characteristic};
use flutter_rust_bridge::{RustOpaque, StreamSink};
use futures::Stream;

use super::{send, Command, Error, DEVICES};

/// A GATT service. Services are groups of characteristics, which may be standard or
/// device-specific.
pub struct BleService {
    pub uuid: String,
    pub primary: bool,
    pub characteristics: Vec<BleCharacteristic>,
}

impl BleService {
    fn from_services(services: BTreeSet<Service>) -> Vec<BleService> {
        services
            .into_iter()
            .map(|s| BleService {
                uuid: s.uuid.to_string(),
                primary: s.primary,
                characteristics: s
                    .characteristics
                    .into_iter()
                    .map(|c| BleCharacteristic {
                        characteristic: RustOpaque::new(c),
                    })
                    .collect(),
            })
            .collect()
    }
}

/// A Bluetooth characteristic. Characteristics are the main way you will interact with other
/// bluetooth devices. Characteristics are identified by a UUID which may be standardized
/// (like 0x2803, which identifies a characteristic for reading heart rate measurements) but more
/// often are specific to a particular device. The standard set of characteristics can be found
/// [here](https://www.bluetooth.com/specifications/gatt/characteristics).
///
/// A characteristic may be interacted with in various ways depending on its properties. You may be
/// able to write to it, read from it, set its notify or indicate status, or send a command to it.
pub struct BleCharacteristic {
    pub characteristic: RustOpaque<Characteristic>,
}

// pub struct Characteristic {
//     pub uuid: String,
//     pub service_uuid: String,
//     pub properties: CharacteristicProperties,
//     pub descriptors: Vec<Descriptor>,
// }

/// A set of properties that indicate what operations are supported by a Characteristic.
pub struct CharacteristicProperties {
    pub broadcast: bool,
    pub read: bool,
    pub write_without_response: bool,
    pub write: bool,
    pub notify: bool,
    pub indicate: bool,
    pub authenticated_signed_writes: bool,
    pub extended_properties: bool,
}

pub struct BleDescriptor {
    pub uuid: String,
    pub service_uuid: String,
    pub characteristic_uuid: String,
}

pub fn connect(id: String) -> Result<()> {
    log(format!("Try to connect to: {id}"));
    send(Command::Connect { id })
}

pub(crate) async fn inner_connect(id: String) -> Result<()> {
    log(format!("Try to connect to: {id}"));
    let devices = DEVICES.lock().await;
    let device = devices.get(&id).ok_or(Error::UnknownPeripheral(id))?;
    device.connect().await
}

pub fn disconnect(id: String) -> Result<()> {
    send(Command::Disconnect { id })
}

pub(crate) async fn inner_disconnect(id: String) -> Result<()> {
    log(format!("Try to disconnect from: {id}"));
    let devices = DEVICES.lock().await;
    let device = devices.get(&id).ok_or(Error::UnknownPeripheral(id))?;
    device.disconnect().await
}

pub(crate) async fn inner_discover_services(
    id: String,
    sink: StreamSink<Vec<BleService>>,
) -> Result<()> {
    log(format!("Try to discover services from: {id}"));
    let devices = DEVICES.lock().await;
    let device = devices.get(&id).ok_or(Error::UnknownPeripheral(id))?;
    device.peripheral.discover_services().await?;
    sink.add(BleService::from_services(device.peripheral.services()));
    sink.close();
    Ok(())
}

pub fn discover_services(id: String, sink: StreamSink<Vec<BleService>>) -> Result<()> {
    send(Command::DiscoverService { id, sink })
}

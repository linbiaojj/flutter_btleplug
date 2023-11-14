use crate::ble::device::{BleCharacteristic, BleDescriptor, BleService, CharacteristicProperties};
use crate::ble::{self, event::BleEvent, scan::BleDevice};
use crate::logger::{self, LogEntry};
use anyhow::Result;
pub use btleplug::api::{CharPropFlags, Characteristic};
use flutter_rust_bridge::StreamSink;

pub fn init() -> Result<()> {
    ble::init()
}

/// Scan for Bluetooth Low Energy devices and send the results through the given sink.
/// In Dart/Flutter one can listen() to bleScan(). The scan is automatically stopped when the subscription is cancelled.
///
/// # Parameters
///
/// * `sink` - A stream sink that new discovered devices will be passed through.
/// * `filter` - A list of filter strings to apply to the scan result.
///
/// # Return
///
/// Returns a `Result<()>` indicating if the scan operation has successfully started.
///
/// # Dart/Flutter Example
/// ```dart
/// final scan = bleScan().await;
/// scan.listen((devices) {
/// ...
/// })
/// ```
pub fn scan(sink: StreamSink<Vec<BleDevice>>, filter: Vec<String>) -> Result<()> {
    ble::scan::scan(sink, filter)
}

pub fn events(sink: StreamSink<BleEvent>) -> Result<()> {
    ble::event::events(sink)
}

pub fn connect(id: String) -> Result<()> {
    ble::device::connect(id)
}

pub fn disconnect(id: String) -> Result<()> {
    ble::device::disconnect(id)
}

pub fn discover_services(id: String, sink: StreamSink<Vec<BleService>>) -> Result<()> {
    ble::device::discover_services(id, sink)
}

pub fn create_log_stream(s: StreamSink<LogEntry>) {
    logger::create_log_stream(s);
}

impl BleCharacteristic {
    pub fn uuid(&self) -> String {
        self.characteristic.uuid.to_string()
    }

    pub fn service_uuid(&self) -> String {
        self.characteristic.service_uuid.to_string()
    }

    pub fn properties(&self) -> CharacteristicProperties {
        let p = self.characteristic.properties;
        CharacteristicProperties {
            broadcast: p.contains(CharPropFlags::BROADCAST),
            read: p.contains(CharPropFlags::READ),
            write_without_response: p.contains(CharPropFlags::WRITE_WITHOUT_RESPONSE),
            write: p.contains(CharPropFlags::WRITE),
            notify: p.contains(CharPropFlags::NOTIFY),
            indicate: p.contains(CharPropFlags::INDICATE),
            authenticated_signed_writes: p.contains(CharPropFlags::AUTHENTICATED_SIGNED_WRITES),
            extended_properties: p.contains(CharPropFlags::EXTENDED_PROPERTIES),
        }
    }

    pub fn descriptors(&self) -> Vec<BleDescriptor> {
        self.characteristic
            .descriptors
            .iter()
            .map(|d| BleDescriptor {
                uuid: d.uuid.to_string(),
                service_uuid: d.service_uuid.to_string(),
                characteristic_uuid: d.characteristic_uuid.to_string(),
            })
            .collect()
    }
}
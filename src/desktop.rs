use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use serde::de::DeserializeOwned;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<Bluetooth<R>> {
    Ok(Bluetooth(app.clone()))
}

/// Access to the bluetooth APIs.
pub struct Bluetooth<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Bluetooth<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn get_availability(&self) -> crate::Result<bool> {
        // BLE is always available thanks to `deviceplug/btleplug`
        Ok(true)
    }

    pub async fn request_device(&self, options: RequestDeviceOptions) -> crate::Result<DeviceInfo> {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).unwrap();

        // start scanning for devices
        central.start_scan(ScanFilter::default()).await?;
        // instead of waiting, you can use central.events() to get a stream which will
        // notify you of new devices, for an example of that see examples/event_driven_discovery.rs
        sleep(Duration::from_secs(2));
        // Use btleplug to scan for devices
        Ok(DeviceInfo {
            id: "device-id".to_string(),
            services: vec!["service-id".to_string()],
        })
    }
}

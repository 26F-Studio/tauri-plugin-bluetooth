use serde::de::DeserializeOwned;
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
}

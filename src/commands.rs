use tauri::{command, AppHandle, Runtime};

use crate::bluetooth::models::RequestDeviceOptions;
use crate::models::*;
use crate::Result;
use crate::{Error, PluginExt};

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.plugin_base().ping(payload)
}

#[command]
pub(crate) async fn gatt_connect<R: Runtime>(app: AppHandle<R>, device_id: String) -> Result<()> {
    app.bluetooth_manager().gatt_connect(device_id).await
}

#[command]
pub(crate) async fn gatt_connected<R: Runtime>(app: AppHandle<R>, device_id: String) -> Result<bool> {
    app.bluetooth_manager().gatt_connected(device_id).await
}

#[command]
pub(crate) async fn get_availability<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.bluetooth_manager().get_availability().await
}

#[command]
pub(crate) async fn request_device<R: Runtime>(
    app: AppHandle<R>,
    options: RequestDeviceOptions,
) -> Result<DeviceInfo> {
    if !options.accept_all_devices.unwrap_or(false) && options.filters.is_none() {
        return Err(Error::InvalidRequestDeviceOptions);
    }
    app.bluetooth_manager().request_device(options).await
}

pub fn collect_handlers<R: Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool {
    tauri::generate_handler![ping, gatt_connect, gatt_connected, get_availability, request_device]
}

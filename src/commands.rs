use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::BluetoothExt;
use crate::Result;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.bluetooth().ping(payload)
}

#[command]
pub(crate) async fn get_availability<R: Runtime>(app: AppHandle<R>) -> Result<bool> {
    app.bluetooth().get_availability()
}

#[command]
pub(crate) async fn request_device<R: Runtime>(
    app: AppHandle<R>,
    options: RequestDeviceOptions,
) -> Result<DeviceInfo> {
    app.bluetooth().request_device(options).await
}

pub fn collect_handlers<R: Runtime>() -> impl Fn(tauri::ipc::Invoke<R>) -> bool {
    tauri::generate_handler![
        ping,
        get_availability
    ]
}
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

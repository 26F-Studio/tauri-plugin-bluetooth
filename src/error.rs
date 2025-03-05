use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    BluetoothLE(#[from] btleplug::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Gatt connect failure")]
    GattConnectFailure,
    #[error("Invalid request device options")]
    InvalidRequestDeviceOptions,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No available bluetooth adapter")]
    NoAdapter,
    #[error("Scan start failure")]
    ScanStartFailure,
    #[error("Scan stop failure")]
    ScanStopFailure,
    #[error(transparent)]
    TimeoutExpired(#[from] tokio::time::error::Elapsed),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

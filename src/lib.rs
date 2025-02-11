mod commands;
#[cfg(desktop)]
mod desktop;
mod error;
#[cfg(mobile)]
mod mobile;
mod models;

#[cfg(desktop)]
use desktop::Bluetooth;
pub use error::{Error, Result};
#[cfg(mobile)]
use mobile::Bluetooth;
pub use models::*;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the bluetooth APIs.
pub trait BluetoothExt<R: Runtime> {
    fn bluetooth(&self) -> &Bluetooth<R>;
}

impl<R: Runtime, T: Manager<R>> crate::BluetoothExt<R> for T {
    fn bluetooth(&self) -> &Bluetooth<R> {
        self.state::<Bluetooth<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    use commands::*;
    Builder::new("bluetooth")
        .invoke_handler(tauri::generate_handler![ping])
        .invoke_handler(tauri::generate_handler![get_availability])
        .setup(|app, api| {
            #[cfg(mobile)]
            let bluetooth = mobile::init(app, api)?;
            #[cfg(desktop)]
            let bluetooth = desktop::init(app, api)?;
            app.manage(bluetooth);
            Ok(())
        })
        .build()
}

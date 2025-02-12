mod bluetooth;
mod commands;
#[cfg(desktop)]
mod desktop;
mod error;
#[cfg(mobile)]
mod mobile;
mod models;

use crate::bluetooth::BluetoothManager;
#[cfg(desktop)]
use desktop::PluginBase;
use error::{Error, Result};
#[cfg(mobile)]
use mobile::PluginBase;
use models::*;
use tauri::plugin::PluginApi;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub trait PluginExt<R: Runtime> {
    fn bluetooth_manager(&self) -> &BluetoothManager;
    fn plugin_base(&self) -> &PluginBase<R>;
}

impl<R: Runtime, T: Manager<R>> PluginExt<R> for T {
    fn bluetooth_manager(&self) -> &BluetoothManager {
        self.state::<BluetoothManager>().inner()
    }
    fn plugin_base(&self) -> &PluginBase<R> {
        self.state::<PluginBase<R>>().inner()
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("bluetooth")
        .invoke_handler(commands::collect_handlers())
        .setup(|app, api| _setup(app, api))
        .build()
}

fn _setup<R: Runtime>(
    app: &tauri::AppHandle<R>,
    api: PluginApi<R, ()>,
) -> core::result::Result<(), Box<dyn core::error::Error>> {
    tauri::async_runtime::block_on(async move {
        #[cfg(mobile)]
        let plugin_base = mobile::init(app, api)?;
        #[cfg(desktop)]
        let plugin_base = desktop::init(app, api)?;
        app.manage(plugin_base);

        let bluetooth_manager = bluetooth::init().await?;
        app.manage(bluetooth_manager);
        Ok(())
    })
}

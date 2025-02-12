pub mod models;
mod utils;

use crate::{DeviceInfo, Error, RequestDeviceOptions, Result};
use btleplug::api::{
    Central, CentralEvent, Manager as _, Peripheral as _, PeripheralProperties, ScanFilter,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::RwLock;
use tokio::time;
use utils::match_services;
use uuid::Uuid;

pub async fn init() -> Result<BluetoothManager> {
    BluetoothManager::new().await
}

pub struct BluetoothManager {
    _adapter: Arc<RwLock<Option<Adapter>>>,
    _devices: Arc<RwLock<HashMap<String, Peripheral>>>,
    _manager: Manager,
}

impl BluetoothManager {
    pub async fn new() -> Result<Self> {
        Self::with_adapter_index(0).await
    }

    pub async fn with_adapter_index(adapter_index: usize) -> Result<Self> {
        let manager = Manager::new().await?;
        Ok(Self {
            _adapter: Arc::new(RwLock::new(
                manager.adapters().await?.into_iter().nth(adapter_index),
            )),
            _devices: Arc::new(RwLock::new(HashMap::new())),
            _manager: manager,
        })
    }

    pub async fn get_availability(&self) -> Result<bool> {
        Ok(!self._manager.adapters().await?.is_empty())
    }

    pub async fn request_device(&self, options: RequestDeviceOptions) -> Result<DeviceInfo> {
        let adapter_with_lock = self._adapter.read().await;
        let adapter = match adapter_with_lock.as_ref() {
            Some(adapter) => adapter,
            None => return Err(Error::NoAdapter),
        };

        let mut events = adapter.events().await?;
        adapter.start_scan(ScanFilter::default()).await?;

        let (device_id, properties) = time::timeout(Duration::from_secs(30), async {
            while let Some(event) = events.next().await {
                if let CentralEvent::DeviceDiscovered(id) = event {
                    if let Ok(peripheral) = adapter.peripheral(&id).await {
                        if let Some(properties) = peripheral.properties().await? {
                            if _match_options(&properties, &options).await {
                                let device_id = Uuid::new_v4().hyphenated().to_string();
                                self._devices
                                    .write()
                                    .await
                                    .insert(device_id.clone(), peripheral);
                                return Ok((device_id, properties));
                            }
                        }
                    }
                }
            }
            Err(btleplug::Error::DeviceNotFound)
        })
        .await??;

        adapter.stop_scan().await?;

        Ok(DeviceInfo {
            id: device_id,
            services: properties
                .services
                .iter()
                .map(|uuid| uuid.hyphenated().to_string())
                .collect(),
        })
    }

    async fn _update_adapter(&self, adapter_index: usize) -> Result<bool> {
        let mut result = true;
        if self._adapter.read().await.is_none() {
            let adapter_opt = self
                ._manager
                .adapters()
                .await?
                .into_iter()
                .nth(adapter_index);
            result = adapter_opt.is_some();
            *self._adapter.write().await = adapter_opt;
        }
        Ok(result)
    }
}

async fn _match_options(properties: &PeripheralProperties, options: &RequestDeviceOptions) -> bool {
    if options.accept_all_devices.unwrap_or(false) {
        return true;
    }

    let local_name = properties.local_name.as_deref().unwrap_or_default();
    if let Some(ref filters) = options.filters {
        return filters.iter().any(|filter| {
            filter.services.as_ref().map_or(true, |filter_services| {
                match_services(&properties.services, filter_services)
            }) && filter
                .name
                .as_ref()
                .map_or(true, |filter_name| local_name == filter_name)
                && filter
                    .name_prefix
                    .as_ref()
                    .map_or(true, |filter_name_prefix| {
                        local_name.starts_with(filter_name_prefix)
                    })
        });
    }
    false
}

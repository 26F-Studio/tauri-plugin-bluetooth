pub mod models;
mod utils;

use crate::bluetooth::models::RequestDeviceOptions;
use crate::{DeviceInfo, Error, Result};
use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use futures::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::RwLock;
use tokio::time;
use uuid::Uuid;

// 使用更具表达性的类型别名
type AdapterLock = Arc<RwLock<Option<Adapter>>>;
type DeviceMap = Arc<RwLock<HashMap<String, Peripheral>>>;

pub async fn init() -> Result<BluetoothManager> {
    BluetoothManager::new().await
}

pub struct BluetoothManager {
    adapter: AdapterLock,
    devices: DeviceMap,
    manager: Manager,
}

impl BluetoothManager {
    pub async fn new() -> Result<Self> {
        Self::with_adapter_index(0).await
    }

    pub async fn with_adapter_index(adapter_index: usize) -> Result<Self> {
        let manager = Manager::new().await?;
        Ok(Self {
            adapter: Arc::new(RwLock::new(
                manager.adapters().await?.into_iter().nth(adapter_index),
            )),
            devices: Arc::new(RwLock::new(HashMap::new())),
            manager,
        })
    }

    pub async fn get_availability(&self) -> Result<bool> {
        Ok(!self.manager.adapters().await?.is_empty())
    }

    pub async fn request_device(&self, options: RequestDeviceOptions) -> Result<DeviceInfo> {
        let adapter_with_lock = self.adapter.read().await;
        let adapter = match adapter_with_lock.as_ref() {
            Some(adapter) => adapter,
            None => return Err(Error::NoAdapter),
        };

        let mut events = adapter.events().await?;
        adapter
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| {
                log::error!("Failed to start scan: {}", e);
                Error::ScanStartFailure
            })?;

        let (device_id, properties) = time::timeout(Duration::from_secs(30), async {
            while let Some(event) = events.next().await {
                if let CentralEvent::DeviceDiscovered(id) = event {
                    if let Ok(peripheral) = adapter.peripheral(&id).await {
                        if let Some(properties) = peripheral.properties().await? {
                            if utils::match_options(&properties, &options) {
                                let device_id = Uuid::new_v4().hyphenated().to_string();
                                self._cache_peripheral(device_id.clone(), peripheral).await;
                                return Ok((device_id, properties));
                            }
                        }
                    }
                }
            }
            Err(btleplug::Error::DeviceNotFound)
        })
        .await??;

        adapter.stop_scan().await.map_err(|e| {
            log::error!("Failed to stop scan: {}", e);
            Error::ScanStopFailure
        })?;

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
        if self.adapter.read().await.is_none() {
            let adapter_opt = self
                .manager
                .adapters()
                .await?
                .into_iter()
                .nth(adapter_index);
            result = adapter_opt.is_some();
            *self.adapter.write().await = adapter_opt;
        }
        Ok(result)
    }

    async fn _cache_peripheral(&self, id: String, peripheral: Peripheral) {
        self.devices.write().await.entry(id).or_insert(peripheral);
    }
}

impl Drop for BluetoothManager {
    fn drop(&mut self) {
        if let Some(adapter) = self.adapter.blocking_read().as_ref() {
            tauri::async_runtime::block_on(async {
                adapter.stop_scan().await.expect("Failed to stop scan");
            })
        }
    }
}

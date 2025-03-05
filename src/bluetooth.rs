pub mod models;
mod utils;

use crate::bluetooth::models::RequestDeviceOptions;
use crate::{DeviceInfo, Error, Result};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use btleplug::api::{Central, Manager as _, Peripheral as _, ScanFilter};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::async_runtime::RwLock;
use uuid::Uuid;

// 使用更具表达性的类型别名
type DeviceMap = Arc<RwLock<HashMap<String, Peripheral>>>;

pub async fn init() -> Result<BluetoothManager> {
    BluetoothManager::new().await
}

pub struct BluetoothManager {
    adapter_index: usize,
    address_to_id_map: Arc<RwLock<HashMap<String, String>>>,
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
            adapter_index,
            address_to_id_map: Arc::new(RwLock::new(HashMap::new())),
            devices: Arc::new(RwLock::new(HashMap::new())),
            manager,
        })
    }

    pub async fn gatt_connect(&self, device_id: String) -> Result<()> {
        let devices = self.devices.read().await;
        let device = match devices.get(&device_id) {
            Some(peripheral) => peripheral,
            None => return Err(Error::DeviceNotFound),
        };

        device.connect().await.map_err(|e| {
            log::error!("Failed to connect to device: {}", e);
            Error::GattConnectFailure
        })?;

        Ok(())
    }

    pub async fn gatt_connected(&self, device_id: String) -> Result<bool> {
        let devices = self.devices.read().await;
        let device = match devices.get(&device_id) {
            Some(peripheral) => peripheral,
            None => return Err(Error::DeviceNotFound),
        };

        Ok(device.is_connected().await?)
    }

    pub async fn get_availability(&self) -> Result<bool> {
        Ok(!self.manager.adapters().await?.is_empty())
    }

    pub async fn request_device(&self, options: RequestDeviceOptions) -> Result<DeviceInfo> {
        let adapter = match self._get_adapter().await? {
            Some(adapter) => adapter,
            None => return Err(Error::NoAdapter),
        };

        adapter
            .start_scan(ScanFilter::default())
            .await
            .map_err(|e| {
                log::error!("Failed to start scan: {}", e);
                Error::ScanStartFailure
            })?;
        tokio::time::sleep(Duration::from_millis(options.timeout.unwrap_or(5000))).await;
        adapter.stop_scan().await.map_err(|e| {
            log::error!("Failed to stop scan: {}", e);
            Error::ScanStopFailure
        })?;

        for peripheral in adapter.peripherals().await?.iter() {
            if let Some(properties) = peripheral.properties().await? {
                if utils::match_options(&properties, &options) {
                    log::info!("Found {:#?}", properties);
                    let device_id = self._cache_peripheral_and_get_id(peripheral).await;
                    return Ok(DeviceInfo {
                        id: device_id,
                        services: properties
                            .services
                            .iter()
                            .map(|uuid| uuid.hyphenated().to_string())
                            .collect(),
                    });
                }
            }
        }
        Err(btleplug::Error::DeviceNotFound)?

        // let mut events = adapter.events().await?;
        // let (device_id, properties) = time::timeout(Duration::from_secs(10), async {
        //     while let Some(event) = events.next().await {
        //         if let CentralEvent::DeviceDiscovered(id) = event {
        //             if let Ok(peripheral) = adapter.peripheral(&id).await {
        //                 if let Some(properties) = peripheral.properties().await? {
        //                     if utils::match_options(&properties, &options) {
        //                         log::info!("Found {:#?}: {:#?}", id, properties);
        //                         let device_id = self._cache_peripheral_and_get_id(peripheral).await;
        //                         return Ok((device_id, properties));
        //                     }
        //                 }
        //             }
        //         }
        //     }
        //     Err(btleplug::Error::DeviceNotFound)
        // })
        // .await??;
    }

    async fn _cache_peripheral_and_get_id(&self, peripheral: &Peripheral) -> String {
        if let Some(device_id) = self
            .address_to_id_map
            .read()
            .await
            .get(&peripheral.address().to_string())
        {
            return device_id.clone();
        }

        // Chromium uses a base64 encoded UUID as the device ID
        // https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/modules/bluetooth/bluetooth_device.h;l=99
        let device_id = BASE64_STANDARD.encode(Uuid::new_v4().hyphenated().to_string());

        self.address_to_id_map
            .write()
            .await
            .insert(peripheral.address().to_string(), device_id.clone());
        self.devices
            .write()
            .await
            .insert(device_id.clone(), peripheral.clone());
        device_id
    }

    async fn _get_adapter(&self) -> Result<Option<Adapter>> {
        Ok(self
            .manager
            .adapters()
            .await?
            .into_iter()
            .nth(self.adapter_index))
    }
}

impl Drop for BluetoothManager {
    fn drop(&mut self) {
        tauri::async_runtime::block_on(async {
            if let Ok(Some(adapter)) = self._get_adapter().await {
                adapter.stop_scan().await.expect("Failed to stop scan");
            }
        })
    }
}

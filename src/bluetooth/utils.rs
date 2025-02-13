use crate::bluetooth::models::{BluetoothLEScanFilter, BluetoothServiceUUID, RequestDeviceOptions};
use btleplug::api::{bleuuid, PeripheralProperties};
use uuid::Uuid;

pub fn match_services(target_services: &[Uuid], filter_services: &[BluetoothServiceUUID]) -> bool {
    filter_services
        .iter()
        .all(|filter_service| match filter_service {
            BluetoothServiceUUID::Number(uuid) => {
                target_services.contains(&bleuuid::uuid_from_u32(*uuid))
            }
            BluetoothServiceUUID::String(uuid) => {
                target_services.contains(&Uuid::parse_str(uuid).unwrap())
            }
        })
}

fn is_services_match(properties: &PeripheralProperties, filter: &BluetoothLEScanFilter) -> bool {
    match &filter.services {
        Some(services) => match_services(&properties.services, services),
        None => true,
    }
}

fn is_name_match(properties: &PeripheralProperties, filter: &BluetoothLEScanFilter) -> bool {
    match (&filter.name, &properties.local_name) {
        (Some(name), Some(local_name)) => local_name == name,
        (None, _) => true,
        _ => false,
    }
}

fn is_prefix_match(properties: &PeripheralProperties, filter: &BluetoothLEScanFilter) -> bool {
    match (&filter.name_prefix, &properties.local_name) {
        (Some(prefix), Some(local_name)) => local_name.starts_with(prefix),
        (None, _) => true,
        _ => false,
    }
}

pub fn match_options(properties: &PeripheralProperties, options: &RequestDeviceOptions) -> bool {
    options.accept_all_devices.unwrap_or(false)
        || options.filters.as_ref().is_some_and(|filters| {
            filters.iter().any(|filter| {
                is_services_match(properties, filter)
                    && is_name_match(properties, filter)
                    && is_prefix_match(properties, filter)
            })
        })
}

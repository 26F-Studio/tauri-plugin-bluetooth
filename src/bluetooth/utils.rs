use crate::bluetooth::models::{BluetoothServiceUUID, RequestDeviceOptions};
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

pub fn match_options(properties: &PeripheralProperties, options: &RequestDeviceOptions) -> bool {
    options.accept_all_devices.unwrap_or(false)
        || options.filters.as_ref().is_some_and(|filters| {
            filters.iter().any(|filter| {
                filter.services.as_ref().map_or(true, |services| {
                    match_services(&properties.services, services)
                }) && filter
                    .name
                    .as_ref()
                    .map_or(true, |name| properties.local_name.as_deref() == Some(name))
                    && filter.name_prefix.as_ref().map_or(true, |prefix| {
                        properties
                            .local_name
                            .as_deref()
                            .is_some_and(|n| n.starts_with(prefix))
                    })
            })
        })
}

use crate::bluetooth::models::BluetoothServiceUUID;
use btleplug::api::bleuuid;
use uuid::Uuid;

pub fn match_services(
    target_services: &Vec<Uuid>,
    filter_services: &Vec<BluetoothServiceUUID>,
) -> bool {
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

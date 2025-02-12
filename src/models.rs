use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
    pub value: Option<String>,
}

/**
Represents a Bluetooth service UUID which can be either a number or a string.

Typescript equivalent:

```typescript
type BluetoothServiceUUID = number | string;
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum BluetoothServiceUUID {
    Number(u32),
    String(String),
}

/**
Represents a Bluetooth manufacturer data filter.

Typescript equivalent:

```typescript
interface BluetoothDataFilter {
    readonly dataPrefix?: BufferSource | undefined;
    readonly mask?: BufferSource | undefined;
}
```
*/
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothDataFilter {
    pub data_prefix: Option<Vec<u8>>,
    pub mask: Option<Vec<u8>>,
}

/**
Represents a Bluetooth service data filter.

Typescript equivalent:

```typescript
interface BluetoothManufacturerDataFilter extends BluetoothDataFilter {
    companyIdentifier: number;
}
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothManufacturerDataFilter {
    pub data_prefix: Option<Vec<u8>>,
    pub mask: Option<Vec<u8>>,
    pub company_identifier: u16,
}

/**
Represents a Bluetooth service data filter.

Typescript equivalent:

```typescript
interface BluetoothServiceDataFilter extends BluetoothDataFilter {
    service: BluetoothServiceUUID;
}
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothServiceDataFilter {
    pub data_prefix: Option<Vec<u8>>,
    pub mask: Option<Vec<u8>>,
    pub service: BluetoothServiceUUID,
}

/**
Represents a filter for Bluetooth LE scans.

Typescript equivalent:

```typescript
interface BluetoothLEScanFilter {
  readonly name?: string | undefined;
  readonly namePrefix?: string | undefined;
  readonly services?: BluetoothServiceUUID[] | undefined;
  readonly manufacturerData?: BluetoothManufacturerDataFilter[] | undefined;
  readonly serviceData?: BluetoothServiceDataFilter[] | undefined;
}
```
*/
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothLEScanFilter {
    pub name: Option<String>,
    pub name_prefix: Option<String>,
    pub services: Option<Vec<BluetoothServiceUUID>>,
    pub manufacturer_data: Option<Vec<BluetoothManufacturerDataFilter>>,
    pub service_data: Option<Vec<BluetoothServiceDataFilter>>,
}

/**
Represents options for requesting a Bluetooth device.

Typescript equivalent:

```typescript
type RequestDeviceOptions = {
  filters: BluetoothLEScanFilter[];
  optionalServices?: BluetoothServiceUUID[] | undefined;
  optionalManufacturerData?: number[] | undefined;
} | {
  acceptAllDevices: boolean;
  optionalServices?: BluetoothServiceUUID[] | undefined;
  optionalManufacturerData?: number[] | undefined;
};
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RequestDeviceOptions {
    Filters {
        filters: Vec<BluetoothLEScanFilter>,
        optional_services: Option<Vec<BluetoothServiceUUID>>,
        optional_manufacturer_data: Option<Vec<u8>>,
    },
    AcceptAllDevices {
        accept_all_devices: bool,
        optional_services: Option<Vec<BluetoothServiceUUID>>,
        optional_manufacturer_data: Option<Vec<u8>>,
    },
}

/**
Represents the info of a bluetooth device.
Typescript equivalent:

```typescript
interface DeviceInfo {
  id: string
  services: string[]
}
```
*/
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub id: String,
    pub services: Vec<String>,
}
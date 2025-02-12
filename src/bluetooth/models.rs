use serde::{Deserialize, Serialize};

/**
Represents a Bluetooth service UUID which can be either a number or a string.

Typescript reference:

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
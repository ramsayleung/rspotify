use crate::model::DeviceType;
use serde::{Deserialize, Serialize};

/// All objects related to device
/// [get a users available devices](https://developer.spotify.com/web-api/get-a-users-available-devices/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: DeviceType,
    pub volume_percent: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_devices() {
        let json_str = r#"{
"devices" : [ {
    "id" : "5fbb3ba6aa454b5534c4ba43a8c7e8e45a63ad0e",
    "is_active" : false,
    "is_private_session": true,
    "is_restricted" : false,
    "name" : "My fridge",
    "type" : "Computer",
    "volume_percent" : 100
  } ]}
"#;
        let payload: DevicePayload = serde_json::from_str(&json_str).unwrap();
        assert_eq!(payload.devices[0]._type, DeviceType::Computer)
    }
}

use crate::model::DeviceType;
use serde::{Deserialize, Serialize};

/// Device object
///
/// [Reference](https://developer.spotify.com/web-api/get-a-users-available-devices/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: DeviceType,
    pub volume_percent: Option<u32>,
}

/// Device payload object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/)
// TODO: Reduce this wrapper object to `Vec<Device>`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}


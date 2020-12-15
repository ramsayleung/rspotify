//! Offset object
use crate::model::{from_option_duration_ms, to_option_duration_ms};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Offset object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/start-a-users-playback/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Offset {
    #[serde(default)]
    #[serde(
        deserialize_with = "from_option_duration_ms",
        serialize_with = "to_option_duration_ms"
    )]
    pub position: Option<Duration>,
    pub uri: Option<String>,
}

pub fn for_position(position: u64) -> Option<Offset> {
    Some(Offset {
        position: Some(Duration::from_millis(position)),
        uri: None,
    })
}

pub fn for_uri(uri: String) -> Option<Offset> {
    Some(Offset {
        position: None,
        uri: Some(uri),
    })
}

//! Offset object
use crate::model::option_duration_ms;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Offset object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-start-a-users-playback)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Offset {
    #[serde(default)]
    #[serde(with = "option_duration_ms")]
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

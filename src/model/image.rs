//! Image object
use serde::{Deserialize, Serialize};

/// Image object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-imageobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}

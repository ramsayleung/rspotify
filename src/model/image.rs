//! Image object
use serde::{Deserialize, Serialize};

/// [image object](https://developer.spotify.com/web-api/object-model/#image-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}

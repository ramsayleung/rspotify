//! Image object
use serde::{Deserialize, Serialize};

/// [image object](https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}

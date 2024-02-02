//! Image object

pub use crate::data_type_patcher::as_some_u32;

use serde::{Deserialize, Serialize};

/// Image object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Image {
    // TODO: remove this statement after Spotify fix the [issue](https://github.com/ramsayleung/rspotify/issues/452)
    #[serde(deserialize_with = "as_some_u32")]
    pub height: Option<u32>,
    pub url: String,
    // TODO: remove this statement after Spotify fix the [issue](https://github.com/ramsayleung/rspotify/issues/452)
    #[serde(deserialize_with = "as_some_u32")]
    pub width: Option<u32>,
}

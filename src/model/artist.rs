//! All objects related to artist defined by Spotify API
use serde::{Deserialize, Serialize};

use super::image::Image;
use super::page::CursorBasedPage;
use crate::model::{Followers, Type};
use std::collections::HashMap;
/// Simplified Artist Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: Option<String>,
}

/// Full Artist Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullArtist {
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Full Artists vector wrapped by cursor-based-page object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/follow/get-followed/)
// TODO: Reduce this wrapper object to `CursorBasedPage<FullArtist>`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CursorPageFullArtists {
    pub artists: CursorBasedPage<FullArtist>,
}

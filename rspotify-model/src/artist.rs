//! All objects related to artist defined by Spotify API

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{ArtistId, CursorBasedPage, Followers, Image};

/// Simplified Artist Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedartistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<ArtistId>,
    pub name: String,
}

/// Full Artist Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-artistobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullArtist {
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: ArtistId,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
}

/// Intermediate full artist object wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-artists)
#[derive(Deserialize)]
pub struct FullArtists {
    pub artists: Vec<FullArtist>,
}

/// Intermediate full Artists vector wrapped by cursor-based-page object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-followed)
#[derive(Deserialize)]
pub struct CursorPageFullArtists {
    pub artists: CursorBasedPage<FullArtist>,
}

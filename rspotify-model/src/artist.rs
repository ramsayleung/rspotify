//! All objects related to artist defined by Spotify API

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{data_type_patcher::as_u32, ArtistId, CursorBasedPage, Followers, Image};

/// Simplified Artist Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimplifiedArtist {
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<ArtistId<'static>>,
    pub name: String,
}

/// Full Artist Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullArtist {
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub genres: Vec<String>,
    pub href: String,
    pub id: ArtistId<'static>,
    pub images: Vec<Image>,
    pub name: String,
    // TODO: remove this statement after Spotify fix the [issue](https://github.com/ramsayleung/rspotify/issues/452)
    #[serde(deserialize_with = "as_u32")]
    pub popularity: u32,
}

/// Intermediate full artist object wrapped by `Vec`
#[derive(Deserialize)]
pub struct FullArtists {
    pub artists: Vec<FullArtist>,
}

/// Intermediate full Artists vector wrapped by cursor-based-page object
#[derive(Deserialize)]
pub struct CursorPageFullArtists {
    pub artists: CursorBasedPage<FullArtist>,
}

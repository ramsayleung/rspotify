//! All objects related to album defined by Spotify API

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    AlbumId, AlbumType, Copyright, DatePrecision, Image, Page, RestrictionReason, SimplifiedArtist,
    SimplifiedTrack,
};

/// Simplified Album Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-simplifiedalbumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<String>,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<AlbumId>,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date_precision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
}

/// Full Album Object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: AlbumType,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<Copyright>,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: AlbumId,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub tracks: Page<SimplifiedTrack>,
}

/// Full Albums wrapped by Vec object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-albums)
#[derive(Deserialize)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

/// Simplified Albums wrapped by Page object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-new-releases)
#[derive(Deserialize)]
pub struct PageSimpliedAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

/// Saved Album object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-savedalbumobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SavedAlbum {
    pub added_at: DateTime<Utc>,
    pub album: FullAlbum,
}

/// Album restriction object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-albumrestrictionobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

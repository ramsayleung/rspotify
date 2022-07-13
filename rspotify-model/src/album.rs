//! All objects related to album defined by Spotify API

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    AlbumId, AlbumType, Copyright, DatePrecision, Image, Page, RestrictionReason, SimplifiedArtist,
    SimplifiedTrack,
};

/// Simplified Album Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SimplifiedAlbum {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_group: Option<String>,
    pub album_type: Option<String>,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<AlbumId<'static>>,
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
    pub album_type: AlbumType,
    pub available_markets: Option<Vec<String>>,
    pub copyrights: Vec<Copyright>,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: AlbumId<'static>,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: DatePrecision,
    pub tracks: Page<SimplifiedTrack>,
    /// Not documented in official Spotify docs, however most albums do contain this field
    pub label: Option<String>,
}

/// Intermediate full Albums wrapped by Vec object
#[derive(Deserialize)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

/// Intermediate simplified Albums wrapped by Page object
#[derive(Deserialize)]
pub struct PageSimplifiedAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

/// Saved Album object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SavedAlbum {
    pub added_at: DateTime<Utc>,
    pub album: FullAlbum,
}

/// Album restriction object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

//! All kinds of tracks object
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::album::SimplifiedAlbum;
use super::artist::SimplifiedArtist;
use super::Restriction;
use crate::model::Type;

/// Full track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_playable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_from: Option<TrackLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub popularity: u32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Track link object
///
/// [Reference] https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackLink {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Full track wrapped by `Vec`
///
/// [Reference](https://developer.spotify.com/web-api/get-several-tracks/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub(in crate) struct FullTracks {
    pub tracks: Vec<FullTrack>,
}

/// Simplified track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<String>,
    pub is_local: bool,
    // These three fields are only present when track relinking is applied.
    //-------------------//
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub restrictions: Option<Restriction>,
    //-------------------//
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Saved track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#saved-track-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: FullTrack,
}

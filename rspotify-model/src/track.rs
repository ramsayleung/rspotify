//! All kinds of tracks object

use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    custom_serde::duration_ms, PlayableId, Restriction, SimplifiedAlbum, SimplifiedArtist, TrackId,
};

/// Full track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    /// Note that a track may not have an ID/URI if it's local
    pub id: Option<TrackId<'static>>,
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
}

/// Track link object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrackLink {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: TrackId<'static>,
}

/// Intermediate full track wrapped by `Vec`
#[derive(Deserialize)]
pub struct FullTracks {
    pub tracks: Vec<FullTrack>,
}

/// Simplified track object.
///
/// `is_playable`, `linked_from` and `restrictions` will only be present when
/// relinking is applied.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedTrack {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<SimplifiedAlbum>,
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    #[serde(with = "duration_ms", rename = "duration_ms")]
    pub duration: Duration,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    #[serde(default)]
    pub href: Option<String>,
    pub id: Option<TrackId<'static>>,
    pub is_local: bool,
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub restrictions: Option<Restriction>,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
}

/// Saved track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: FullTrack,
}

/// Track id with specific positions track in a playlist
///
/// This is a short-lived struct for endpoint parameters, so it uses
/// `PlayableId<'a>` instead of `PlayableId<'static>` to avoid the unnecessary
/// allocation. Same goes for the positions slice instead of vector.
pub struct ItemPositions<'a> {
    pub id: PlayableId<'a>,
    pub positions: &'a [u32],
}

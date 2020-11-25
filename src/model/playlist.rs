//! All kinds of playlists objects
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::image::Image;
use super::page::Page;
use super::track::FullTrack;
use super::user::PublicUser;
use crate::model::{Followers, Type};

/// Playlist result object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/playlists/add-tracks-to-playlist/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistResult {
    pub snapshot_id: String,
}

/// Simplified playlist object
///
///[Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#playlist-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: HashMap<String, Value>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Full playlist object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#playlist-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Page<PlaylistItem>,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

/// Playlist track object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#playlist-track-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlaylistItem {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    pub track: Option<FullTrack>,
}
/// Featured playlists object
/// [Reference](https://developer.spotify.com/web-api/get-list-featured-playlists/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct FeaturedPlaylists {
    pub message: String,
    pub playlists: Page<SimplifiedPlaylist>,
}

/// Category playlists object wrapped by `Page`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/browse/get-categorys-playlists/)
#[derive(Deserialize)]
pub(in crate) struct CategoryPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}

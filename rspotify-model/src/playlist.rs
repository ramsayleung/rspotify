//! All kinds of playlists objects

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{Followers, Image, Page, PlayableItem, PlaylistId, PublicUser};

/// Playlist result object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistResult {
    pub snapshot_id: String,
}

/// Playlist Track Reference Object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistTracksRef {
    pub href: String,
    pub total: u32,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

/// Simplified playlist object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: PlaylistId<'static>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: PlaylistTracksRef,
}

/// Full playlist object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub href: String,
    pub id: PlaylistId<'static>,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Page<PlaylistItem>,
}

/// Playlist track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PlaylistItem {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    pub track: Option<PlayableItem>,
}

/// Featured playlists object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FeaturedPlaylists {
    pub message: String,
    pub playlists: Page<SimplifiedPlaylist>,
}

/// Intermediate category playlists object wrapped by `Page`
#[derive(Deserialize)]
pub struct CategoryPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}

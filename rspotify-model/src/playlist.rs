#![allow(unreachable_patterns)]
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
#[allow(unreachable_patterns)]
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
    /// Note: This field is kept for compatibility before, during and after
    /// Spotify's February 2026 API migration. It is synced with `items`
    /// during deserialization.
    #[deprecated(
        since = "0.16.0",
        note = "Renamed to `items` by Spotify. Use `items` instead. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    #[serde(alias = "items")]
    #[allow(unreachable_patterns)]
    pub tracks: PlaylistTracksRef,
    #[serde(alias = "tracks")]
    #[allow(unreachable_patterns)]
    pub items: PlaylistTracksRef,
}

/// Full playlist object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[allow(unreachable_patterns)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Followers,
    pub href: String,
    pub id: PlaylistId<'static>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    /// Note: This field is kept for compatibility before, during and after
    /// Spotify's February 2026 API migration. It is synced with `items`
    /// during deserialization.
    #[deprecated(
        since = "0.16.0",
        note = "Renamed to `items` by Spotify. Use `items` instead. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    #[serde(alias = "items")]
    #[allow(unreachable_patterns)]
    pub tracks: Page<PlaylistItem>,
    #[serde(alias = "tracks")]
    #[allow(unreachable_patterns)]
    pub items: Page<PlaylistItem>,
}

/// Playlist track object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[allow(unreachable_patterns)]
pub struct PlaylistItem {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    /// Note: This field is kept for compatibility before, during and after
    /// Spotify's February 2026 API migration. It is synced with `item`
    /// during deserialization.
    #[deprecated(
        since = "0.16.0",
        note = "Renamed to `item` by Spotify. Use `item` instead. See https://github.com/ramsayleung/rspotify/issues/550"
    )]
    #[serde(alias = "item")]
    #[allow(unreachable_patterns)]
    pub track: Option<PlayableItem>,
    #[serde(alias = "track")]
    #[allow(unreachable_patterns)]
    pub item: Option<PlayableItem>,
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

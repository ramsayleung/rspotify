//! All kinds of playlists objects

use crate::{Followers, Image, Page, PlayableItem, PlaylistId, PublicUser};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

#[derive(Deserialize)]
struct SimplifiedPlaylistShadow {
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
    #[serde(default)]
    pub items: Option<PlaylistTracksRef>,
    #[serde(default)]
    pub tracks: Option<PlaylistTracksRef>,
}

#[allow(deprecated)]
impl From<SimplifiedPlaylistShadow> for SimplifiedPlaylist {
    fn from(shadow: SimplifiedPlaylistShadow) -> Self {
        let items = shadow
            .items
            .or(shadow.tracks)
            .expect("missing items/tracks");
        Self {
            collaborative: shadow.collaborative,
            external_urls: shadow.external_urls,
            href: shadow.href,
            id: shadow.id,
            images: shadow.images,
            name: shadow.name,
            owner: shadow.owner,
            public: shadow.public,
            snapshot_id: shadow.snapshot_id,
            tracks: items.clone(),
            items,
        }
    }
}

/// Simplified playlist object
///
/// Note: The `tracks` field was renamed to `items` by Spotify. This struct
/// uses an internal "shadow" struct for deserialization to ensure that both
/// fields are correctly populated regardless of whether Spotify sends the old
/// or new key. This maintains compatibility before, during and after the
/// February 2026 API migration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(from = "SimplifiedPlaylistShadow")]
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
    pub tracks: PlaylistTracksRef,
    pub items: PlaylistTracksRef,
}

#[derive(Deserialize)]
struct FullPlaylistShadow {
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
    #[serde(alias = "tracks")]
    pub items: Page<PlaylistItem>,
}

#[allow(deprecated)]
impl From<FullPlaylistShadow> for FullPlaylist {
    fn from(shadow: FullPlaylistShadow) -> Self {
        Self {
            collaborative: shadow.collaborative,
            description: shadow.description,
            external_urls: shadow.external_urls,
            followers: shadow.followers,
            href: shadow.href,
            id: shadow.id,
            images: shadow.images,
            name: shadow.name,
            owner: shadow.owner,
            public: shadow.public,
            snapshot_id: shadow.snapshot_id,
            tracks: shadow.items.clone(),
            items: shadow.items,
        }
    }
}

/// Full playlist object
///
/// Note: The `tracks` field was renamed to `items` by Spotify. This struct
/// uses an internal "shadow" struct for deserialization to ensure that both
/// fields are correctly populated regardless of whether Spotify sends the old
/// or new key. This maintains compatibility before, during and after the
/// February 2026 API migration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(from = "FullPlaylistShadow")]
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
    pub tracks: Page<PlaylistItem>,
    pub items: Page<PlaylistItem>,
}

#[derive(Deserialize)]
struct PlaylistItemShadow {
    pub added_at: Option<DateTime<Utc>>,
    pub added_by: Option<PublicUser>,
    pub is_local: bool,
    #[serde(alias = "track")]
    pub item: Option<PlayableItem>,
}

#[allow(deprecated)]
impl From<PlaylistItemShadow> for PlaylistItem {
    fn from(shadow: PlaylistItemShadow) -> Self {
        Self {
            added_at: shadow.added_at,
            added_by: shadow.added_by,
            is_local: shadow.is_local,
            track: shadow.item.clone(),
            item: shadow.item,
        }
    }
}

/// Playlist track object
///
/// Note: The `track` field was renamed to `item` by Spotify. This struct
/// uses an internal "shadow" struct for deserialization to ensure that both
/// fields are correctly populated regardless of whether Spotify sends the old
/// or new key. This maintains compatibility before, during and after the
/// February 2026 API migration.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(from = "PlaylistItemShadow")]
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
    pub track: Option<PlayableItem>,
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

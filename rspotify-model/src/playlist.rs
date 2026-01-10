//! All kinds of playlists objects

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{Followers, Image, Page, PlayableItem, PlaylistId, PublicUser, UserId};

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

/// Deserialization helper for playlists that may have null fields.
/// This struct has optional fields to handle cases where the Spotify API
/// returns null values for playlist properties. After deserialization,
/// invalid playlists (with null IDs) are filtered out, and valid ones
/// are converted to [`SimplifiedPlaylist`].
#[derive(Deserialize)]
struct SimplifiedPlaylistOptional {
    pub collaborative: bool,
    pub description: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub href: Option<String>,
    pub id: Option<String>, // Use String for ID to handle null
    #[serde(deserialize_with = "deserialize_null_default")]
    pub images: Vec<Image>,
    pub name: Option<String>,
    pub owner: PublicUserOptional,
    pub public: Option<bool>,
    pub snapshot_id: Option<String>,
    pub tracks: PlaylistTracksRefOptional,
}

/// Deserialization helper for public users that may have null fields.
/// Handles null user data from the API and converts to [`PublicUser`] after validation.
#[derive(Deserialize)]
struct PublicUserOptional {
    pub display_name: Option<String>,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<Followers>,
    pub href: Option<String>,
    pub id: Option<String>,
    #[serde(default = "Vec::new")]
    pub images: Vec<Image>,
}

/// Deserialization helper for playlist tracks references that may have null fields.
/// Handles null tracks data from the API and converts to [`PlaylistTracksRef`]
/// after validation.
#[derive(Deserialize)]
struct PlaylistTracksRefOptional {
    pub href: Option<String>,
    pub total: Option<u32>,
}

impl SimplifiedPlaylistOptional {
    /// Attempts to convert this optional playlist into a valid [`SimplifiedPlaylist`].
    /// Returns `None` if any required fields are null or invalid.
    fn into_simplified_playlist(self) -> Option<SimplifiedPlaylist> {
        Some(SimplifiedPlaylist {
            collaborative: self.collaborative,
            external_urls: self.external_urls,
            href: self.href?,
            id: PlaylistId::from_id(self.id?).ok()?,
            images: self.images,
            name: self.name?,
            owner: self.owner.into_public_user()?,
            public: self.public,
            snapshot_id: self.snapshot_id?,
            tracks: self.tracks.into_playlist_tracks_ref()?,
        })
    }
}

impl PublicUserOptional {
    /// Attempts to convert this optional user into a valid [`PublicUser`].
    /// Returns `None` if any required fields are null or invalid.
    fn into_public_user(self) -> Option<PublicUser> {
        Some(PublicUser {
            display_name: self.display_name,
            external_urls: self.external_urls,
            followers: self.followers,
            href: self.href?,
            id: UserId::from_id(self.id?).ok()?,
            images: self.images,
        })
    }
}

impl PlaylistTracksRefOptional {
    /// Attempts to convert this optional tracks ref into a valid [`PlaylistTracksRef`].
    /// Returns `None` if any required fields are null.
    fn into_playlist_tracks_ref(self) -> Option<PlaylistTracksRef> {
        Some(PlaylistTracksRef {
            href: self.href?,
            total: self.total?,
        })
    }
}

/// Custom deserializer for category playlists that filters out invalid entries.
/// Deserializes a page of optional playlists, filters out those with null IDs,
/// and converts the remaining valid ones to [`SimplifiedPlaylist`].
fn deserialize_filtered_page<'de, D>(deserializer: D) -> Result<Page<SimplifiedPlaylist>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let optional_page: Page<SimplifiedPlaylistOptional> = Page::deserialize(deserializer)?;
    let filtered_items = optional_page
        .items
        .into_iter()
        .filter_map(|optional| optional.into_simplified_playlist())
        .collect();

    Ok(Page {
        href: optional_page.href,
        items: filtered_items,
        limit: optional_page.limit,
        next: optional_page.next,
        offset: optional_page.offset,
        previous: optional_page.previous,
        total: optional_page.total,
    })
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
    #[serde(deserialize_with = "deserialize_null_default")]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CategoryPlaylists {
    #[serde(deserialize_with = "deserialize_filtered_page")]
    pub playlists: Page<SimplifiedPlaylist>,
}

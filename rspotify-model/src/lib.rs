//! All Spotify API endpoint response objects. Please refer to the endpoints
//! where they are used for a link to their reference in the Spotify API
//! documentation.
pub mod album;
pub mod artist;
pub mod audio;
pub mod auth;
pub mod category;
pub mod context;
pub(crate) mod custom_serde;
pub mod device;
pub mod enums;
pub mod error;
pub mod idtypes;
pub mod image;
pub mod offset;
pub mod page;
pub mod playing;
pub mod playlist;
pub mod recommend;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

pub use {
    album::*, artist::*, audio::*, auth::*, category::*, context::*, device::*, enums::*, error::*,
    idtypes::*, image::*, offset::*, page::*, playing::*, playlist::*, recommend::*, search::*,
    show::*, track::*, user::*,
};

use serde::{Deserialize, Serialize};

/// Followers object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// A full track object or a full episode object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
    // The fallback variant to store the raw JSON for anything that doesn't parse
    // see https://github.com/ramsayleung/rspotify/issues/525 for
    // detail
    Unknown(serde_json::Value),
}

impl PlayableItem {
    /// Check if this is an unknown/malformed item that couldn't be parsed as Track or Episode.
    ///
    /// Returns `true` if the item was captured as raw JSON due to schema mismatch.
    pub fn is_unknown(&self) -> bool {
        matches!(self, PlayableItem::Unknown(_))
    }

    /// Utility to get the ID from either variant in the enum.
    ///
    /// Note that if it's a track and if it's local, it may not have an ID, in
    /// which case this function will return `None`.
    #[must_use]
    pub fn id(&self) -> Option<PlayableId<'_>> {
        match self {
            PlayableItem::Track(t) => t.id.as_ref().map(|t| PlayableId::Track(t.as_ref())),
            PlayableItem::Episode(e) => Some(PlayableId::Episode(e.id.as_ref())),
            PlayableItem::Unknown(value) => {
                let id_str = value.get("id")?.as_str()?;
                if let Some(type_str) = value.get("type").and_then(|v| v.as_str()) {
                    match type_str {
                        "episode" => Some(PlayableId::Episode(EpisodeId::from_id(id_str).ok()?)),
                        _ => Some(PlayableId::Track(TrackId::from_id(id_str).ok()?)),
                    }
                } else {
                    // Default to track if type is unclear
                    Some(PlayableId::Track(TrackId::from_id(id_str).ok()?))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
        // Assert artist
        let artist_id = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
        let id = ArtistId::from_id_or_uri(artist_id).unwrap();
        assert_eq!("2WX2uTcsvV5OnS0inACecP", id.id());

        // Assert album
        let album_id_a = "spotify/album/2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            "2WX2uTcsvV5OnS0inACecP",
            AlbumId::from_id_or_uri(album_id_a).unwrap().id()
        );

        // Mismatch type
        assert_eq!(
            Err(IdError::InvalidType),
            ArtistId::from_id_or_uri(album_id_a)
        );

        // Could not split
        let artist_id_c = "spotify-album-2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            Err(IdError::InvalidId),
            ArtistId::from_id_or_uri(artist_id_c)
        );

        let playlist_id = "spotify:playlist:59ZbFPES4DQwEjBpWHzrtC";
        assert_eq!(
            "59ZbFPES4DQwEjBpWHzrtC",
            PlaylistId::from_id_or_uri(playlist_id).unwrap().id()
        );
    }

    #[test]
    fn test_get_uri() {
        let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
        let track_id2 = "1301WleyT98MSxVHPZCA6M";
        let id1 = TrackId::from_id_or_uri(track_id1).unwrap();
        let id2 = TrackId::from_id_or_uri(track_id2).unwrap();
        assert_eq!(track_id1, &id1.uri());
        assert_eq!("spotify:track:1301WleyT98MSxVHPZCA6M", &id2.uri());
    }
}

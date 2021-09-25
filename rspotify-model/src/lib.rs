//! All Spotify API endpoint response objects.
//!
//! [Reference](https://developer.spotify.com/documentation/web-api/reference/#objects-index)

pub mod album;
pub mod artist;
pub mod audio;
pub mod auth;
pub mod category;
pub mod context;
pub(in crate) mod custom_serde;
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
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-followersobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// A full track object or a full episode object
///
/// + [Reference to full track](https://developer.spotify.com/documentation/web-api/reference/#object-trackobject)
/// + [Reference to full episode](https://developer.spotify.com/documentation/web-api/reference/#object-episodeobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayableItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

impl PlayableItem {
    /// Utility to get the ID from either variant in the enum
    pub fn id(&self) -> &dyn PlayableId {
        match self {
            PlayableItem::Track(t) => &t.id,
            PlayableItem::Episode(e) => &e.id,
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

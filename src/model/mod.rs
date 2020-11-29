//! All Spotify API endpoint response object
pub mod album;
pub mod artist;
pub mod audio;
pub mod category;
pub mod context;
pub mod device;
pub mod enums;
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

use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;

/// Restriction object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-restriction-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

/// Followers object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#followers-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

/// A full track object or a full episode object
///
/// + [Reference to full track](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full)
/// + [Reference to full episode](https://developer.spotify.com/documentation/web-api/reference/object-model/#episode-object-full)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Id<'id> {
    _type: Type,
    id: &'id str,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Error)]
pub enum IdError {
    InvalidPrefix,
    InvalidFormat,
    InvalidType,
    InvalidId,
}

impl std::fmt::Display for Id<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("spotify:")?;
        f.write_str(self._type.as_ref())?;
        f.write_str(":")?;
        f.write_str(self.id)?;
        Ok(())
    }
}

impl AsRef<str> for Id<'_> {
    fn as_ref(&self) -> &str {
        self.id
    }
}

impl Id<'_> {
    pub fn _type(&self) -> Type {
        self._type
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", self._type.as_ref(), self.id)
    }

    pub fn from_id_or_uri<'a, 'b: 'a>(_type: Type, id_or_uri: &'b str) -> Result<Id<'a>, IdError> {
        match Self::from_uri(id_or_uri) {
            Ok(id) if id._type == _type => Ok(id),
            Ok(_) => Err(IdError::InvalidType),
            Err(IdError::InvalidPrefix) => Self::from_id(_type, id_or_uri),
            Err(error) => Err(error),
        }
    }

    pub fn from_id<'a, 'b: 'a>(_type: Type, id: &'b str) -> Result<Id<'a>, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            Ok(Id { _type, id })
        } else {
            Err(IdError::InvalidId)
        }
    }

    pub fn from_uri<'a, 'b: 'a>(uri: &'b str) -> Result<Id<'a>, IdError> {
        if let Some((tpe, id)) = if let Some(uri) = uri.strip_prefix("spotify:") {
            uri.rfind(':').map(|mid| uri.split_at(mid))
        } else if let Some(uri) = uri.strip_prefix("spotify/") {
            uri.rfind('/').map(|mid| uri.split_at(mid))
        } else {
            return Err(IdError::InvalidPrefix);
        } {
            let _type = match tpe {
                "artist" => Type::Artist,
                "album" => Type::Album,
                "track" => Type::Track,
                "user" => Type::User,
                "playlist" => Type::Playlist,
                "show" => Type::Show,
                "episode" => Type::Episode,
                _ => return Err(IdError::InvalidType),
            };

            Self::from_id(_type, &id[1..])
        } else {
            Err(IdError::InvalidFormat)
        }
    }
}

pub use {
    album::*, artist::*, audio::*, category::*, context::*, device::*, enums::*, image::*,
    offset::*, page::*, playing::*, playlist::*, recommend::*, search::*, show::*, track::*,
    user::*,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_id() {
        // Assert artist
        let artist_id = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
        let id = Id::from_id_or_uri(Type::Artist, artist_id).unwrap();
        assert_eq!("2WX2uTcsvV5OnS0inACecP", id.id());

        // Assert album
        let album_id_a = "spotify/album/2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            "2WX2uTcsvV5OnS0inACecP",
            Id::from_id_or_uri(Type::Album, album_id_a).unwrap().id()
        );

        // Mismatch type
        assert_eq!(
            Err(IdError::InvalidType),
            Id::from_id_or_uri(Type::Artist, album_id_a)
        );

        // Could not split
        let artist_id_c = "spotify-album-2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            Err(IdError::InvalidId),
            Id::from_id_or_uri(Type::Artist, artist_id_c)
        );

        let playlist_id = "spotify:playlist:59ZbFPES4DQwEjBpWHzrtC";
        assert_eq!(
            "59ZbFPES4DQwEjBpWHzrtC",
            Id::from_id_or_uri(Type::Playlist, playlist_id)
                .unwrap()
                .id()
        );
    }

    #[test]
    fn test_get_uri() {
        let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
        let track_id2 = "1301WleyT98MSxVHPZCA6M";
        let id1 = Id::from_id_or_uri(Type::Track, track_id1).unwrap();
        let id2 = Id::from_id_or_uri(Type::Track, track_id2).unwrap();
        assert_eq!(track_id1, &id1.uri());
        assert_eq!("spotify:track:1301WleyT98MSxVHPZCA6M", &id2.uri());
    }
}

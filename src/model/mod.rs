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

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{de, Deserialize, Serialize, Serializer};
use std::{fmt, time::Duration};
use strum::Display;
use thiserror::Error;

/// Vistor to help deserialize duration represented as millisecond to `std::time::Duration`
struct DurationVisitor;
impl<'de> de::Visitor<'de> for DurationVisitor {
    type Value = Duration;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a milliseconds represents std::time::Duration")
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Duration::from_millis(v))
    }
}

/// Deserialize `std::time::Duration` from milliseconds (represented as u64)
pub(in crate) fn from_duration_ms<'de, D>(d: D) -> Result<Duration, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_u64(DurationVisitor)
}

/// Serialize `std::time::Duration` to milliseconds (represented as u64)
pub(in crate) fn to_duration_ms<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(x.as_millis() as u64)
}

/// Vistor to help deserialize unix millisecond timestamp to `chrono::DateTime`
struct DateTimeVisitor;

impl<'de> de::Visitor<'de> for DateTimeVisitor {
    type Value = DateTime<Utc>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an unix millisecond timestamp represents DataTime<UTC>"
        )
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let second = (v - v % 1000) / 1000;
        let nanosecond = ((v % 1000) * 1000000) as u32;
        // The maximum value of i64 is large enough to hold millisecond, so it would be safe to convert it i64
        let dt = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(second as i64, nanosecond),
            Utc,
        );
        Ok(dt)
    }
}

/// Deserialize Unix millisecond timestamp to `DateTime<Utc>`
pub(in crate) fn from_millisecond_timestamp<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_u64(DateTimeVisitor)
}

/// Serialize DateTime<Utc> to Unix millisecond timestamp
pub(in crate) fn to_millisecond_timestamp<S>(x: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_i64(x.timestamp_millis())
}

/// Vistor to help deserialize duration represented as millisecond to `Option<std::time::Duration>`
struct OptionDurationVisitor;

impl<'de> de::Visitor<'de> for OptionDurationVisitor {
    type Value = Option<Duration>;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a optional milliseconds represents std::time::Duration"
        )
    }
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Some(deserializer.deserialize_u64(DurationVisitor)?))
    }
}

/// Deserialize `Option<std::time::Duration>` from milliseconds (represented as u64)
pub(in crate) fn from_option_duration_ms<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_option(OptionDurationVisitor)
}

/// Serialize `Option<std::time::Duration>` to milliseconds (represented as u64)
pub(in crate) fn to_option_duration_ms<S>(x: &Option<Duration>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *x {
        Some(duration) => s.serialize_u64(duration.as_millis() as u64),
        None => s.serialize_none(),
    }
}

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

/// A Spotify object id of given [type](crate::model::enums::types::Type)
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
        write!(f, "spotify:{}:{}", self._type, self.id)
    }
}

impl AsRef<str> for Id<'_> {
    fn as_ref(&self) -> &str {
        self.id
    }
}

impl Id<'_> {
    /// Spotify object type
    pub fn _type(&self) -> Type {
        self._type
    }

    pub(in crate) fn check_type(self, _type: Type) -> Result<Self, IdError> {
        if self._type == _type {
            Ok(self)
        } else {
            Err(IdError::InvalidType)
        }
    }

    /// Spotify object id (guaranteed to be a string of alphanumeric characters)
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Spotify object URI in a well-known format: spotify:type:id
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`, `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", self._type, self.id)
    }

    /// Full Spotify object URL, can be opened in a browser
    ///
    /// Examples: https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an, https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI
    pub fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", self._type, self.id)
    }

    /// Parse Spotify id or URI from string slice
    ///
    /// Spotify URI must be in one of the following formats: `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`, `show`, or `episode`,
    /// and `{id}` is a non-empty alphanumeric string.
    /// The URI must be of given `_type`, otherwise `IdError::InvalidType` error is returned.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`, `spotify/track/4y4VO05kYgUTo2bzbox1an`.
    ///
    /// If input string is not a valid Spotify URI (it's not started with `spotify:` or `spotify/`),
    /// it must be a valid Spotify object id, i.e. a non-empty alphanumeric string.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidType` - if `id_or_uri` is an URI, and it's type part is not equal to `_type`,
    /// - `IdError::InvalidId` - either if `id_or_uri` is an URI with invalid id part, or it's an invalid id
    ///    (id is invalid if it contains non-alphanumeric characters),
    /// - `IdError::InvalidFormat` - if `id_or_uri` is an URI, and it can't be split into type and id parts.
    pub fn from_id_or_uri<'a, 'b: 'a>(_type: Type, id_or_uri: &'b str) -> Result<Id<'a>, IdError> {
        match Self::from_uri(id_or_uri) {
            Ok(id) if id._type == _type => Ok(id),
            Ok(_) => Err(IdError::InvalidType),
            Err(IdError::InvalidPrefix) => Self::from_id(_type, id_or_uri),
            Err(error) => Err(error),
        }
    }

    /// Parse Spotify id from string slice
    ///
    /// A valid Spotify object id must be a non-empty alphanumeric string.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidId` - if `id` contains non-alphanumeric characters.
    pub fn from_id<'a, 'b: 'a>(_type: Type, id: &'b str) -> Result<Id<'a>, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            Ok(Id { _type, id })
        } else {
            Err(IdError::InvalidId)
        }
    }

    /// Parse Spotify URI from string slice
    ///
    /// Spotify URI must be in one of the following formats: `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`, `show`, or `episode`,
    /// and `{id}` is a non-empty alphanumeric string.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`, `spotify/track/4y4VO05kYgUTo2bzbox1an`.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidPrefix` - if `uri` is not started with `spotify:` or `spotify/`,
    /// - `IdError::InvalidType` - if type part of an `uri` is not a valid Spotify type,
    /// - `IdError::InvalidId` - if id part of an `uri` is not a valid id,
    /// - `IdError::InvalidFormat` - if it can't be splitted into type and id parts.
    pub fn from_uri<'a, 'b: 'a>(uri: &'b str) -> Result<Id<'a>, IdError> {
        let rest = uri.strip_prefix("spotify").ok_or(IdError::InvalidPrefix)?;
        let sep = match rest.chars().next() {
            Some(ch) if ch == '/' || ch == ':' => ch,
            _ => return Err(IdError::InvalidPrefix),
        };
        let rest = &rest[1..];

        if let Some((tpe, id)) = rest.rfind(sep).map(|mid| rest.split_at(mid)) {
            let _type = tpe.parse().map_err(|_| IdError::InvalidType)?;
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

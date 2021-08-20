//! This module defines the necessary elements in order to represent Spotify IDs
//! and URIs with type safety and no overhead.
//!
//! The struct [`Id`] is the central element of this module. It's generic over
//! its type ([`Album`], [`Episode`], etc), and implements the logic to
//! initialize and use IDs. [`Id`] is equivalent to a `&str` and [`IdBuf`] to a
//! `String` so that there's no overhead at all; you may use whichever suits you
//! best.
//!
//! This module also exports aliases to make its usage simpler; [`AlbumId`] is
//! equivalent to `Id<Album>`, and [`AlbumIdBuf`] is the same as `IdBuf<Album>`.
//!
//! The types `Id` is generic over are grouped up in traits like
//! [`PlayableIdType`] for cases when the ID may be of different values.
//!
//! ## Skipping the type safety
//!
//! Note that sometimes the ID's type is only known at runtime, so it's
//! impossible to handle IDs with type safety. [`UnknownId`] and
//! [`UnknownIdBuf`] can be used for that:
//!
//! 1. They don't check the type of URI it's dealing with when initialized (it
//!    still needs to be a variant from [`crate::Type`])
//! 2. They may be used anywhere, as they implement every single trait in this
//!    module.
//!
//! ## Examples
//!
//! You may let the type inferrer guess what kind of ID you're dealing with by
//! using [`Id`]:
//!
//!
//! ```
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = Id::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id); // this function takes a `TrackId`, so `id` must be one
//! ```
//!
//! You can also specify the type explicitly by not using [`Id`] directly:
//!
//! ```
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = TrackId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id);
//! ```
//!
//! Notice how it's type safe; this would fail at compile-time:
//!
//! ```compile_fail
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = EpisodeId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id);
//! ```
//!
//! And this would panic because it's a `TrackId` but its URI string specifies
//! it's an album (`spotify:album:xxxx`).
//!
//! ```should_panic
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = TrackId::from_id("spotify:album:6akEvsycLGftJxYudPjmqK").unwrap();
//! pause_track(id);
//! ```

use crate::Type;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::ops::Deref;
use strum::Display;
use thiserror::Error;

// This is a sealed trait pattern implementation, it stops external code from
// implementing the `IdType` trait. The `Sealed` trait must be in a private mod,
// so external code can not see and implement it.
//
// We use the `sealed_types` macro for an easier implementation here.
//
// See also: https://rust-lang.github.io/api-guidelines/future-proofing.html
mod private {
    pub trait Sealed {}
}

pub trait IdType: private::Sealed + Send + Sync {
    const TYPE: Type;
}
pub trait PlayableIdType: IdType {}
pub trait PlayContextIdType: IdType {}

/// This macro helps consistently define ID types by implementing the sealed
/// trait and creating aliases.
macro_rules! sealed_idtypes {
    ($($name:ident => $alias:ident, $alias_buf:ident);+) => {
        $(
            #[doc = "Please refer to [`crate::idtypes`] for more information."]
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            pub enum $name {}
            impl private::Sealed for $name {}
            impl IdType for $name {
                const TYPE: Type = Type::$name;
            }

            #[doc = concat!("Alias for `Id<", stringify!($name), ">`. Please refer to [`crate::idtypes`] for more information.")]
            pub type $alias = Id<$name>;
            #[doc = concat!("Owned type for [`", stringify!($alias), "`]. Please refer to [`crate::idtypes`] for more information.")]
            pub type $alias_buf = IdBuf<$name>;
        )+
    }
}

sealed_idtypes!(
    Unknown => UnknownId, UnknownIdBuf;
    Artist => ArtistId, ArtistIdBuf;
    Album => AlbumId, AlbumIdBuf;
    Track => TrackId, TrackIdBuf;
    Playlist => PlaylistId, PlaylistIdBuf;
    User => UserId, UserIdBuf;
    Show => ShowId, ShowIdBuf;
    Episode => EpisodeId, EpisodeIdBuf
);

impl PlayContextIdType for Unknown {}
impl PlayContextIdType for Artist {}
impl PlayContextIdType for Album {}
impl PlayContextIdType for Playlist {}
impl PlayContextIdType for Show {}
impl PlayableIdType for Unknown {}
impl PlayableIdType for Track {}
impl PlayableIdType for Episode {}

/// A Spotify object ID of a given [type](crate::enums::types::Type).
///
/// This is a not-owning type, it stores a `&str` only. See
/// [IdBuf](crate::idtypes::IdBuf) for owned version of the type.
#[derive(Debug, PartialEq, Eq, Serialize, Hash)]
pub struct Id<T> {
    #[serde(default)]
    _type: PhantomData<T>,
    #[serde(flatten)]
    id: str,
}

/// An owned Spotify object ID of a given [type](crate::enums::types::Type).
///
/// This is an owning type, it stores a `String`. See [Id](crate::idtypes::Id)
/// for light-weight non-owning type.
///
/// Use `Id::from_id(val).to_owned()`, `Id::from_uri(val).to_owned()` or
/// `Id::from_id_or_uri(val).to_owned()` to construct an instance of this type.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct IdBuf<T> {
    #[serde(default)]
    _type: PhantomData<T>,
    #[serde(flatten)]
    id: String,
}

impl<T> AsRef<Id<T>> for IdBuf<T> {
    fn as_ref(&self) -> &Id<T> {
        // Safe, b/c of the same T between types, IdBuf can't be constructed
        // from invalid id, and Id is just a wrapped str with ZST type tag
        unsafe { &*(&*self.id as *const str as *const Id<T>) }
    }
}

impl<T> Borrow<Id<T>> for IdBuf<T> {
    fn borrow(&self) -> &Id<T> {
        self.as_ref()
    }
}

impl<T> Deref for IdBuf<T> {
    type Target = Id<T>;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: IdType> IdBuf<T> {
    /// Get a [`Type`](crate::enums::types::Type) of the id
    pub fn _type(&self) -> Type {
        T::TYPE
    }

    /// Spotify object id (guaranteed to be a string of alphanumeric characters)
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Spotify object URI in a well-known format: spotify:type:id
    pub fn uri(&self) -> String {
        self.as_ref().uri()
    }

    /// Full Spotify object URL, can be opened in a browser
    pub fn url(&self) -> String {
        self.as_ref().url()
    }
}

/// Spotify id or URI parsing error
///
/// See also [`Id`](crate::idtypes::Id) for details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Error)]
pub enum IdError {
    /// Spotify URI prefix is not `spotify:` or `spotify/`
    InvalidPrefix,
    /// Spotify URI can't be split into type and id parts
    /// (e.g. it has invalid separator)
    InvalidFormat,
    /// Spotify URI has invalid type name, or id has invalid type in a given
    /// context (e.g. a method expects a track id, but artist id is provided)
    InvalidType,
    /// Spotify id is invalid (empty or contains non-alphanumeric characters)
    InvalidId,
}

impl<T: IdType> std::fmt::Display for &Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "spotify:{}:{}", T::TYPE, &self.id)
    }
}

impl<T> AsRef<str> for &Id<T> {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl<T> Borrow<str> for &Id<T> {
    fn borrow(&self) -> &str {
        &self.id
    }
}

impl<T: IdType> std::str::FromStr for IdBuf<T> {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Id::from_id_or_uri(s).map(|id| id.to_owned())
    }
}

impl<T: IdType> Id<T> {
    /// Owned version of the id [`IdBuf`](crate::idtypes::IdBuf).
    pub fn to_owned(&self) -> IdBuf<T> {
        IdBuf {
            _type: PhantomData,
            id: (&self.id).to_owned(),
        }
    }

    /// Spotify object type
    pub fn _type(&self) -> Type {
        T::TYPE
    }

    /// Spotify object id (guaranteed to be a string of alphanumeric characters)
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Spotify object URI in a well-known format: spotify:type:id
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", T::TYPE, &self.id)
    }

    /// Full Spotify object URL, can be opened in a browser
    ///
    /// Examples: `https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an`,
    /// `https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI`.
    pub fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", T::TYPE, &self.id)
    }

    /// Parse Spotify id or URI from string slice
    ///
    /// Spotify URI must be in one of the following formats:
    /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`,
    /// `user`, `show`, or `episode`, and `{id}` is a non-empty
    /// alphanumeric string.
    /// The URI must be of given `T`ype, otherwise `IdError::InvalidType`
    /// error is returned.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify/track/4y4VO05kYgUTo2bzbox1an`.
    ///
    /// If input string is not a valid Spotify URI (it's not started with
    /// `spotify:` or `spotify/`), it must be a valid Spotify object id,
    /// i.e. a non-empty alphanumeric string.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidType` - if `id_or_uri` is an URI, and it's type part
    ///    is not equal to `T`,
    /// - `IdError::InvalidId` - either if `id_or_uri` is an URI with invalid id
    ///    part, or it's an invalid id (id is invalid if it contains
    ///    non-alphanumeric characters),
    /// - `IdError::InvalidFormat` - if `id_or_uri` is an URI, and it can't be
    ///    split into type and id parts.
    pub fn from_id_or_uri<'a, 'b: 'a>(id_or_uri: &'b str) -> Result<&'a Id<T>, IdError> {
        match Id::<T>::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::InvalidPrefix) => Id::<T>::from_id(id_or_uri),
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
    pub fn from_id<'a, 'b: 'a>(id: &'b str) -> Result<&'a Id<T>, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            // Safe, b/c Id is just a str with ZST type tag, and id is proved
            // to be a valid id at this point
            Ok(unsafe { &*(id as *const str as *const Id<T>) })
        } else {
            Err(IdError::InvalidId)
        }
    }

    /// Parse Spotify URI from string slice
    ///
    /// Spotify URI must be in one of the following formats:
    /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`,
    /// `show`, or `episode`, and `{id}` is a non-empty alphanumeric string.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify/track/4y4VO05kYgUTo2bzbox1an`.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidPrefix` - if `uri` is not started with `spotify:`
    ///    or `spotify/`,
    /// - `IdError::InvalidType` - if type part of an `uri` is not a valid
    ///    Spotify type `T`,
    /// - `IdError::InvalidId` - if id part of an `uri` is not a valid id,
    /// - `IdError::InvalidFormat` - if it can't be splitted into type and
    ///    id parts.
    pub fn from_uri<'a, 'b: 'a>(uri: &'b str) -> Result<&'a Id<T>, IdError> {
        let mut chars = uri
            .strip_prefix("spotify")
            .ok_or(IdError::InvalidPrefix)?
            .chars();
        let sep = match chars.next() {
            Some(ch) if ch == '/' || ch == ':' => ch,
            _ => return Err(IdError::InvalidPrefix),
        };
        let rest = chars.as_str();

        let (tpe, id) = rest
            .rfind(sep)
            .map(|mid| rest.split_at(mid))
            .ok_or(IdError::InvalidFormat)?;

        match tpe.parse::<Type>() {
            Ok(tpe) if T::TYPE == Type::Unknown || tpe == T::TYPE => Id::<T>::from_id(&id[1..]),
            _ => Err(IdError::InvalidType),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Valid values:
    const ID: &str = "4iV5W9uYEdYUVa79Axb7Rh";
    const URI: &str = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_SLASHES: &str = "spotify/track/4iV5W9uYEdYUVa79Axb7Rh";
    // Invalid values:
    const URI_EMPTY: &str = "spotify::4iV5W9uYEdYUVa79Axb7Rh";
    // Note that the API doesn't really have an 'Unknown' type.
    const URI_WRONGTYPE1: &str = "spotify:unknown:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_WRONGTYPE2: &str = "spotify:something:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_SHORT: &str = "track:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_MIXED1: &str = "spotify/track:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_MIXED2: &str = "spotify:track/4iV5W9uYEdYUVa79Axb7Rh";

    #[test]
    fn test_id_parse() {
        assert!(TrackId::from_id(ID).is_ok());
        assert_eq!(TrackId::from_id(URI), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_SLASHES), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_EMPTY), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_WRONGTYPE1), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_WRONGTYPE2), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_SHORT), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_MIXED1), Err(IdError::InvalidId));
        assert_eq!(TrackId::from_id(URI_MIXED2), Err(IdError::InvalidId));
    }

    #[test]
    fn test_uri_parse() {
        assert!(TrackId::from_uri(URI).is_ok());
        assert!(TrackId::from_uri(URI_SLASHES).is_ok());
        assert_eq!(TrackId::from_uri(ID), Err(IdError::InvalidPrefix));
        assert_eq!(TrackId::from_uri(URI_SHORT), Err(IdError::InvalidPrefix));
        assert_eq!(TrackId::from_uri(URI_EMPTY), Err(IdError::InvalidType));
        assert_eq!(TrackId::from_uri(URI_WRONGTYPE1), Err(IdError::InvalidType));
        assert_eq!(TrackId::from_uri(URI_WRONGTYPE2), Err(IdError::InvalidType));
        assert_eq!(TrackId::from_uri(URI_MIXED1), Err(IdError::InvalidFormat));
        assert_eq!(TrackId::from_uri(URI_MIXED2), Err(IdError::InvalidFormat));
    }

    #[test]
    fn test_id_or_uri_parse() {
        assert!(TrackId::from_id_or_uri(ID).is_ok());
        assert!(TrackId::from_id_or_uri(URI).is_ok());
        assert!(TrackId::from_id_or_uri(URI_SLASHES).is_ok());
        assert_eq!(TrackId::from_id_or_uri(URI_SHORT), Err(IdError::InvalidId));
        assert_eq!(
            TrackId::from_id_or_uri(URI_EMPTY),
            Err(IdError::InvalidType)
        );
        assert_eq!(
            TrackId::from_id_or_uri(URI_WRONGTYPE1),
            Err(IdError::InvalidType)
        );
        assert_eq!(
            TrackId::from_id_or_uri(URI_WRONGTYPE2),
            Err(IdError::InvalidType)
        );
        assert_eq!(
            TrackId::from_id_or_uri(URI_MIXED1),
            Err(IdError::InvalidFormat)
        );
        assert_eq!(
            TrackId::from_id_or_uri(URI_MIXED2),
            Err(IdError::InvalidFormat)
        );
    }

    #[test]
    fn test_unknown() {
        // Passing a Track ID to an Unknown ID type should work just fine.
        assert!(UnknownId::from_id(ID).is_ok());
        assert!(UnknownId::from_uri(URI).is_ok());
        assert!(UnknownId::from_uri(URI_WRONGTYPE1).is_ok());
        assert!(UnknownId::from_id_or_uri(ID).is_ok());
        assert!(UnknownId::from_id_or_uri(URI).is_ok());

        // The given type must still be a variant of the `Type` enum, so it will
        // fail for invalid ones.
        assert_eq!(UnknownId::from_uri(URI_EMPTY), Err(IdError::InvalidType));
        assert_eq!(
            UnknownId::from_uri(URI_WRONGTYPE2),
            Err(IdError::InvalidType)
        );

        // But it will still catch other kinds of error
        assert_eq!(
            UnknownId::from_id_or_uri(URI_SHORT),
            Err(IdError::InvalidId)
        );
        assert_eq!(
            UnknownId::from_id_or_uri(URI_MIXED1),
            Err(IdError::InvalidFormat)
        );
        assert_eq!(
            UnknownId::from_id_or_uri(URI_MIXED2),
            Err(IdError::InvalidFormat)
        );
    }
}

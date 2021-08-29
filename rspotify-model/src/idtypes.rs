//! This module defines the necessary elements in order to represent Spotify IDs
//! and URIs with type safety and no overhead.
//!
//! ## Concrete IDs
//!
//! The trait [`Id`] is the central element of this module. It's implemented by
//! different kinds of ID ([`AlbumId`], [`EpisodeId`], etc), and implements the
//! logic to initialize and use IDs.
//!
//! * [`Type::Artist`] => [`ArtistId`]
//! * [`Type::Album`] => [`AlbumId`]
//! * [`Type::Track`] => [`TrackId`]
//! * [`Type::Playlist`] => [`PlaylistId`]
//! * [`Type::User`] => [`UserId`]
//! * [`Type::Show`] => [`ShowId`]
//! * [`Type::Episode`] => [`EpisodeId`]
//!
//! [`Id`] is the borrowed variant, equivalent to a `&str`, and [`IdBuf`] is the
//! owned variant, like a `String`; you may use whichever suits you best.
//!
//! ## Group IDs
//!
//! There are also group IDs, which may contain different kinds of IDs and
//! therefore have an unknown type at compile time. They will perform no checks
//! for what kind of URI they are given. For example, you can use [`PlayableId`]
//! to hold IDs of either tracks or episodes, or [`AnyId`] for *any* kind of ID.
//! These types are useful when an endpoint takes different kinds of IDs as a
//! parameter, or when the kind of ID you're dealing with is known only at
//! runtime.
//!
//! * [`AnyId`] => Any ID
//! * [`PlayContextId`] => [`ArtistId`], [`AlbumId`], [`PlaylistId`], [`ShowId`]
//! * [`PlayableId`] => [`TrackId`], [`EpisodeId`]
//!
//! You can convert specific IDs into group ones with its `AsRef`
//! implementation, since it's a cheap type conversion.
//!
//! ## Examples
//!
//! If an endpoint requires a `TrackId`, you may pass it as:
//!
//! ```
//! use rspotify_model::{Id, TrackId};
//!
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = TrackId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id);
//! ```
//!
//! Notice how it's type safe; the following example would fail at compile-time:
//!
//! ```compile_fail
//! use rspotify_model::{Id, TrackId, EpisodeId};
//!
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
//! use rspotify_model::{Id, TrackId};
//!
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = TrackId::from_uri("spotify:album:6akEvsycLGftJxYudPjmqK").unwrap();
//! pause_track(id);
//! ```
//!
//! A more complex example where an endpoint takes a vector of IDs of different
//! types:
//!
//! ```
//! use rspotify_model::{Id, TrackId, EpisodeId, PlayableId};
//!
//! fn track(id: &TrackId) { /* ... */ }
//! fn episode(id: &EpisodeId) { /* ... */ }
//! fn add_to_queue(id: &[&PlayableId]) { /* ... */ }
//!
//! let tracks = &[
//!     TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
//!     TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap(),
//! ];
//! let episodes = &[
//!     EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
//!     EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap(),
//! ];
//!
//! // First we get some info about the tracks and episodes
//! let track_info = tracks.iter().map(|id| track(id)).collect::<Vec<_>>();
//! let ep_info = episodes.iter().map(|id| episode(id)).collect::<Vec<_>>();
//! println!("Track info: {:?}", track_info);
//! println!("Episode info: {:?}", ep_info);
//!
//! // And then we play them
//! let playable = tracks
//!     .iter()
//!     .map(|id| id.as_ref())
//!     .chain(
//!         episodes.iter().map(|id| id.as_ref())
//!     )
//!     .collect::<Vec<_>>();
//! add_to_queue(&playable);
//! ```

use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;

use std::borrow::{Borrow, ToOwned};
use std::fmt::Debug;
use std::hash::Hash;

use crate::Type;

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

/// The main interface for an ID.
///
/// # Implementation note
///
/// Note that the requried usage for IDs requires this trait to be object-safe.
/// Thanks to that, it is possible to use `Vec<Box<dyn Id>>` to take multiple
/// kinds of IDs, and `Box<dyn Id>` in case the type isn't known at compile
/// time. This is why it includes both [`Self::_type`] and
/// [`Self::_type_static`], and why all of the static methods use `where Self:
/// Sized`.
///
/// Unfortunately, since `where Self: Sized` has to be enforced, IDs cannot be
/// simply a `str` internally because these aren't sized. Thus, IDs will have the
/// slight overhead of having to use an owned type like `String` -- though
/// having a single type for IDs that is owned makes its usage considerably
/// simpler.
pub trait Id: Send + Sync {
    /// Spotify object Id (guaranteed to be a string of alphanumeric characters)
    fn id(&self) -> &str;

    /// The type of the ID. The difference with [`Self::_type_static`] is that
    /// this method can be used so that `Id` is an object-safe trait.
    fn _type(&self) -> Type;

    /// The type of the ID, which can be used without initializing it
    fn _type_static() -> Type
    where
        Self: Sized;

    /// Initialize the Id without checking its validity.
    ///
    /// # Safety
    ///
    /// The string passed to this method must be made out of alphanumeric
    /// numbers only; otherwise undefined behaviour may occur.
    unsafe fn from_id_unchecked(id: &str) -> Self
    where
        Self: Sized;

    /// Spotify object URI in a well-known format: `spotify:type:id`
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    fn uri(&self) -> String {
        format!("spotify:{}:{}", self._type(), self.id())
    }

    /// Full Spotify object URL, can be opened in a browser
    ///
    /// Examples: `https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an`,
    /// `https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI`.
    fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", self._type(), self.id())
    }

    /// Parse Spotify id from string slice
    ///
    /// A valid Spotify object id must be a non-empty alphanumeric string.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidId` - if `id` contains non-alphanumeric characters.
    fn from_id(id: &str) -> Result<Self, IdError>
    where
        Self: Sized,
    {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            // Safe, we've just checked that the Id is valid.
            Ok(unsafe { Self::from_id_unchecked(id) })
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
    fn from_uri(uri: &str) -> Result<Self, IdError>
    where
        Self: Sized,
    {
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

        // Note that in case the type isn't known at compile time, any type will
        // be accepted.
        match tpe.parse::<Type>() {
            Ok(tpe) if tpe == Self::_type_static() => Self::from_id(&id[1..]),
            _ => Err(IdError::InvalidType),
        }
    }

    /// Parse Spotify id or URI from string slice
    ///
    /// Spotify URI must be in one of the following formats:
    /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`,
    /// `show`, or `episode`, and `{id}` is a non-empty alphanumeric string.
    /// The URI must be match with the ID's type (`Id::TYPE`), otherwise
    /// `IdError::InvalidType` error is returned.
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
    fn from_id_or_uri(id_or_uri: &str) -> Result<Self, IdError>
    where
        Self: Sized,
    {
        match Self::from_uri(id_or_uri) {
            Ok(id) => Ok(id),
            Err(IdError::InvalidPrefix) => Self::from_id(id_or_uri),
            Err(error) => Err(error),
        }
    }
}

pub trait PlayableId: Id {}
pub trait PlayContextId: Id {}

/// This macro helps consistently define ID types. It contains a lot of code but
/// mostly it's just repetitive work that's not of much interest for being
/// trivial.
///
/// * The `$type` parameter indicates what type the ID is made out of (say,
///   `Artist`, `Album`...) from the enum `Type`.
/// * The `$name_borrowed` and `$name_owned` parameters are the identifiers of
///   the borrowed and owned structs for that type, respectively.
macro_rules! define_idtypes {
    ($($type:ident => $name:ident),+) => {
        $(
            #[doc = concat!(
                "ID of type [`Type::",
                stringify!($type),
                "`]. Refer to the [module-level docs][`crate::idtypes`] for more information.")
            ]
            #[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Hash)]
            pub struct $name(String);

            impl Id for $name {
                #[inline]
                fn id(&self) -> &str {
                    &self.0
                }

                #[inline]
                fn _type(&self) -> Type {
                    Type::$type
                }

                #[inline]
                fn _type_static() -> Type where Self: Sized {
                    Type::$type
                }

                #[inline]
                unsafe fn from_id_unchecked(id: &str) -> Self {
                    $name(id.to_owned())
                }
            }

            /// Cheap conversion to `str`
            impl AsRef<str> for $name {
                fn as_ref(&self) -> &str {
                    self.id()
                }
            }

            /// `Id`s may be borrowed as `str` the same way `Box<T>` may be
            /// borrowed as `T` or `String` as `str`
            impl Borrow<str> for $name {
                fn borrow(&self) -> &str {
                    self.id()
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.uri())
                }
            }

            /// Owned IDs can also be used to convert from a `str`
            impl std::str::FromStr for $name {
                type Err = IdError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Self::from_id_or_uri(s).map(|id| id.to_owned())
                }
            }
        )+
    }
}

define_idtypes!(
    Artist => ArtistId,
    Album => AlbumId,
    Track => TrackId,
    Playlist => PlaylistId,
    User => UserId,
    Show => ShowId,
    Episode => EpisodeId
);

impl PlayContextId for ArtistId {}
impl PlayContextId for AlbumId {}
impl PlayContextId for PlaylistId {}
impl PlayContextId for ShowId {}
impl PlayableId for TrackId {}
impl PlayableId for EpisodeId {}

#[cfg(test)]
mod test {
    use super::*;

    // Valid values:
    const ID: &str = "4iV5W9uYEdYUVa79Axb7Rh";
    const URI: &str = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_SLASHES: &str = "spotify/track/4iV5W9uYEdYUVa79Axb7Rh";
    // Invalid values:
    const URI_EMPTY: &str = "spotify::4iV5W9uYEdYUVa79Axb7Rh";
    // TODO: remove one of these
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
    fn test_multiple_types() {
        fn endpoint(_ids: impl IntoIterator<Item = Box<dyn PlayableId>>) {}

        let tracks: [Box<dyn PlayableId>; 4] = [
            Box::new(TrackId::from_id(ID).unwrap()),
            Box::new(TrackId::from_id(ID).unwrap()),
            Box::new(EpisodeId::from_id(ID).unwrap()),
            Box::new(EpisodeId::from_id(ID).unwrap()),
        ];
        endpoint(tracks);
    }

    #[test]
    fn test_unknown_at_compile_time() {
        fn endpoint1(input: &str, is_episode: bool) -> Box<dyn PlayableId> {
            if is_episode {
                Box::new(EpisodeId::from_id(input).unwrap())
            } else {
                Box::new(TrackId::from_id(input).unwrap())
            }
        }
        fn endpoint2(_id: &[Box<dyn PlayableId>]) {}

        let id = endpoint1(ID, false);
        endpoint2(&[id]);
    }
}

//! This module makes it possible to represent Spotify IDs and URIs with type
//! safety and almost no overhead.
//!
//! ## Concrete IDs
//!
//! The trait [`Id`] is the central element of this module. It's implemented by
//! all kinds of ID, and includes the main functionality to use them. Remember
//! that you will need to import this trait to access its methods. The easiest
//! way is to add `use rspotify::prelude::*`.
//!
//! * [`Type::Artist`] => [`ArtistId`]
//! * [`Type::Album`] => [`AlbumId`]
//! * [`Type::Track`] => [`TrackId`]
//! * [`Type::Playlist`] => [`PlaylistId`]
//! * [`Type::User`] => [`UserId`]
//! * [`Type::Show`] => [`ShowId`]
//! * [`Type::Episode`] => [`EpisodeId`]
//!
//! Every kind of ID defines its own validity function, i.e., what characters it
//! can be made up of, such as alphanumeric or any.
//!
//! These types are just wrappers for [`Cow<str>`], so their usage should be
//! quite similar overall.
//!
//! [`Cow<str>`]: [`std::borrow::Cow`]
//!
//! ## Examples
//!
//! If an endpoint requires a `TrackId`, you may pass it as:
//!
//! ```
//! # use rspotify_model::TrackId;
//! fn pause_track(id: TrackId<'_>) { /* ... */ }
//!
//! let id = TrackId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id);
//! ```
//!
//! Notice how this way it's type safe; the following example would fail at
//! compile-time:
//!
//! ```compile_fail
//! # use rspotify_model::{TrackId, EpisodeId};
//! fn pause_track(id: TrackId<'_>) { /* ... */ }
//!
//! let id = EpisodeId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(id);
//! ```
//!
//! And this would panic because it's a `TrackId` but its URI string specifies
//! it's an album (`spotify:album:xxxx`).
//!
//! ```should_panic
//! # use rspotify_model::TrackId;
//! fn pause_track(id: TrackId<'_>) { /* ... */ }
//!
//! let id = TrackId::from_uri("spotify:album:6akEvsycLGftJxYudPjmqK").unwrap();
//! pause_track(id);
//! ```
//!
//! A more complex example where an endpoint takes a vector of IDs of different
//! types:
//!
//! ```
//! use rspotify_model::{TrackId, EpisodeId, PlayableId};
//!
//! fn track(id: TrackId<'_>) { /* ... */ }
//! fn episode(id: EpisodeId<'_>) { /* ... */ }
//! fn add_to_queue(id: &[PlayableId<'_>]) { /* ... */ }
//!
//! let tracks = [
//!     TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
//!     TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap(),
//! ];
//! let episodes = [
//!     EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
//!     EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap(),
//! ];
//!
//! // First we get some info about the tracks and episodes
//! let track_info = tracks.iter().map(|id| track(id.as_ref())).collect::<Vec<_>>();
//! let ep_info = episodes.iter().map(|id| episode(id.as_ref())).collect::<Vec<_>>();
//! println!("Track info: {:?}", track_info);
//! println!("Episode info: {:?}", ep_info);
//!
//! // And then we add both the tracks and episodes to the queue
//! let playable = tracks
//!     .into_iter()
//!     .map(|t| t.as_ref().into())
//!     .chain(
//!         episodes.into_iter().map(|e| e.as_ref().into())
//!     )
//!     .collect::<Vec<PlayableId>>();
//! add_to_queue(&playable);
//! ```

use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;

use std::{borrow::Cow, fmt::Debug, hash::Hash};

use crate::Type;

/// Spotify ID or URI parsing error
///
/// See also [`Id`](crate::idtypes::Id) for details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Error)]
pub enum IdError {
    /// Spotify URI prefix is not `spotify:` or `spotify/`.
    InvalidPrefix,
    /// Spotify URI can't be split into type and id parts (e.g., it has invalid
    /// separator).
    InvalidFormat,
    /// Spotify URI has invalid type name, or id has invalid type in a given
    /// context (e.g. a method expects a track id, but artist id is provided).
    InvalidType,
    /// Spotify id is invalid (empty or contains invalid characters).
    InvalidId,
}

/// The main interface for an ID.
///
/// See the [module level documentation] for more information.
///
/// [module level documentation]: [`crate::idtypes`]
#[enum_dispatch]
pub trait Id {
    /// Returns the inner Spotify object ID, which is guaranteed to be valid for
    /// its type.
    fn id(&self) -> &str;

    /// The type of the ID, as a function.
    fn _type(&self) -> Type;

    /// Returns a Spotify object URI in a well-known format: `spotify:type:id`.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    fn uri(&self) -> String {
        format!("spotify:{}:{}", self._type(), self.id())
    }

    /// Returns a full Spotify object URL that can be opened in a browser.
    ///
    /// Examples: `https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an`,
    /// `https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI`.
    fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", self._type(), self.id())
    }
}

/// A lower level function to parse a URI into both its type and its actual ID.
/// Note that this function doesn't check the validity of the returned ID (e.g.,
/// whether it's alphanumeric; that should be done in `Id::from_id`).
///
/// This is only useful for advanced use-cases, such as implementing your own ID
/// type.
pub fn parse_uri(uri: &str) -> Result<(Type, &str), IdError> {
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

    // Note that in case the type isn't known at compile time,
    // any type will be accepted.
    match tpe.parse::<Type>() {
        Ok(tpe) => Ok((tpe, &id[1..])),
        _ => Err(IdError::InvalidType),
    }
}

/// This macro helps consistently define ID types.
///
/// * The `$type` parameter indicates what variant in `Type` the ID is for (say,
///   `Artist`, or `Album`).
/// * The `$name` parameter is the identifier of the struct.
/// * The `$validity` parameter is the implementation of `id_is_valid`.
macro_rules! define_idtypes {
    ($($type:ident => {
        name: $name:ident,
        validity: $validity:expr
    }),+) => {
        $(
            #[doc = concat!(
                "ID of type [`Type::", stringify!($type), "`]. The validity of \
                its characters is defined by the closure `",
                stringify!($validity), "`.\n\nRefer to the [module-level \
                docs][`crate::idtypes`] for more information. "
            )]
            #[repr(transparent)]
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
            pub struct $name<'a>(Cow<'a, str>);

            impl<'a> $name<'a> {
                /// The type of the ID, as a constant.
                const TYPE: Type = Type::$type;

                /// Only returns `true` in case the given string is valid
                /// according to that specific ID (e.g., some may require
                /// alphanumeric characters only).
                pub fn id_is_valid(id: &str) -> bool {
                    const VALID_FN: fn(&str) -> bool = $validity;
                    VALID_FN(id)
                }

                /// Initialize the ID without checking its validity.
                ///
                /// # Safety
                ///
                /// The string passed to this method must be made out of valid
                /// characters only; otherwise undefined behaviour may occur.
                pub unsafe fn from_id_unchecked<S>(id: S) -> Self
                    where
                        S: Into<Cow<'a, str>>
                {
                    Self(id.into())
                }

                /// Parse Spotify ID from string slice.
                ///
                /// A valid Spotify object id must be a non-empty string with
                /// valid characters.
                ///
                /// # Errors
                ///
                /// - `IdError::InvalidId` - if `id` contains invalid characters.
                pub fn from_id<S>(id: S) -> Result<Self, IdError>
                    where
                        S: Into<Cow<'a, str>>
                {
                    let id = id.into();
                    if Self::id_is_valid(&id) {
                        // Safe, we've just checked that the ID is valid.
                        Ok(unsafe { Self::from_id_unchecked(id) })
                    } else {
                        Err(IdError::InvalidId)
                    }
                }

                /// Parse Spotify URI from string slice
                ///
                /// Spotify URI must be in one of the following formats:
                /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
                /// Where `{type}` is one of `artist`, `album`, `track`,
                /// `playlist`, `user`, `show`, or `episode`, and `{id}` is a
                /// non-empty valid string.
                ///
                /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
                /// `spotify/track/4y4VO05kYgUTo2bzbox1an`.
                ///
                /// # Errors
                ///
                /// - `IdError::InvalidPrefix` - if `uri` is not started with
                ///   `spotify:` or `spotify/`,
                /// - `IdError::InvalidType` - if type part of an `uri` is not a
                ///   valid Spotify type `T`,
                /// - `IdError::InvalidId` - if id part of an `uri` is not a
                ///   valid id,
                /// - `IdError::InvalidFormat` - if it can't be splitted into
                ///   type and id parts.
                ///
                /// # Implementation details
                ///
                /// Unlike [`Self::from_id`], this method takes a `&str` rather
                /// than an `Into<Cow<str>>`. This is because the inner `Cow` in
                /// the ID would reference a slice from the given `&str` (i.e.,
                /// taking the ID out of the URI). The parameter wouldn't live
                /// long enough when using `Into<Cow<str>>`, so the only
                /// sensible choice is to just use a `&str`.
                pub fn from_uri(uri: &'a str) -> Result<Self, IdError> {
                    let (tpe, id) = parse_uri(&uri)?;
                    if tpe == Type::$type {
                        Self::from_id(id)
                    } else {
                        Err(IdError::InvalidType)
                    }
                }

                /// Parse Spotify ID or URI from string slice
                ///
                /// Spotify URI must be in one of the following formats:
                /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
                /// Where `{type}` is one of `artist`, `album`, `track`,
                /// `playlist`, `user`, `show`, or `episode`, and `{id}` is a
                /// non-empty valid string. The URI must be match with the ID's
                /// type (`Id::TYPE`), otherwise `IdError::InvalidType` error is
                /// returned.
                ///
                /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
                /// `spotify/track/4y4VO05kYgUTo2bzbox1an`.
                ///
                /// If input string is not a valid Spotify URI (it's not started
                /// with `spotify:` or `spotify/`), it must be a valid Spotify
                /// object ID, i.e. a non-empty valid string.
                ///
                /// # Errors
                ///
                /// - `IdError::InvalidType` - if `id_or_uri` is an URI, and
                ///   it's type part is not equal to `T`,
                /// - `IdError::InvalidId` - either if `id_or_uri` is an URI
                ///   with invalid id part, or it's an invalid id (id is invalid
                ///   if it contains valid characters),
                /// - `IdError::InvalidFormat` - if `id_or_uri` is an URI, and
                ///   it can't be split into type and id parts.
                ///
                /// # Implementation details
                ///
                /// Unlike [`Self::from_id`], this method takes a `&str` rather
                /// than an `Into<Cow<str>>`. This is because the inner `Cow` in
                /// the ID would reference a slice from the given `&str` (i.e.,
                /// taking the ID out of the URI). The parameter wouldn't live
                /// long enough when using `Into<Cow<str>>`, so the only
                /// sensible choice is to just use a `&str`.
                pub fn from_id_or_uri(id_or_uri: &'a str) -> Result<Self, IdError> {
                    match Self::from_uri(id_or_uri) {
                        Ok(id) => Ok(id),
                        Err(IdError::InvalidPrefix) => Self::from_id(id_or_uri),
                        Err(error) => Err(error),
                    }
                }

                /// This creates an ID with the underlying `&str` variant from a
                /// reference. Useful to use an ID multiple times without having
                /// to clone it.
                pub fn as_ref(&'a self) -> Self {
                    Self(Cow::Borrowed(self.0.as_ref()))
                }

                /// An ID is a `Cow` after all, so this will switch to the its
                /// owned version, which has a `'static` lifetime.
                pub fn into_static(self) -> $name<'static> {
                    $name(Cow::Owned(self.0.into_owned()))
                }

                /// Similar to [`Self::into_static`], but without consuming the
                /// original ID.
                pub fn clone_static(&self) -> $name<'static> {
                    $name(Cow::Owned(self.0.clone().into_owned()))
                }
            }

            impl Id for $name<'_> {
                fn id(&self) -> &str {
                    &self.0
                }

                fn _type(&self) -> Type {
                    Self::TYPE
                }
            }

            // Deserialization may take either an ID or an URI, so its
            // implementation has to be done manually.
            impl<'de> Deserialize<'de> for $name<'static> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct IdVisitor;

                    impl<'de> serde::de::Visitor<'de> for IdVisitor {
                        type Value = $name<'static>;

                        fn expecting(
                            &self, formatter: &mut std::fmt::Formatter<'_>
                        ) -> Result<(), std::fmt::Error>
                        {
                            let msg = concat!("ID or URI for struct ", stringify!($name));
                            formatter.write_str(msg)
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            $name::from_id_or_uri(value)
                                .map($name::into_static)
                                .map_err(serde::de::Error::custom)
                        }

                        fn visit_newtype_struct<A>(
                            self,
                            deserializer: A,
                        ) -> Result<Self::Value, A::Error>
                        where
                            A: serde::Deserializer<'de>,
                        {
                            deserializer.deserialize_str(self)
                        }

                        fn visit_seq<A>(
                            self,
                            mut seq: A,
                        ) -> Result<Self::Value, A::Error>
                        where
                            A: serde::de::SeqAccess<'de>,
                        {
                            let field: &str = seq.next_element()?
                                .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                            $name::from_id_or_uri(field)
                                .map($name::into_static)
                                .map_err(serde::de::Error::custom)
                        }
                    }

                    deserializer.deserialize_newtype_struct(stringify!($name), IdVisitor)
                }
            }

            /// `Id`s may be borrowed as `str` the same way `Box<T>` may be
            /// borrowed as `T` or `String` as `str`
            impl std::borrow::Borrow<str> for $name<'_> {
                fn borrow(&self) -> &str {
                    self.id()
                }
            }

            /// Displaying the ID shows its URI
            impl std::fmt::Display for $name<'_> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", self.uri())
                }
            }
        )+
    }
}

// First declaring the regular IDs. Those with custom behaviour will have to be
// declared manually later on.
define_idtypes!(
    Artist => {
        name: ArtistId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    Album => {
        name: AlbumId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    Track => {
        name: TrackId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    Playlist => {
        name: PlaylistId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    Show => {
        name: ShowId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    Episode => {
        name: EpisodeId,
        validity: |id| id.chars().all(|ch| ch.is_ascii_alphanumeric())
    },
    User => {
        name: UserId,
        validity: |_| true
    }
);

// We use `enum_dispatch` for dynamic dispatch, which is not only easier to use
// than `dyn`, but also more efficient.
/// Grouping up multiple kinds of IDs to treat them generically. This also
/// implements [`Id`], and [`From`] to instantiate it.
#[enum_dispatch(Id)]
pub enum PlayContextId<'a> {
    Artist(ArtistId<'a>),
    Album(AlbumId<'a>),
    Playlist(PlaylistId<'a>),
    Show(ShowId<'a>),
}
// These don't work with `enum_dispatch`, unfortunately.
impl<'a> PlayContextId<'a> {
    pub fn as_ref(&'a self) -> Self {
        match self {
            PlayContextId::Artist(x) => PlayContextId::Artist(x.as_ref()),
            PlayContextId::Album(x) => PlayContextId::Album(x.as_ref()),
            PlayContextId::Playlist(x) => PlayContextId::Playlist(x.as_ref()),
            PlayContextId::Show(x) => PlayContextId::Show(x.as_ref()),
        }
    }

    pub fn into_static(self) -> PlayContextId<'static> {
        match self {
            PlayContextId::Artist(x) => PlayContextId::Artist(x.into_static()),
            PlayContextId::Album(x) => PlayContextId::Album(x.into_static()),
            PlayContextId::Playlist(x) => PlayContextId::Playlist(x.into_static()),
            PlayContextId::Show(x) => PlayContextId::Show(x.into_static()),
        }
    }

    pub fn clone_static(&'a self) -> PlayContextId<'static> {
        match self {
            PlayContextId::Artist(x) => PlayContextId::Artist(x.clone_static()),
            PlayContextId::Album(x) => PlayContextId::Album(x.clone_static()),
            PlayContextId::Playlist(x) => PlayContextId::Playlist(x.clone_static()),
            PlayContextId::Show(x) => PlayContextId::Show(x.clone_static()),
        }
    }
}

/// Grouping up multiple kinds of IDs to treat them generically. This also
/// implements [`Id`] and [`From`] to instantiate it.
#[enum_dispatch(Id)]
pub enum PlayableId<'a> {
    Track(TrackId<'a>),
    Episode(EpisodeId<'a>),
}
// These don't work with `enum_dispatch`, unfortunately.
impl<'a> PlayableId<'a> {
    pub fn as_ref(&'a self) -> Self {
        match self {
            PlayableId::Track(x) => PlayableId::Track(x.as_ref()),
            PlayableId::Episode(x) => PlayableId::Episode(x.as_ref()),
        }
    }

    pub fn into_static(self) -> PlayableId<'static> {
        match self {
            PlayableId::Track(x) => PlayableId::Track(x.into_static()),
            PlayableId::Episode(x) => PlayableId::Episode(x.into_static()),
        }
    }

    pub fn clone_static(&'a self) -> PlayableId<'static> {
        match self {
            PlayableId::Track(x) => PlayableId::Track(x.clone_static()),
            PlayableId::Episode(x) => PlayableId::Episode(x.clone_static()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{borrow::Cow, error::Error};

    // Valid values:
    const ID: &str = "4iV5W9uYEdYUVa79Axb7Rh";
    const URI: &str = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    const URI_SLASHES: &str = "spotify/track/4iV5W9uYEdYUVa79Axb7Rh";
    // Invalid values:
    const URI_EMPTY: &str = "spotify::4iV5W9uYEdYUVa79Axb7Rh";
    const URI_WRONGTYPE1: &str = "spotify:unknown:4iV5W9uYEdYUVa79Axb7Rh";
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
        assert_eq!(TrackId::from_uri(URI_MIXED1), Err(IdError::InvalidFormat));
        assert_eq!(TrackId::from_uri(URI_MIXED2), Err(IdError::InvalidFormat));
    }

    /// Deserialization should accept both IDs and URIs as well.
    #[test]
    fn test_id_or_uri_and_deserialize() {
        fn test_any<F, E>(check: F)
        where
            F: Fn(&str) -> Result<TrackId<'_>, E>,
            E: Error,
        {
            // In this case we also check that the contents are the ID and not
            // the URI.
            assert!(check(ID).is_ok());
            assert_eq!(check(ID).unwrap().id(), ID);
            assert!(check(URI).is_ok());
            assert_eq!(check(URI).unwrap().id(), ID);
            assert!(check(URI_SLASHES).is_ok());
            assert_eq!(check(URI_SLASHES).unwrap().id(), ID);

            // These should not work in any case
            assert!(check(URI_SHORT).is_err());
            assert!(check(URI_EMPTY).is_err());
            assert!(check(URI_WRONGTYPE1).is_err());
            assert!(check(URI_MIXED1).is_err());
            assert!(check(URI_MIXED2).is_err());
        }

        // Easily testing both ways to obtain an ID
        test_any(|s| TrackId::from_id_or_uri(s));
        test_any(|s| {
            let json = format!("\"{s}\"");
            serde_json::from_str::<'_, TrackId>(&json)
        });
    }

    /// Serializing should return the Id within it, not the URI.
    #[test]
    fn test_serialize() {
        let json_expected = format!("\"{ID}\"");
        let track = TrackId::from_uri(URI).unwrap();
        let json = serde_json::to_string(&track).unwrap();
        assert_eq!(json, json_expected);
    }

    #[test]
    fn test_multiple_types() {
        fn endpoint<'a>(_ids: impl IntoIterator<Item = PlayableId<'a>>) {}

        let tracks: Vec<PlayableId> = vec![
            PlayableId::Track(TrackId::from_id(ID).unwrap()),
            PlayableId::Track(TrackId::from_id(ID).unwrap()),
            PlayableId::Episode(EpisodeId::from_id(ID).unwrap()),
            PlayableId::Episode(EpisodeId::from_id(ID).unwrap()),
        ];
        endpoint(tracks);
    }

    #[test]
    fn test_unknown_at_compile_time() {
        fn endpoint1(input: &str, is_episode: bool) -> PlayableId<'_> {
            if is_episode {
                PlayableId::Episode(EpisodeId::from_id(input).unwrap())
            } else {
                PlayableId::Track(TrackId::from_id(input).unwrap())
            }
        }
        fn endpoint2(_id: &[PlayableId]) {}

        let id = endpoint1(ID, false);
        endpoint2(&[id]);
    }

    #[test]
    fn test_constructor() {
        // With `&str`
        let _ = EpisodeId::from_id(ID).unwrap();
        // With `String`
        let _ = EpisodeId::from_id(ID.to_string()).unwrap();
        // With borrowed `Cow<str>`
        let _ = EpisodeId::from_id(Cow::Borrowed(ID)).unwrap();
        // With owned `Cow<str>`
        let _ = EpisodeId::from_id(Cow::Owned(ID.to_string())).unwrap();
    }

    #[test]
    fn test_owned() {
        // We check it twice to make sure cloning statically also works.
        fn check_static(_: EpisodeId<'static>) {}

        // With lifetime smaller than static because it's a locally owned
        // variable.
        let local_id = String::from(ID);

        // With `&str`: should be converted
        let id: EpisodeId<'_> = EpisodeId::from_id(local_id.as_str()).unwrap();
        check_static(id.clone_static());
        check_static(id.into_static());

        // With `String`: already static
        let id = EpisodeId::from_id(local_id.clone()).unwrap();
        check_static(id.clone());
        check_static(id);
    }
}

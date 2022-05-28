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
//! pause_track(&id);
//! ```
//!
//! Notice how this way it's type safe; the following example would fail at
//! compile-time:
//!
//! ```compile_fail
//! use rspotify_model::{Id, TrackId, EpisodeId};
//!
//! fn pause_track(id: &TrackId) { /* ... */ }
//!
//! let id = EpisodeId::from_id("4iV5W9uYEdYUVa79Axb7Rh").unwrap();
//! pause_track(&id);
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
//! pause_track(&id);
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
//! fn add_to_queue(id: &[&dyn PlayableId]) { /* ... */ }
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
//! // And then we add both the tracks and episodes to the queue
//! let playable = tracks
//!     .iter()
//!     .map(|id| id as &dyn PlayableId)
//!     .chain(
//!         episodes.iter().map(|id| id as &dyn PlayableId)
//!     )
//!     .collect::<Vec<&dyn PlayableId>>();
//! add_to_queue(&playable);
//! ```

use serde::{Deserialize, Serialize};
use strum::Display;
use thiserror::Error;

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
    /// Spotify id is invalid (empty or contains invalid characters)
    InvalidId,
}

/// The main interface for an ID.
///
/// # Implementation note
///
/// Note that for IDs to be useful their trait must be object-safe. Otherwise,
/// it wouldn't be possible to use `Vec<Box<dyn Id>>` to take multiple kinds of
/// IDs or just `Box<dyn Id>` in case the type wasn't known at compile time.
/// This is why this trait includes both [`Self::_type`] and
/// [`Self::_type_static`], and why all of the static methods use `where Self:
/// Sized`.
///
/// Unfortunately, since `where Self: Sized` has to be enforced, IDs cannot be
/// simply a `str` internally because these aren't sized. Thus, IDs will have the
/// slight overhead of having to use an owned type like `String`.
pub trait Id<'a> {
    /// The type of the ID.
    const TYPE: Type;

    /// Spotify object Id (guaranteed to be valid for that type)
    fn id(&self) -> &str;

    /// Only returns `true` in case the given string is valid according to that
    /// specific Id (e.g., some may require alphanumeric characters only).
    fn id_is_valid(id: &str) -> bool;

    /// Initialize the Id without checking its validity.
    ///
    /// # Safety
    ///
    /// The string passed to this method must be made out of valid characters
    /// only; otherwise undefined behaviour may occur.
    unsafe fn from_id_unchecked(id: &'a str) -> Self
    where
        Self: Sized;

    /// Spotify object URI in a well-known format: `spotify:type:id`
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    fn uri(&self) -> String {
        format!("spotify:{}:{}", Self::TYPE, self.id())
    }

    /// Full Spotify object URL, can be opened in a browser
    ///
    /// Examples: `https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an`,
    /// `https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI`.
    fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", Self::TYPE, self.id())
    }

    /// Parse Spotify Id from string slice.
    ///
    /// A valid Spotify object id must be a non-empty string with valid
    /// characters.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidId` - if `id` contains invalid characters.
    fn from_id(id: &'a str) -> Result<Self, IdError>
    where
        Self: Sized,
    {
        if Self::id_is_valid(id) {
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
    /// `show`, or `episode`, and `{id}` is a non-empty valid string.
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
    fn from_uri(uri: &'a str) -> Result<Self, IdError>
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
            Ok(tpe) if tpe == Self::TYPE => Self::from_id(&id[1..]),
            _ => Err(IdError::InvalidType),
        }
    }

    /// Parse Spotify id or URI from string slice
    ///
    /// Spotify URI must be in one of the following formats:
    /// `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`,
    /// `show`, or `episode`, and `{id}` is a non-empty valid string. The URI
    /// must be match with the ID's type (`Id::TYPE`), otherwise
    /// `IdError::InvalidType` error is returned.
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`,
    /// `spotify/track/4y4VO05kYgUTo2bzbox1an`.
    ///
    /// If input string is not a valid Spotify URI (it's not started with
    /// `spotify:` or `spotify/`), it must be a valid Spotify object ID,
    /// i.e. a non-empty valid string.
    ///
    /// # Errors:
    ///
    /// - `IdError::InvalidType` - if `id_or_uri` is an URI, and it's type part
    ///    is not equal to `T`,
    /// - `IdError::InvalidId` - either if `id_or_uri` is an URI with invalid id
    ///    part, or it's an invalid id (id is invalid if it contains valid
    ///    characters),
    /// - `IdError::InvalidFormat` - if `id_or_uri` is an URI, and it can't be
    ///    split into type and id parts.
    fn from_id_or_uri(id_or_uri: &'a str) -> Result<Self, IdError>
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

/// This macro helps consistently define ID types.
///
/// * The `$type` parameter indicates what type the ID is made out of (say,
///   `Artist`, `Album`...) from the enum `Type`.
/// * The `$name` parameter is the identifier of the struct for that type.
///
/// This macro contains a lot of code but mostly it's just repetitive work to
/// implement some common traits that's not of much interest for being trivial.
///
/// * The `$name` parameter is the identifier of the struct for that type.
macro_rules! define_idtypes {
    ($($type:ident => {
        name: $name:ident,
        validity: $validity:expr
    }),+) => {
        $(
            #[doc = concat!(
                "ID of type [`Type::", stringify!($type), "`]. Its \
                implementation of `id_is_valid` is defined by the closure `",
                stringify!($validity), "`.\nRefer to the [module-level \
                docs][`crate::idtypes`] for more information. "
            )]
            #[repr(transparent)]
            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Hash)]
            pub struct $name<'a>(::std::borrow::Cow<'a, str>);

            impl<'a> $name<'a> {
                pub fn into_static(self) -> $name<'static> {
                    $name(self.0.to_string().into())
                }

                pub fn clone_static(&self) -> $name<'static> {
                    $name(self.0.to_string().into())
                }
            }

            impl<'a> Id<'a> for $name<'a> {
                const TYPE: Type = Type::$type;

                #[inline]
                fn id(&self) -> &str {
                    &self.0
                }

                #[inline]
                fn id_is_valid(id: &str) -> bool {
                    // TODO: make prettier?
                    const VALID_FN: fn(&str) -> bool = $validity;
                    VALID_FN(id)
                }

                #[inline]
                unsafe fn from_id_unchecked(id: &'a str) -> Self {
                    Self(std::borrow::Cow::Borrowed(id))
                }
            }

            // Deserialization may take either an Id or an URI, so its
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

                        #[inline]
                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            $name::from_id_or_uri(value)
                                .map($name::into_static)
                                .map_err(serde::de::Error::custom)
                        }

                        #[inline]
                        fn visit_newtype_struct<A>(
                            self,
                            deserializer: A,
                        ) -> Result<Self::Value, A::Error>
                        where
                            A: serde::Deserializer<'de>,
                        {
                            deserializer.deserialize_str(self)
                        }

                        #[inline]
                        fn visit_seq<A>(
                            self,
                            mut seq: A,
                        ) -> Result<Self::Value, A::Error>
                        where
                            A: serde::de::SeqAccess<'de>,
                        {
                            let field = seq.next_element()?
                                .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                            $name::from_id_or_uri(field)
                                .map($name::into_static)
                                .map_err(serde::de::Error::custom)
                        }
                    }

                    deserializer.deserialize_newtype_struct(stringify!($name), IdVisitor)
                }
            }

            /// Cheap conversion to `str`
            impl AsRef<str> for $name<'_> {
                fn as_ref(&self) -> &str {
                    self.id()
                }
            }

            /// `Id`s may be borrowed as `str` the same way `Box<T>` may be
            /// borrowed as `T` or `String` as `str`
            impl std::borrow::Borrow<str> for $name<'_> {
                fn borrow(&self) -> &str {
                    self.id()
                }
            }

            impl<'a> $name<'a> {
                pub fn as_borrowed(&'a self) -> $name<'a> {
                    Self(std::borrow::Cow::Borrowed(self.0.as_ref()))
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

// TODO: replace with `enum_dispatch`?
macro_rules! define_idgroups {
    ($(pub enum $name:ident {
        $(
            $variant_name:ident($variant_type:ident)
        ),+
        $(,)?
    }),+) => {
        $(
            pub enum $name<'a> {
                $(
                    $variant_name($variant_type<'a>),
                )+
            }

            // TODO: turn into `impl Id`?
            // TODO: also implement `into_owned` and etc
            impl<'a> $name<'a> {
                pub fn uri(&self) -> String {
                    match self {
                        $(
                            $name::$variant_name(x) => x.uri(),
                        )+
                    }
                }
                pub fn url(&self) -> String {
                    match self {
                        $(
                            $name::$variant_name(x) => x.url(),
                        )+
                    }
                }
                pub const fn _type(&self) -> Type {
                    match self {
                        $(
                            $name::$variant_name(_) => $variant_type::TYPE,
                        )+
                    }
                }

                pub fn as_borrowed(&'a self) -> Self {
                    match self {
                        $(
                            $name::$variant_name(x) => $name::$variant_name(x.as_borrowed()),
                        )+
                    }
                }
            }
        )+
    }
}

// Grouping up the IDs into more specific traits
define_idgroups!(
    pub enum PlayContextId {
        Artist(ArtistId),
        Album(AlbumId),
        Playlist(PlaylistId),
        Show(ShowId),
    },
    pub enum PlayableId {
        Track(TrackId),
        Episode(EpisodeId),
    }
);

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

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
            F: Fn(&str) -> Result<&TrackId, E>,
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
            let json = format!("\"{}\"", s);
            serde_json::from_str::<'_, TrackId>(&json)
        });
    }

    /// Serializing should return the Id within it, not the URI.
    #[test]
    fn test_serialize() {
        let json_expected = format!("\"{}\"", ID);
        let track = TrackId::from_uri(URI).unwrap();
        let json = serde_json::to_string(&track).unwrap();
        assert_eq!(json, json_expected);
    }

    #[test]
    fn test_multiple_types() {
        fn endpoint(_ids: impl IntoIterator<Item = Playable>) {}

        let tracks: Vec<Playable> = vec![
            Playable::Track(TrackId::from_id(ID).unwrap()),
            Playable::Track(TrackId::from_id(ID).unwrap()),
            Playable::Episode(EpisodeId::from_id(ID).unwrap()),
            Playable::Episode(EpisodeId::from_id(ID).unwrap()),
        ];
        endpoint(tracks);
    }

    #[test]
    fn test_unknown_at_compile_time() {
        fn endpoint1(input: &str, is_episode: bool) -> Playable {
            if is_episode {
                Playable::Episode(EpisodeId::from_id(input).unwrap())
            } else {
                Playable::Track(TrackId::from_id(input).unwrap())
            }
        }
        fn endpoint2(_id: &[Playable]) {}

        let id = endpoint1(ID, false);
        endpoint2(&[id]);
    }
}

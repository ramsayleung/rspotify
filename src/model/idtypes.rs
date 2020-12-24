use crate::model::Type;
use serde::export::PhantomData;
use strum::Display;
use thiserror::Error;

mod private {
    pub trait Sealed {}
}

pub trait IdType: private::Sealed {
    const TYPE: Type;
}

impl IdType for Artist {
    const TYPE: Type = Type::Artist;
}
impl IdType for Album {
    const TYPE: Type = Type::Album;
}
impl IdType for Track {
    const TYPE: Type = Type::Track;
}
impl IdType for Playlist {
    const TYPE: Type = Type::Playlist;
}
impl IdType for User {
    const TYPE: Type = Type::User;
}
impl IdType for Show {
    const TYPE: Type = Type::Show;
}
impl IdType for Episode {
    const TYPE: Type = Type::Episode;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Artist {}
impl private::Sealed for Artist {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Album {}
impl private::Sealed for Album {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Track {}
impl private::Sealed for Track {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Playlist {}
impl private::Sealed for Playlist {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum User {}
impl private::Sealed for User {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Show {}
impl private::Sealed for Show {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Episode {}
impl private::Sealed for Episode {}

/// A Spotify object id of given [type](crate::model::enums::types::Type)
///
/// This is a not-owning type, it stores a &str only.
/// See [IdBuf](crate::model::idtypes::IdBuf) for owned version of the type.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Id<'id, T> {
    _type: PhantomData<T>,
    id: &'id str,
}

impl<'id, T> Id<'id, T> {
    pub fn to_owned(&self) -> IdBuf<T> {
        IdBuf {
            _type: PhantomData,
            id: self.id.to_owned(),
        }
    }
}

/// A Spotify object id of given [type](crate::model::enums::types::Type)
///
/// This is an owning type, it stores a String.
/// See [IdBuf](crate::model::idtypes::Id) for light-weight non-owning type.
///
/// Use `Id::from_id(val).to_owned()`, `Id::from_uri(val).to_owned()` or `Id::from_id_or_uri(val).to_owned()`
/// to construct an instance of this type.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IdBuf<T> {
    _type: PhantomData<T>,
    id: String,
}

impl<'id, T> Into<Id<'id, T>> for &'id IdBuf<T> {
    fn into(self) -> Id<'id, T> {
        Id {
            _type: PhantomData,
            id: &self.id,
        }
    }
}

impl<T: IdType> IdBuf<T> {
    /// Get a non-owning [`Id`](crate::model::idtypes::Id) representation of the id
    pub fn as_ref(&self) -> Id<'_, T> {
        self.into()
    }

    /// Get a [`Type`](crate::model::enums::types::Type) of the id
    pub fn _type(&self) -> Type {
        T::TYPE
    }

    /// Get id value as a &str
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Spotify id or URI parsing error
///
/// See also [`Id`](crate::model::idtypes::Id) for details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, Error)]
pub enum IdError {
    /// Spotify URI prefix is not `spotify:` or `spotify/`
    InvalidPrefix,
    /// Spotify URI can't be split into type and id parts (e.g. it has invalid separator)
    InvalidFormat,
    /// Spotify URI has invalid type name, or id has invalid type in a given context
    /// (e.g. a method expects a track id, but artist id is provided)
    InvalidType,
    /// Spotify id is invalid (empty or contains non-alphanumeric characters)
    InvalidId,
}

impl<T: IdType> std::fmt::Display for Id<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "spotify:{}:{}", T::TYPE, self.id)
    }
}

impl<T> AsRef<str> for Id<'_, T> {
    fn as_ref(&self) -> &str {
        self.id
    }
}

impl<T: IdType> std::str::FromStr for IdBuf<T> {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Id::from_id_or_uri(s).map(|id| id.to_owned())
    }
}

impl<T: IdType> Id<'_, T> {
    /// Spotify object type
    pub fn _type(&self) -> Type {
        T::TYPE
    }

    /// Spotify object id (guaranteed to be a string of alphanumeric characters)
    pub fn id(&self) -> &str {
        self.id
    }

    /// Spotify object URI in a well-known format: spotify:type:id
    ///
    /// Examples: `spotify:album:6IcGNaXFRf5Y1jc7QsE9O2`, `spotify:track:4y4VO05kYgUTo2bzbox1an`.
    pub fn uri(&self) -> String {
        format!("spotify:{}:{}", T::TYPE, self.id)
    }

    /// Full Spotify object URL, can be opened in a browser
    ///
    /// Examples: https://open.spotify.com/track/4y4VO05kYgUTo2bzbox1an, https://open.spotify.com/artist/2QI8e2Vwgg9KXOz2zjcrkI
    pub fn url(&self) -> String {
        format!("https://open.spotify.com/{}/{}", T::TYPE, self.id)
    }

    /// Parse Spotify id or URI from string slice
    ///
    /// Spotify URI must be in one of the following formats: `spotify:{type}:{id}` or `spotify/{type}/{id}`.
    /// Where `{type}` is one of `artist`, `album`, `track`, `playlist`, `user`, `show`, or `episode`,
    /// and `{id}` is a non-empty alphanumeric string.
    /// The URI must be of given `T`ype, otherwise `IdError::InvalidType` error is returned.
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
    pub fn from_id_or_uri<'a, 'b: 'a>(id_or_uri: &'b str) -> Result<Id<'a, T>, IdError> {
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
    pub fn from_id<'a, 'b: 'a>(id: &'b str) -> Result<Id<'a, T>, IdError> {
        if id.chars().all(|ch| ch.is_ascii_alphanumeric()) {
            Ok(Id {
                _type: PhantomData,
                id,
            })
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
    /// - `IdError::InvalidType` - if type part of an `uri` is not a valid Spotify type `T`,
    /// - `IdError::InvalidId` - if id part of an `uri` is not a valid id,
    /// - `IdError::InvalidFormat` - if it can't be splitted into type and id parts.
    pub fn from_uri<'a, 'b: 'a>(uri: &'b str) -> Result<Id<'a, T>, IdError> {
        let rest = uri.strip_prefix("spotify").ok_or(IdError::InvalidPrefix)?;
        let sep = match rest.chars().next() {
            Some(ch) if ch == '/' || ch == ':' => ch,
            _ => return Err(IdError::InvalidPrefix),
        };
        let rest = &rest[1..];

        if let Some((tpe, id)) = rest.rfind(sep).map(|mid| rest.split_at(mid)) {
            let _type: Type = tpe.parse().map_err(|_| IdError::InvalidType)?;
            if _type != T::TYPE {
                return Err(IdError::InvalidType);
            }
            Id::<T>::from_id(&id[1..])
        } else {
            Err(IdError::InvalidFormat)
        }
    }
}

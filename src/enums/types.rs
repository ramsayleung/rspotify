use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind) -> Error {
        Error { kind }
    }

    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}
/// The kind of an error that can occur.
#[derive(Clone, Debug)]
pub enum ErrorKind {
    /// This error occurs when no proper enum was found.
    NoEnum(String),
}
impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::NoEnum(_) => "no proper enum was found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NoEnum(ref s) => write!(f, "can't find proper enum of `{:?}`", s),
        }
    }
}
/// Album type - 'album', 'single', 'appears_on', 'compilation'
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}
impl FromStr for AlbumType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(AlbumType::Album),
            "single" => Ok(AlbumType::Single),
            "appears_on" => Ok(AlbumType::AppearsOn),
            "compilation" => Ok(AlbumType::Compilation),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
impl AlbumType {
    pub fn as_str(&self) -> &str {
        match *self {
            AlbumType::Album => "album",
            AlbumType::Single => "single",
            AlbumType::AppearsOn => "appears_on",
            AlbumType::Compilation => "compilation",
        }
    }
}
#[test]
fn test_album_type_convert_from_str() {
    let album_type = AlbumType::from_str("album");
    assert_eq!(album_type.unwrap(), AlbumType::Album);
    let empty_type = AlbumType::from_str("not exist album");
    assert_eq!(empty_type.is_err(), true);
}

///  Type: 'artist', 'album','track', 'playlist', 'show' or 'episode'
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
}
impl Type {
    pub fn as_str(&self) -> &str {
        match *self {
            Type::Album => "album",
            Type::Artist => "artist",
            Type::Track => "track",
            Type::Playlist => "playlist",
            Type::User => "user",
            Type::Show => "show",
            Type::Episode => "episode",
        }
    }
}
impl FromStr for Type {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(Type::Artist),
            "album" => Ok(Type::Album),
            "track" => Ok(Type::Track),
            "playlist" => Ok(Type::Playlist),
            "user" => Ok(Type::User),
            "show" => Ok(Type::Show),
            "episode" => Ok(Type::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_type_convert_from_str() {
    let _type = Type::from_str("album");
    assert_eq!(_type.unwrap(), Type::Album);

    let empty_type = Type::from_str("not_exist_type");
    assert_eq!(empty_type.is_err(), true);
}

/// additional_typs: track, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}
impl AdditionalType {
    pub fn as_str(&self) -> &str {
        match *self {
            AdditionalType::Track => "track",
            AdditionalType::Episode => "episode",
        }
    }
}
impl FromStr for AdditionalType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(AdditionalType::Track),
            "episode" => Ok(AdditionalType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}
/// currently_playing_type: track, episode, ad, unknown.
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    Advertisement,
    Unknown,
}
impl CurrentlyPlayingType {
    pub fn as_str(&self) -> &str {
        match *self {
            CurrentlyPlayingType::Track => "track",
            CurrentlyPlayingType::Episode => "episode",
            CurrentlyPlayingType::Advertisement => "ad",
            CurrentlyPlayingType::Unknown => "unknown",
        }
    }
}
impl FromStr for CurrentlyPlayingType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(CurrentlyPlayingType::Track),
            "episode" => Ok(CurrentlyPlayingType::Episode),
            "ad" => Ok(CurrentlyPlayingType::Advertisement),
            "unknown" => Ok(CurrentlyPlayingType::Unknown),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

/// Type for search: artist, album, track, playlist, show, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SearchType {
    Artist,
    Album,
    Track,
    Playlist,
    Show,
    Episode,
}

impl SearchType {
    pub fn as_str(&self) -> &str {
        match *self {
            SearchType::Album => "album",
            SearchType::Artist => "artist",
            SearchType::Track => "track",
            SearchType::Playlist => "playlist",
            SearchType::Show => "show",
            SearchType::Episode => "episode",
        }
    }
}
impl FromStr for SearchType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "artist" => Ok(SearchType::Artist),
            "album" => Ok(SearchType::Album),
            "track" => Ok(SearchType::Track),
            "playlist" => Ok(SearchType::Playlist),
            "show" => Ok(SearchType::Show),
            "episode" => Ok(SearchType::Episode),
            _ => Err(Error::new(ErrorKind::NoEnum(s.to_owned()))),
        }
    }
}

#[test]
fn test_convert_search_type_from_str() {
    let search_type = SearchType::from_str("artist");
    assert_eq!(search_type.unwrap(), SearchType::Artist);
    let unknown_search_type = SearchType::from_str("unknown_search_type");
    assert_eq!(unknown_search_type.is_err(), true);
}

/// Device Type: computer, smartphone, speaker, TV, etc.
/// See the [Spotify developer
/// docs](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
/// for more information, or in case we are missing a device type here.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Speaker,
    TV,
    AVR,
    STB,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}

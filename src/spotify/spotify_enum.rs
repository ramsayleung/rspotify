use hyper::StatusCode;

use std::fmt;
// album_type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilcation,
}
impl AlbumType {
    pub fn from_str(s: &str) -> Option<AlbumType> {
        match s {
            "album" => Some(AlbumType::Album),
            "single" => Some(AlbumType::Single),
            "appears_on" => Some(AlbumType::AppearsOn),
            "compilation" => Some(AlbumType::Compilcation),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &AlbumType::Album => "album",
            &AlbumType::Single => "single",
            &AlbumType::AppearsOn => "appears_on",
            &AlbumType::Compilcation => "compilation",
        }
    }
}
impl fmt::Debug for AlbumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AlbumType::Album => write!(f, "album"),
            AlbumType::Single => write!(f, "single"),
            AlbumType::AppearsOn => write!(f, "appears_on"),
            AlbumType::Compilcation => write!(f, "compilation"),
        }
    }
}

//  ‘artist’, ‘album’,‘track’ or ‘playlist’
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
}

impl Type {
    pub fn from_str(s: &str) -> Option<Type> {
        match s {
            "artist" => Some(Type::Artist),
            "album" => Some(Type::Album),
            "track" => Some(Type::Track),
            "playlist" => Some(Type::Playlist),
            "user" => Some(Type::User),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &Type::Album => "album",
            &Type::Artist => "artist",
            &Type::Track => "track",
            &Type::Playlist => "playlist",
            &Type::User => "user",
        }
    }
}

impl fmt::Debug for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Album => write!(f, "album"),
            Type::Artist => write!(f, "artist"),
            Type::Track => write!(f, "track"),
            Type::Playlist => write!(f, "playlist"),
            Type::User => write!(f, "user"),
        }
    }
}

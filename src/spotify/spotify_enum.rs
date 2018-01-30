use std::fmt;
// album_type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ALBUM_TYPE {
    Album,
    Single,
    AppearsOn,
    Compilcation,
}
impl ALBUM_TYPE {
    pub fn from_str(s: &str) -> Option<ALBUM_TYPE> {
        match s {
            "album" => Some(ALBUM_TYPE::Album),
            "single" => Some(ALBUM_TYPE::Single),
            "appears_on" => Some(ALBUM_TYPE::AppearsOn),
            "compilation" => Some(ALBUM_TYPE::Compilcation),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &ALBUM_TYPE::Album => "album",
            &ALBUM_TYPE::Single => "single",
            &ALBUM_TYPE::AppearsOn => "appears_on",
            &ALBUM_TYPE::Compilcation => "compilation",
        }
    }
}
impl fmt::Debug for ALBUM_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ALBUM_TYPE::Album => write!(f, "album"),
            ALBUM_TYPE::Single => write!(f, "single"),
            ALBUM_TYPE::AppearsOn => write!(f, "appears_on"),
            ALBUM_TYPE::Compilcation => write!(f, "compilation"),
        }
    }
}

//  ‘artist’, ‘album’,‘track’ or ‘playlist’
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TYPE {
    Artist,
    Album,
    Track,
    Playlist,
    User,
}

impl TYPE {
    pub fn from_str(s: &str) -> Option<TYPE> {
        match s {
            "artist" => Some(TYPE::Artist),
            "album" => Some(TYPE::Album),
            "track" => Some(TYPE::Track),
            "playlist" => Some(TYPE::Playlist),
            "user" => Some(TYPE::User),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &TYPE::Album => "album",
            &TYPE::Artist => "artist",
            &TYPE::Track => "track",
            &TYPE::Playlist => "playlist",
            &TYPE::User => "user",
        }
    }
}

impl fmt::Debug for TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TYPE::Album => write!(f, "album"),
            TYPE::Artist => write!(f, "artist"),
            TYPE::Track => write!(f, "track"),
            TYPE::Playlist => write!(f, "playlist"),
            TYPE::User => write!(f, "user"),
        }
    }
}

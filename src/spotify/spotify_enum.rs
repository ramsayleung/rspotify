use std::fmt;
// album_type - ‘album’, ‘single’, ‘appears_on’, ‘compilation’
#[derive(Clone, Serialize, Deserialize)]
pub enum ALBUM_TYPE {
    ALBUM,
    SINGLE,
    APPEARS_ON,
    COMPILCATION,
}
impl ALBUM_TYPE {
    pub fn from_str(s: &str) -> Option<ALBUM_TYPE> {
        match s {
            "album" => Some(ALBUM_TYPE::ALBUM),
            "single" => Some(ALBUM_TYPE::SINGLE),
            "appears_on" => Some(ALBUM_TYPE::APPEARS_ON),
            "compilation" => Some(ALBUM_TYPE::COMPILCATION),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &ALBUM_TYPE::ALBUM => "album",
            &ALBUM_TYPE::SINGLE => "single",
            &ALBUM_TYPE::APPEARS_ON => "appears_on",
            &ALBUM_TYPE::COMPILCATION => "compilation",
        }
    }
}
impl fmt::Debug for ALBUM_TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ALBUM_TYPE::ALBUM => write!(f, "album"),
            ALBUM_TYPE::SINGLE => write!(f, "single"),
            ALBUM_TYPE::APPEARS_ON => write!(f, "appears_on"),
            ALBUM_TYPE::COMPILCATION => write!(f, "compilation"),
        }
    }
}

//  ‘artist’, ‘album’,‘track’ or ‘playlist’
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum TYPE {
    #[serde(rename = "artist")]
    ARTIST,
    #[serde(rename = "album")]
    ALBUM,
    #[serde(rename = "track")]
    TRACK,
    #[serde(rename = "playlist")]
    PLAYLIST,
}

impl TYPE {
    pub fn from_str(s: &str) -> Option<TYPE> {
        match s {
            "artist" => Some(TYPE::ARTIST),
            "album" => Some(TYPE::ALBUM),
            "track" => Some(TYPE::TRACK),
            "playlist" => Some(TYPE::PLAYLIST),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            &TYPE::ALBUM => "album",
            &TYPE::ARTIST => "artist",
            &TYPE::TRACK => "track",
            &TYPE::PLAYLIST => "playtlist",
        }
    }
}

impl fmt::Debug for TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TYPE::ALBUM => write!(f, "album"),
            TYPE::ARTIST => write!(f, "artist"),
            TYPE::TRACK => write!(f, "track"),
            TYPE::PLAYLIST => write!(f, "playlist"),
        }
    }
}

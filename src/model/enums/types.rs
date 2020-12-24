use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, ToString};

/// Copyright type: `C` = the copyright, `P` = the sound recording (performance) copyright.
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#copyright-object)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
pub enum CopyrightType {
    #[strum(serialize = "P")]
    #[serde(rename = "P")]
    Performance,
    #[strum(serialize = "C")]
    #[serde(rename = "C")]
    Copyright,
}

/// Album type: `album`, `single`, `appears_on`, `compilation`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-full)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}

/// Type: `artist`, `album`, `track`, `playlist`, `show` or `episode`
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Type {
    Artist,
    Album,
    Track,
    Playlist,
    User,
    Show,
    Episode,
}

pub mod idtypes {
    use super::Type;

    pub trait IdType: PartialEq {
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Album {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Track {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Playlist {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum User {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Show {}
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Episode {}
}

/// Additional typs: `track`, `episode`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-information-about-the-users-current-playback/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}

/// Currently playing type: `track`, `episode`, `ad`, `unknown`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CurrentlyPlayingType {
    Track,
    Episode,
    #[strum(serialize = "ad")]
    #[serde(rename = "ad")]
    Advertisement,
    Unknown,
}

/// Type for search: `artist`, `album`, `track`, `playlist`, `show`, `episode`
///
/// [Reference](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SearchType {
    Artist,
    Album,
    Track,
    Playlist,
    Show,
    Episode,
}

/// The user's Spotify subscription level: `premium`, `free`
///
/// (The subscription level "open" can be considered the same as "free".)
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubscriptionLevel {
    Premium,
    #[serde(alias = "open")]
    Free,
}

/// Device Type: `computer`, `smartphone`, `speaker`, `TV`
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
#[derive(Clone, Debug, Serialize, Deserialize, ToString, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
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

/// Recommendations seed type
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/object-model/#recommendations-seed-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationsSeedType {
    Artist,
    Track,
    Genre,
}

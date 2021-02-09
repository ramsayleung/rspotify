use serde::{Deserialize, Serialize};
use strum::ToString;

/// Copyright type: `C` = the copyright, `P` = the sound recording (performance)
/// copyright.
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
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
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
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#category-search)
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

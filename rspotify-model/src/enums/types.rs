use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, IntoStaticStr};

/// Copyright type: `C` = the copyright, `P` = the sound recording (performance)
/// copyright.
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
pub enum CopyrightType {
    #[strum(serialize = "P")]
    #[serde(rename = "P")]
    Performance,
    #[strum(serialize = "C")]
    #[serde(rename = "C")]
    Copyright,
}

/// Album type: `album`, `single`, `appears_on`, `compilation`
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}

/// Type: `artist`, `album`, `track`, `playlist`, `show` or `episode`
#[derive(
    Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, Display, EnumString, IntoStaticStr,
)]
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
    Collection,
    Collectionyourepisodes, // rename to collectionyourepisodes
}

/// Additional typs: `track`, `episode`
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}

/// Currently playing type: `track`, `episode`, `ad`, `unknown`
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
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
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
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
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubscriptionLevel {
    Premium,
    #[serde(alias = "open")]
    Free,
}

/// Device Type: `computer`, `smartphone`, `speaker`, `TV`
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum DeviceType {
    Computer,
    Tablet,
    Smartphone,
    Smartwatch,
    Speaker,
    /// Though undocumented, it has been reported that the Web API returns both
    /// 'Tv' and 'TV' as the type.
    #[serde(alias = "TV")]
    Tv,
    /// Same as above, the Web API returns both 'AVR' and 'Avr' as the type.
    #[serde(alias = "AVR")]
    Avr,
    /// Same as above, the Web API returns both 'STB' and 'Stb' as the type.
    #[serde(alias = "STB")]
    Stb,
    AudioDongle,
    GameConsole,
    CastVideo,
    CastAudio,
    Automobile,
    Unknown,
}

/// Recommendations seed type
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, IntoStaticStr)]
#[serde(rename_all = "UPPERCASE")]
pub enum RecommendationsSeedType {
    Artist,
    Track,
    Genre,
}

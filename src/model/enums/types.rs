use serde::{Deserialize, Serialize};
use strum::ToString;

/// Album type - 'album', 'single', 'appears_on', 'compilation'
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AlbumType {
    Album,
    Single,
    AppearsOn,
    Compilation,
}

///  Type: 'artist', 'album','track', 'playlist', 'show' or 'episode'
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

/// additional_typs: track, episode
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AdditionalType {
    Track,
    Episode,
}

/// currently_playing_type: track, episode, ad, unknown.
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

/// Type for search: artist, album, track, playlist, show, episode
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

/// The user's Spotify subscription level: "premium", "free", etc. 
/// (The subscription level "open" can be considered the same as "free".)
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/)
#[derive(Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Debug, ToString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubscriptionLevel{
    Premium,
    Open,
    Free
}

/// Device Type: computer, smartphone, speaker, TV, etc.
/// See the [Spotify developer
/// docs](https://developer.spotify.com/documentation/web-api/reference/player/get-a-users-available-devices/#device-types)
/// for more information, or in case we are missing a device type here.
#[derive(Clone, Debug, Serialize, Deserialize, ToString)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_album_type_convert_from_str() {
        let appears_on = AlbumType::AppearsOn;
        assert_eq!("appears_on".to_string(), appears_on.to_string());
    }
    #[test]
    fn test_convert_search_type_from_str() {
        let search_type = SearchType::Artist;
        assert_eq!("artist".to_string(), search_type.to_string());
    }

    #[test]
    fn test_type_convert_from_str() {
        let artist = Type::Artist;
        assert_eq!(artist.to_string(), "artist".to_string());
    }
    #[test]
    fn test_additional_type() {
        let episode = AdditionalType::Episode;
        assert_eq!(episode.to_string(), "episode".to_string());
    }
    #[test]
    fn test_current_playing_type() {
        let ad = CurrentlyPlayingType::Advertisement;
        assert_eq!(ad.to_string(), "ad".to_string());
    }
    #[test]
    fn test_search_type() {
        let episode = SearchType::Episode;
        assert_eq!(episode.to_string(), "episode".to_string());
    }
}

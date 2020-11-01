//! All objects related to context
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::device::Device;
use super::track::FullTrack;
use super::PlayingItem;
use crate::model::{CurrentlyPlayingType, DisallowKey, RepeatState, Type};
/// Context object
/// [get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

/// [Get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimplifiedPlayingContext {
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}

/// [Currently playing object](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlayingContext {
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<PlayingItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}
/// [Currently Playback Context](https://developer.spotify.com/documentation/web-api/reference/player/get-information-about-the-users-current-playback/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<PlayingItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

/// [Actions](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct Actions {
    pub disallows: Vec<DisallowKey>,
}

impl<'de> Deserialize<'de> for Actions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct OriginalActions {
            pub disallows: HashMap<DisallowKey, bool>,
        }
        let orignal_actions = OriginalActions::deserialize(deserializer)?;
        Ok(Actions {
            disallows: orignal_actions
                .disallows
                .into_iter()
                .filter(|(_key, value)| *value == true)
                .map(|(key, _value)| key)
                .collect(),
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_actions() {
        let json_str = r#"
        {
            "disallows": {
                "resuming": true
            }
        }
        "#;
        let actions: Actions = serde_json::from_str(&json_str).unwrap();
        assert_eq!(actions.disallows[0], DisallowKey::Resuming);
    }
}

//! All objects related to context
use super::device::Device;
use super::track::FullTrack;
use super::PlayingItem;
use crate::model::{
    from_millisecond_timestamp, to_millisecond_timestamp, CurrentlyPlayingType, DisallowKey,
    RepeatState, Type,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
/// Context object
///
/// [Reference](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

/// Currently playing object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlayingContext {
    pub context: Option<Context>,
    #[serde(
        deserialize_with = "from_millisecond_timestamp",
        serialize_with = "to_millisecond_timestamp"
    )]
    pub timestamp: DateTime<Utc>,
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
    #[serde(
        deserialize_with = "from_millisecond_timestamp",
        serialize_with = "to_millisecond_timestamp"
    )]
    pub timestamp: DateTime<Utc>,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<PlayingItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

/// Actions object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/player/get-the-users-currently-playing-track/)
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
                .filter(|(_, value)| *value)
                .map(|(key, _)| key)
                .collect(),
        })
    }
}

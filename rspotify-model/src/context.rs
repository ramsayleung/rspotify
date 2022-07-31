//! All objects related to context

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use std::collections::HashMap;
use std::time::Duration;

use crate::{
    custom_serde::{millisecond_timestamp, option_duration_ms},
    CurrentlyPlayingType, Device, DisallowKey, PlayableItem, RepeatState, Type,
};

/// Context object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    /// The URI may be of any type, so it's not parsed into a [`crate::Id`]
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

/// Currently playing object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentlyPlayingContext {
    pub context: Option<Context>,
    #[serde(with = "millisecond_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(with = "option_duration_ms", rename = "progress_ms")]
    pub progress: Option<Duration>,
    pub is_playing: bool,
    pub item: Option<PlayableItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CurrentPlaybackContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    #[serde(with = "millisecond_timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(with = "option_duration_ms", rename = "progress_ms")]
    pub progress: Option<Duration>,
    pub is_playing: bool,
    pub item: Option<PlayableItem>,
    pub currently_playing_type: CurrentlyPlayingType,
    pub actions: Actions,
}

/// Actions object
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Default)]
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
        Ok(Self {
            disallows: orignal_actions
                .disallows
                .into_iter()
                .filter(|(_, value)| *value)
                .map(|(key, _)| key)
                .collect(),
        })
    }
}

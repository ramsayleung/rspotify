//! All objects related to context
use std::collections::HashMap;

use spotify::senum::{Type,RepeatState};
use super::device::Device;
use super::track::FullTrack;
/// Context object
///[get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

/// Full playing context
///[get information about the users current playback](https://developer.spotify.com/web-api/get-information-about-the-users-current-playback/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullPlayingContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}



///[get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedPlayingContext {
    pub context: Option<Context>,
    pub timestamp: u64,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}

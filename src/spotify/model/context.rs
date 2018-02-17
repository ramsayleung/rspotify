use std::collections::HashMap;

use spotify::spotify_enum::Type;
use super::device::Device;
use super::track::FullTrack;
///[get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

///[get information about the users current playback](https://developer.spotify.com/web-api/get-information-about-the-users-current-playback/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullPlayingContext {
    pub device: Device,
    pub repeat_state: RepeatState,
    pub shuffle_state: bool,
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}


#[derive(Clone, Debug,Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepeatState {
    Off,
    Track,
    Context,
}
impl RepeatState {
    pub fn from_str(s: &str) -> Option<RepeatState> {
        match s {
            "off" => Some(RepeatState::Off),
            "track" => Some(RepeatState::Track),
            "context" => Some(RepeatState::Context),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &str {
        match *self {
            RepeatState::Off => "off",
            RepeatState::Track => "track",
            RepeatState::Context => "context",
        }
    }
}


///[get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedPlayingContext {
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}

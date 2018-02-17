use chrono::prelude::*;

use super::context::Context;
use super::track::FullTrack;
use super::track::SimplifiedTrack;
///[get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playing {
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}
///[play history object](https://developer.spotify.com/web-api/object-model/#play-history-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayHistory {
    pub track: SimplifiedTrack,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}

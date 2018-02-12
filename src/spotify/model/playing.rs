use super::context::Context;
use super::track::FullTrack;
///https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playing {
    pub context: Option<Context>,
    pub timestamp: u32,
    pub progress_ms: Option<u32>,
    pub is_playing: bool,
    pub item: Option<FullTrack>,
}

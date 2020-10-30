//! All kinds of play object
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::context::Context;
use super::track::FullTrack;

/// Playing history
/// [play history object](https://developer.spotify.com/documentation/web-api/reference/object-model/#play-history-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayHistory {
    pub track: FullTrack,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}

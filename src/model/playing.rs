//! All kinds of play object
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use super::context::Context;
use super::track::FullTrack;

/// Playing history object
///
/// [Reference](https://developer.spotify.com/documentation/web-api/reference/#object-playhistoryobject)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayHistory {
    pub track: FullTrack,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}

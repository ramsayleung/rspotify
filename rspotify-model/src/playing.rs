//! All kinds of play object

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Context, FullTrack};

/// Playing history object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlayHistory {
    pub track: FullTrack,
    pub played_at: DateTime<Utc>,
    pub context: Option<Context>,
}

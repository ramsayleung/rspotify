//! The result of post/put/delete request
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CUDResult {
    pub snapshot_id: String,
}

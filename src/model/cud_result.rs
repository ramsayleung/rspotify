//! The result of post/put/delete request
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CUDResult {
    pub snapshot_id: String,
}

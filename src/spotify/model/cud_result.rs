//! the result of post/put/delete request  
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CUDResult {
    pub snapshot_id: String,
}

use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use super::user::PublicUser;
use spotify::spotify_enum::TYPE;
///https://developer.spotify.com/web-api/object-model/#playlist-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistSimplified {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: HashMap<String, Value>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

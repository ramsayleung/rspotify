use serde_json::Value;

use std::collections::HashMap;

use super::image::Image;
use spotify::spotify_enum::TYPE;
///https://developer.spotify.com/web-api/object-model/#user-object-public
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub display_name: String,
    pub external_urls: HashMap<String, String>,
    pub followers: HashMap<String, Option<Value>>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

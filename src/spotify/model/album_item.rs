use serde_json;

use std::collections::HashMap;

use super::artist::Artist;
use super::image::Image;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

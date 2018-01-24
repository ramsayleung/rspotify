use std::collections::HashMap;

use super::artist::Artist;
use super::image::Image;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Albums {
    pub href: String,
    pub items: Vec<Album>,
    pub limit: u16,
    pub next: String,
    pub offset: i32,
    pub previous: Option<String>,
    pub total: u32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub artists: Vec<Artist>,
    pub album_type: String,
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

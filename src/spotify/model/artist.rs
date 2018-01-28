
use std::collections::HashMap;
use serde_json::Value;
use spotify::spotify_enum::TYPE;
use super::image::Image;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistSimplified {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}


#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct ArtistFull {
    pub external_urls: HashMap<String, String>,
    pub followers: HashMap<String, Option<Value>>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistFulls {
    pub artists: Vec<ArtistFull>,
}

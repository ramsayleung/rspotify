use super::image::Image;
use std::collections::HashMap;

/// Show object(simplified)
/// [Show object simplified](https://developer.spotify.com/documentation/web-api/reference/object-model/#show-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedShow {
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub description: String,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub is_externally_hosted: Option<bool>,
    pub languages: Vec<String>,
    pub media_type: String,
    pub name: String,
    pub publisher: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Show {
    pub added_at: String,
    pub show: SimplifiedShow,
}

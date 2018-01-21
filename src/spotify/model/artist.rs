use serde_json;

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub uri: String,
}

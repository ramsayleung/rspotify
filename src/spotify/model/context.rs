use std::collections::HashMap;
use spotify::spotify_enum::Type;

///https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub uri: String,
    pub href: String,
    pub external_urls: HashMap<String, String>,
    #[serde(rename = "type")]
    pub _type: Type,
}

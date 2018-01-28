
use std::collections::HashMap;

use super::artist::ArtistSimplified;
use super::album::AlbumSimplified;
use spotify::spotify_enum::TYPE;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackFull {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackFulls {
    pub tracks: Vec<TrackFull>,
}

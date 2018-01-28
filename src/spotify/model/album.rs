use std::collections::HashMap;

use spotify::spotify_enum::{TYPE, ALBUM_TYPE};
use super::track::TrackSimplified;
use super::artist::ArtistSimplified;
use super::image::Image;
use super::page::Page;
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumSimplified {
    pub artists: Vec<ArtistSimplified>,
    pub album_type: String,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumFull {
    pub artists: Vec<ArtistSimplified>,
    pub album_type: ALBUM_TYPE,
    pub available_markets: Vec<String>,
    pub copyrights: Vec<HashMap<String, String>>,
    pub external_ids: HashMap<String, String>,
    pub external_urls: HashMap<String, String>,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: u32,
    pub release_date: String,
    pub release_date_precision: String,
    pub tracks: Page<TrackSimplified>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumFulls {
    pub albums: Vec<AlbumFull>,
}

use std::collections::HashMap;

use spotify::spotify_enum::{TYPE, ALBUM_TYPE};
use super::track::SimplifiedTrack;
use super::artist::SimplifiedArtist;
use super::image::Image;
use super::page::Page;
///https://developer.spotify.com/web-api/object-model/#album-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedAlbum {
    pub artists: Vec<SimplifiedArtist>,
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
///https://developer.spotify.com/web-api/object-model/#album-object-full
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbum {
    pub artists: Vec<SimplifiedArtist>,
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
    pub tracks: Page<SimplifiedTrack>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAlbums {
    pub albums: Vec<FullAlbum>,
}

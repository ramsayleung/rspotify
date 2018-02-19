//! All kinds of tracks object
use chrono::prelude::*;

use std::collections::HashMap;

use super::artist::SimplifiedArtist;
use super::album::SimplifiedAlbum;
use spotify::senum::Type;
///[track object full](https://developer.spotify.com/web-api/object-model/#track-object-full)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullTrack {
    pub album: SimplifiedAlbum,
    pub artists: Vec<SimplifiedArtist>,
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
    pub _type: Type,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullTracks {
    pub tracks: Vec<FullTrack>,
}
///[track object simplified](https://developer.spotify.com/web-api/object-model/#track-object-simplified)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedTrack {
    pub artists: Vec<SimplifiedArtist>,
    pub available_markets: Option<Vec<String>>,
    pub disc_number: i32,
    pub duration_ms: u32,
    pub explicit: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    pub preview_url: Option<String>,
    pub track_number: u32,
    #[serde(rename = "type")]
    pub _type: Type,
    pub uri: String,
}

///[saved track object](https://developer.spotify.com/web-api/object-model/#saved-track-object)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SavedTrack {
    pub added_at: DateTime<Utc>,
    pub track: FullTrack,
}

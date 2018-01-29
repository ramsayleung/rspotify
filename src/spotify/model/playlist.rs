use serde_json::Value;
use chrono::NaiveDateTime;

use std::collections::HashMap;

use super::image::Image;
use super::user::PublicUser;
use super::track::FullTrack;
use super::page::Page;
use spotify::spotify_enum::TYPE;
///https://developer.spotify.com/web-api/object-model/#playlist-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimplifiedPlaylist {
    pub collaborative: bool,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: HashMap<String, Value>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullPlaylist {
    pub collaborative: bool,
    pub description: String,
    pub external_urls: HashMap<String, String>,
    pub followers: Option<HashMap<String, Value>>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub owner: PublicUser,
    pub public: Option<bool>,
    pub snapshot_id: String,
    pub tracks: Page<PlaylistTrack>,
    #[serde(rename = "type")]
    pub _type: TYPE,
    pub uri: String,
}

///https://developer.spotify.com/web-api/object-model/#playlist-track-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub added_at: NaiveDateTime,
    pub added_by: PublicUser,
    pub is_local: bool,
    pub track: FullTrack,
}

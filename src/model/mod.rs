//! All Spotify API endpoint response object
pub mod album;
pub mod artist;
pub mod audio;
pub mod category;
pub mod context;
pub mod cud_result;
pub mod device;
pub mod enums;
pub mod offset;
pub mod page;
pub mod playing;
pub mod playlist;
pub mod recommend;
pub mod search;
pub mod show;
pub mod track;
pub mod user;

use serde::{Deserialize, Serialize};
use RestrictionReason;

/// [image object](https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image {
    pub height: Option<u32>,
    pub url: String,
    pub width: Option<u32>,
}

/// [track restriction object](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-restriction-object)
/// [album restriction object](https://developer.spotify.com/documentation/web-api/reference/object-model/#album-restriction-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

pub use {
    album::*, artist::*, audio::*, category::*, context::*, cud_result::*, device::*, enums::*,
    offset::*, page::*, playing::*, playlist::*, recommend::*, search::*, show::*, track::*,
    user::*,
};

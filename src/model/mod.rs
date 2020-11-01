//! All Spotify API endpoint response object
pub mod album;
pub mod artist;
pub mod audio;
pub mod category;
pub mod context;
pub mod cud_result;
pub mod device;
pub mod enums;
pub mod image;
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

/// [track restriction object](https://developer.spotify.com/documentation/web-api/reference/object-model/#track-restriction-object)
/// [album restriction object](https://developer.spotify.com/documentation/web-api/reference/object-model/#album-restriction-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Restriction {
    pub reason: RestrictionReason,
}

/// [Followers object](https://developer.spotify.com/documentation/web-api/reference/object-model/#followers-object)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Followers {
    // This field will always set to null, as the Web API does not support it at the moment.
    // pub href: Option<String>,
    pub total: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

pub use {
    album::*, artist::*, audio::*, category::*, context::*, cud_result::*, device::*, enums::*,
    image::*, offset::*, page::*, playing::*, playlist::*, recommend::*, search::*, show::*,
    track::*, user::*,
};

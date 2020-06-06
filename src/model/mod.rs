//! All Spotify API endpoint response object
pub mod album;
pub mod artist;
pub mod audio;
pub mod category;
pub mod context;
pub mod cud_result;
pub mod device;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayingItem {
    Track(track::FullTrack),
    Episode(show::FullEpisode),
}

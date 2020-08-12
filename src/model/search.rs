//! All object related to search
use super::album::SimplifiedAlbum;
use super::artist::FullArtist;
use super::page::Page;
use super::playlist::SimplifiedPlaylist;
use super::show::{SimplifiedEpisode, SimplifiedShow};
use super::track::FullTrack;
use serde::{Deserialize, Serialize};

///[Search item](https://developer.spotify.com/web-api/search-item/);
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}
///[Search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

///[Search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchArtists {
    pub artists: Page<FullArtist>,
}
///[Search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTracks {
    pub tracks: Page<FullTrack>,
}

/// [Search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchShows {
    pub shows: Page<SimplifiedShow>,
}
/// [Search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchEpisodes {
    pub episodes: Page<SimplifiedEpisode>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SearchResult {
    #[serde(rename = "playlists")]
    Playlists(Page<SimplifiedPlaylist>),
    #[serde(rename = "albums")]
    Albums(Page<SimplifiedAlbum>),
    #[serde(rename = "artists")]
    Artists(Page<FullArtist>),
    #[serde(rename = "tracks")]
    Tracks(Page<FullTrack>),
    #[serde(rename = "shows")]
    Shows(Page<SimplifiedShow>),
    #[serde(rename = "episodes")]
    Episodes(Page<SimplifiedEpisode>),
}

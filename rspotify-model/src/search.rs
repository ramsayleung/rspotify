//! All object related to search

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::{
    FullArtist, FullTrack, Page, SimplifiedAlbum, SimplifiedEpisode, SimplifiedPlaylist,
    SimplifiedShow,
};

/// Search for playlists
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}

/// Search for albums
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct SearchAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

/// Search for artists
///
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchArtists {
    pub artists: Page<FullArtist>,
}

/// Search item
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchTracks {
    pub tracks: Page<FullTrack>,
}

/// Search for shows
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchShows {
    pub shows: Page<SimplifiedShow>,
}

/// Search for episodes
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchEpisodes {
    pub episodes: Page<SimplifiedEpisode>,
}

/// Search result of any kind
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
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

/// Valid filters to used in the search endpoint's query string
#[derive(Debug, Display, PartialEq, Eq, PartialOrd, Ord)]
#[strum(serialize_all = "snake_case")]
pub enum SearchFilter {
    Album,
    Artist,
    Track,
    Year,
    Upc,
    #[strum(serialize = "tag:hipster")]
    TagHipster,
    #[strum(serialize = "tag:new")]
    TagNew,
    Isrc,
    Genre,
}

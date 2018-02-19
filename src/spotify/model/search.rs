//! All object related to search 
use super::page::Page;
use super::album::SimplifiedAlbum;
use super::artist::FullArtist;
use super::track::FullTrack;
use super::playlist::SimplifiedPlaylist;
///[search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlaylists {
    pub playlists: Page<SimplifiedPlaylist>,
}
///[search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchAlbums {
    pub albums: Page<SimplifiedAlbum>,
}

///[search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchArtists {
    pub artists: Page<FullArtist>,
}
///[search item](https://developer.spotify.com/web-api/search-item/)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTracks {
    pub tracks: Page<FullTrack>,
}

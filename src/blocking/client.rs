//! Client to Spotify API endpoint
// 3rd-part library
use derive_deref::Deref;
use lazy_static::lazy_static;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde::Deserialize;
use serde_json::Value;
use tokio::runtime::Runtime;

//  Built-in battery
use std::collections::HashMap;
use std::fmt;
use std::io::Read;

use crate::blocking::oauth2::SpotifyClientCredentials;
use crate::blocking::util::convert_map_to_string;
use crate::client::Spotify as AsyncSpotify;

pub use crate::client::ApiError;

/// Spotify API object
#[derive(Debug, Clone, Deref)]
pub struct Spotify(AsyncSpotify);

// The endpoints will be added from the main client.rs with a macro.
impl Spotify {
    pub fn default() -> Self {
        Spotify(AsyncSpotify::default())
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        self.0 = self.0.prefix(prefix);
        self
    }

    pub fn access_token(mut self, access_token: &str) -> Self {
        self.0 = self.0.access_token(access_token);
        self
    }

    pub fn client_credentials_manager(
        mut self,
        client_credential_manager: SpotifyClientCredentials,
    ) -> Self {
        self.0 = self
            .0
            .client_credentials_manager(client_credential_manager.0);
        self
    }

    pub fn build(self) -> Self {
        Spotify(self.0.build())
    }

    /// Append device ID to API path.
    fn append_device_id(&self, path: &str, device_id: Option<String>) -> String {
        self.0.append_device_id(path, device_id)
    }

    fn get_uri(&self, _type: Type, _id: &str) -> String {
        self.0.get_uri(_type, _id)
    }

    /// Get spotify id by type and id
    fn get_id(&self, _type: Type, id: &str) -> String {
        self.0.get_id(_type, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_id() {
        // Assert artist
        let spotify = Spotify::default().access_token("test-access").build();
        let mut artist_id = String::from("spotify:artist:2WX2uTcsvV5OnS0inACecP");
        let id = spotify.get_id(Type::Artist, &mut artist_id);
        assert_eq!("2WX2uTcsvV5OnS0inACecP", &id);
        // Assert album
        let mut artist_id_a = String::from("spotify/album/2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
            "2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Album, &mut artist_id_a)
        );

        // Mismatch type
        let mut artist_id_b = String::from("spotify:album:2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
            "spotify:album:2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Artist, &mut artist_id_b)
        );

        // Could not split
        let mut artist_id_c = String::from("spotify-album-2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
            "spotify-album-2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Artist, &mut artist_id_c)
        );

        let mut playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
        assert_eq!(
            "59ZbFPES4DQwEjBpWHzrtC",
            &spotify.get_id(Type::Playlist, &mut playlist_id)
        );
    }
    #[test]
    fn test_get_uri() {
        let spotify = Spotify::default().access_token("test-access").build();
        let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
        let track_id2 = "1301WleyT98MSxVHPZCA6M";
        let uri1 = spotify.get_uri(Type::Track, track_id1);
        let uri2 = spotify.get_uri(Type::Track, track_id2);
        assert_eq!(track_id1, uri1);
        assert_eq!("spotify:track:1301WleyT98MSxVHPZCA6M", &uri2);
    }
}

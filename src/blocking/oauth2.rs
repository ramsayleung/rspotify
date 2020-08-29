//! The module contains function about authorization and client-credential
// use 3rd party library
use chrono::prelude::*;
use derive_deref::{Deref, DerefMut};
use log::{debug, error, trace};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;

// Use built-in library
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::ops::Deref;
use std::path::{Path, PathBuf};

// Use customized library
use crate::blocking::RT;
use crate::oauth2::SpotifyClientCredentials as AsyncSpotifyClientCredentials;
use crate::oauth2::SpotifyOAuth as AsyncSpotifyOAuth;
use crate::util::{convert_map_to_string, datetime_to_timestamp, generate_random_string};

pub use crate::oauth2::TokenInfo;

/// Client credentials object for spotify
#[derive(Debug, Clone, Serialize, Deserialize, Deref, DerefMut)]
pub struct SpotifyClientCredentials(pub(in crate) AsyncSpotifyClientCredentials);

/// Authorization for spotify
#[derive(Clone, Debug, Serialize, Deserialize, Deref, DerefMut)]
pub struct SpotifyOAuth(pub(in crate) AsyncSpotifyOAuth);

// The methods that return Self have to be re-implemented so that Deref
// won't return the underlying type instead of this wrapper.
impl SpotifyClientCredentials {
    /// Build default SpotifyClientCredentials
    pub fn default() -> Self {
        SpotifyClientCredentials(AsyncSpotifyClientCredentials::default())
    }

    pub fn client_id(mut self, client_id: &str) -> Self {
        self.0 = self.0.client_id(client_id);
        self
    }

    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.0 = self.0.client_secret(client_secret);
        self
    }

    pub fn token_info(mut self, token_info: TokenInfo) -> Self {
        self.0 = self.0.token_info(token_info);
        self
    }

    pub fn build(self) -> Self {
        SpotifyClientCredentials(self.0.build())
    }

    /// Get access token from self.token_info, if self.token_info is none or is
    /// expired. fetch token info by HTTP request
    pub fn get_access_token(&self) -> String {
        RT.handle()
            .block_on(async move { self.0.get_access_token().await })
    }
}

impl SpotifyOAuth {
    pub fn default() -> Self {
        SpotifyOAuth(AsyncSpotifyOAuth::default())
    }

    pub fn client_id(mut self, client_id: &str) -> Self {
        self.0 = self.0.client_id(client_id);
        self
    }

    pub fn client_secret(mut self, client_secret: &str) -> Self {
        self.0 = self.0.client_secret(client_secret);
        self
    }

    pub fn redirect_uri(mut self, redirect_uri: &str) -> Self {
        self.0 = self.0.redirect_uri(redirect_uri);
        self
    }

    pub fn scope(mut self, scope: &str) -> Self {
        self.0 = self.0.scope(scope);
        self
    }

    pub fn state(mut self, state: &str) -> Self {
        self.0 = self.0.state(state);
        self
    }

    pub fn cache_path(mut self, cache_path: PathBuf) -> Self {
        self.0 = self.0.cache_path(cache_path);
        self
    }

    pub fn proxies(mut self, proxies: &str) -> Self {
        self.0 = self.0.proxies(proxies);
        self
    }

    pub fn build(self) -> Self {
        SpotifyOAuth(self.0.build())
    }

    pub fn get_cached_token(&mut self) -> Option<TokenInfo> {
        RT.handle()
            .block_on(async move { self.0.get_cached_token().await })
    }

    /// Gets the access_token for the app with given the code without caching token.
    pub fn get_access_token_without_cache(&self, code: &str) -> Option<TokenInfo> {
        RT.handle()
            .block_on(async move { self.0.get_access_token_without_cache(code).await })
    }

    /// Gets the access_token for the app with given the code
    pub fn get_access_token(&self, code: &str) -> Option<TokenInfo> {
        RT.handle()
            .block_on(async move { self.0.get_access_token(code).await })
    }

    /// Refresh token without caching token.
    pub fn refresh_access_token_without_cache(&self, refresh_token: &str) -> Option<TokenInfo> {
        RT.handle().block_on(async move {
            self.0
                .refresh_access_token_without_cache(refresh_token)
                .await
        })
    }

    /// After refresh access_token, the response may be empty
    /// when refresh_token again
    pub fn refresh_access_token(&self, refresh_token: &str) -> Option<TokenInfo> {
        RT.handle().block_on(async move {
            self.0
                .refresh_access_token_without_cache(refresh_token)
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::path::PathBuf;
    #[test]
    fn test_is_scope_subset() {
        let mut needle_scope = String::from("1 2 3");
        let mut haystack_scope = String::from("1 2 3 4");
        let mut broken_scope = String::from("5 2 4");
        assert!(SpotifyOAuth::is_scope_subset(
            &mut needle_scope,
            &mut haystack_scope
        ));
        assert!(!SpotifyOAuth::is_scope_subset(
            &mut broken_scope,
            &mut haystack_scope
        ));
    }
    #[test]
    fn test_save_token_info() {
        let spotify_oauth = SpotifyOAuth::default()
            .state(&generate_random_string(16))
            .scope("playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private streaming ugc-image-upload user-follow-modify user-follow-read user-library-read user-library-modify user-read-private user-read-birthdate user-read-email user-top-read user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played")
            .cache_path(PathBuf::from(".spotify_token_cache.json"))
            .build();
        let token_info = TokenInfo::default()
            .access_token("test-access_token")
            .token_type("code")
            .expires_in(3600)
            .expires_at(1515841743)
            .scope("playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private streaming ugc-image-upload user-follow-modify user-follow-read user-library-read user-library-modify user-read-private user-read-birthdate user-read-email user-top-read user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played")
            .refresh_token("fghjklrftyhujkuiovbnm");
        match serde_json::to_string(&token_info) {
            Ok(token_info_string) => {
                spotify_oauth.save_token_info(&token_info_string);
                let display = spotify_oauth.cache_path.display();
                let mut file = match File::open(&spotify_oauth.cache_path) {
                    Err(why) => panic!("couldn't open {}: {}", display, why.to_string()),
                    Ok(file) => file,
                };
                let mut token_info_string_from_file = String::new();
                match file.read_to_string(&mut token_info_string_from_file) {
                    Err(why) => panic!("couldn't read {}: {}", display, why.to_string()),
                    Ok(_) => {
                        assert_eq!(token_info_string, token_info_string_from_file);
                    }
                }
            }
            Err(why) => panic!(
                "couldn't convert token_info to string: {} ",
                why.to_string()
            ),
        }
    }

    #[test]
    fn test_parse_response_code() {
        let mut url = String::from("http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN#_=_");
        let spotify_oauth = SpotifyOAuth::default()
            .state(&generate_random_string(16))
            .scope("playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private streaming ugc-image-upload user-follow-modify user-follow-read user-library-read user-library-modify user-read-private user-read-birthdate user-read-email user-top-read user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played")
            .cache_path(PathBuf::from(".spotify_token_cache.json"))
            .build();
        match spotify_oauth.parse_response_code(&mut url) {
            Some(code) => assert_eq!(code, "AQD0yXvFEOvw"),
            None => println!("failed"),
        }
    }
}

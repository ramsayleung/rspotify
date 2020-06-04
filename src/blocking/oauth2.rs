//! The module contains function about authorization and client-credential
// use 3rd party library
use chrono::prelude::*;
use dotenv::dotenv;
use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use reqwest::blocking::Client;
use serde_json;

// use built-in library
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

// use customized library
use super::util::{convert_map_to_string, datetime_to_timestamp, generate_random_string};

/// Client credentials object for spotify
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub token_info: Option<TokenInfo>,
}
/// Authorization for spotify
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpotifyOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub state: String,
    pub cache_path: PathBuf,
    pub scope: String,
    pub proxies: Option<String>,
}

/// Spotify token-info
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: String,
}
impl TokenInfo {
    pub fn default() -> TokenInfo {
        TokenInfo {
            access_token: String::new(),
            token_type: String::new(),
            expires_in: 0u32,
            expires_at: None,
            refresh_token: None,
            scope: String::new(),
        }
    }
    pub fn access_token(mut self, access_token: &str) -> TokenInfo {
        self.access_token = access_token.to_owned();
        self
    }
    pub fn token_type(mut self, token_type: &str) -> TokenInfo {
        self.token_type = token_type.to_owned();
        self
    }
    pub fn expires_in(mut self, expires_in: u32) -> TokenInfo {
        self.expires_in = expires_in;
        self
    }
    pub fn scope(mut self, scope: &str) -> TokenInfo {
        self.scope = scope.to_owned();
        self
    }
    pub fn expires_at(mut self, expires_at: i64) -> TokenInfo {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn refresh_token(mut self, refresh_token: &str) -> TokenInfo {
        self.refresh_token = Some(refresh_token.to_owned());
        self
    }
    pub fn set_expires_at(&mut self, expires_at: i64) {
        self.expires_at = Some(expires_at);
    }
    pub fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = Some(refresh_token.to_owned());
    }
}

impl SpotifyClientCredentials {
    /// build default SpotifyClientCredentials
    pub fn default() -> SpotifyClientCredentials {
        dotenv().ok();
        let client_id = env::var("CLIENT_ID").unwrap_or_default();
        let client_secret = env::var("CLIENT_SECRET").unwrap_or_default();
        trace!(
            "SpotifyClientCredentials.default(): client_id:{:?}, client_secret:{:?}",
            client_id,
            client_secret
        );
        SpotifyClientCredentials {
            client_id,
            client_secret,
            token_info: None,
        }
    }
    pub fn client_id(mut self, client_id: &str) -> SpotifyClientCredentials {
        self.client_id = client_id.to_owned();
        self
    }
    pub fn client_secret(mut self, client_secret: &str) -> SpotifyClientCredentials {
        self.client_secret = client_secret.to_owned();
        self
    }
    pub fn token_info(mut self, token_info: TokenInfo) -> SpotifyClientCredentials {
        self.token_info = Some(token_info);
        self
    }
    pub fn build(self) -> SpotifyClientCredentials {
        const ERROR_MESSAGE: &str = "
    You need to set your Spotify API credentials. You can do this by
    setting environment variables in `.env` file:
    CLIENT_ID='your-spotify-client-id'
    CLIENT_SECRET='your-spotify-client-secret'
    REDIRECT_URI='your-app-redirect-url'
    Get your credentials at `https://developer.spotify.com/my-applications`";
        trace!("SpotifyClientCredentials.default(): client_id:{:?}, client_secret:{:?} empty_flag:{:?}",self.client_id, self.client_secret, !(self.client_id.is_empty()||self.client_secret.is_empty())&&self.token_info.is_none());
        let empty_flag = (self.client_id.is_empty() || self.client_secret.is_empty())
            && self.token_info.is_none();
        if empty_flag {
            error!("{}", ERROR_MESSAGE);
        } else {
            debug!(
                "client_id:{:?}, client_secret:{:?}",
                self.client_id, self.client_secret
            );
        }
        self
    }
    /// get access token from self.token_info, if self.token_info is none or is
    /// expired. fetch token info by HTTP request
    pub fn get_access_token(&self) -> String {
        let access_token = match self.token_info {
            Some(ref token_info) => {
                if !self.is_token_expired(token_info) {
                    debug!("token info: {:?}", &token_info);
                    Some(&token_info.access_token)
                } else {
                    None
                }
            }
            None => None,
        };
        match access_token {
            Some(access_token) => access_token.to_owned(),
            None => match self.request_access_token() {
                Some(new_token_info) => {
                    debug!("token info: {:?}", &new_token_info);
                    new_token_info.access_token
                }
                None => String::new(),
            },
        }
    }
    fn is_token_expired(&self, token_info: &TokenInfo) -> bool {
        is_token_expired(token_info)
    }
    fn request_access_token(&self) -> Option<TokenInfo> {
        let mut payload = HashMap::new();
        payload.insert("grant_type", "client_credentials");
        if let Some(mut token_info) =
            self.fetch_access_token(&self.client_id, &self.client_secret, &payload)
        {
            let expires_in = token_info.expires_in;
            token_info.set_expires_at(datetime_to_timestamp(expires_in));
            Some(token_info)
        } else {
            None
        }
    }
    fn fetch_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        payload: &HashMap<&str, &str>,
    ) -> Option<TokenInfo> {
        fetch_access_token(client_id, client_secret, payload)
    }
}

impl SpotifyOAuth {
    // spotify token example:
    // {
    //    "access_token": "NgCXRK...MzYjw",
    //    "token_type": "Bearer",
    //    "scope": "user-read-private user-read-email",
    //    "expires_in": 3600,
    //    "refresh_token": "NgAagA...Um_SHo"
    // }

    pub fn default() -> SpotifyOAuth {
        dotenv().ok();
        let client_id = env::var("CLIENT_ID").unwrap_or_default();
        let client_secret = env::var("CLIENT_SECRET").unwrap_or_default();
        let redirect_uri = env::var("REDIRECT_URI").unwrap_or_default();
        SpotifyOAuth {
            client_id,
            client_secret,
            redirect_uri,
            state: generate_random_string(16),
            scope: String::new(),
            cache_path: PathBuf::from(".spotify_token_cache.json"),
            proxies: None,
        }
    }
    pub fn client_id(mut self, client_id: &str) -> SpotifyOAuth {
        self.client_id = client_id.to_owned();
        self
    }
    pub fn client_secret(mut self, client_secret: &str) -> SpotifyOAuth {
        self.client_secret = client_secret.to_owned();
        self
    }
    pub fn redirect_uri(mut self, redirect_uri: &str) -> SpotifyOAuth {
        self.redirect_uri = redirect_uri.to_owned();
        self
    }
    pub fn scope(mut self, scope: &str) -> SpotifyOAuth {
        self.scope = scope.to_owned();
        self
    }
    pub fn state(mut self, state: &str) -> SpotifyOAuth {
        self.state = state.to_owned();
        self
    }
    pub fn cache_path(mut self, cache_path: PathBuf) -> SpotifyOAuth {
        self.cache_path = cache_path;
        self
    }
    pub fn proxies(mut self, proxies: &str) -> SpotifyOAuth {
        self.proxies = Some(proxies.to_owned());
        self
    }
    pub fn build(self) -> SpotifyOAuth {
        const ERROR_MESSAGE: &str = "
    You need to set your Spotify API credentials. You can do this by
    setting environment variables in `.env` file:
    CLIENT_ID='your-spotify-client-id'
    CLIENT_SECRET='your-spotify-client-secret'
    REDIRECT_URI='your-app-redirect-url'
    Get your credentials at `https://developer.spotify.com/my-applications`";
        let empty_flag = self.redirect_uri.is_empty()
            || self.client_id.is_empty()
            || self.client_secret.is_empty();

        if empty_flag {
            error!("{}", ERROR_MESSAGE);
        } else {
            trace!(
                "client_id:{:?}, client_secret:{:?}, redirect_uri:{:?}",
                self.client_id,
                self.client_secret,
                self.redirect_uri
            );
        }
        self
    }
    pub fn get_cached_token(&mut self) -> Option<TokenInfo> {
        let display = self.cache_path.display();
        let mut file = match File::open(&self.cache_path) {
            Ok(file) => file,
            Err(why) => {
                error!("couldn't open {}: {:?}", display, why.to_string());
                return None;
            }
        };
        let mut token_info_string = String::new();
        match file.read_to_string(&mut token_info_string) {
            Err(why) => {
                error!("couldn't read {}: {}", display, why.to_string());
                None
            }
            Ok(_) => {
                let mut token_info: TokenInfo = serde_json::from_str(&token_info_string)
                    .unwrap_or_else(|_| {
                        panic!("convert [{:?}] to json failed", self.cache_path.display())
                    });
                if !SpotifyOAuth::is_scope_subset(&mut self.scope, &mut token_info.scope) {
                    None
                } else if self.is_token_expired(&token_info) {
                    if let Some(refresh_token) = token_info.refresh_token {
                        self.refresh_access_token(&refresh_token)
                    } else {
                        None
                    }
                } else {
                    Some(token_info)
                }
            }
        }
    }
    /// gets the access_token for the app with given the code without caching token.

    pub fn get_access_token_without_cache(&self, code: &str) -> Option<TokenInfo> {
        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert("redirect_uri", &self.redirect_uri);
        payload.insert("code", code);
        payload.insert("grant_type", "authorization_code");
        payload.insert("scope", &self.scope);
        payload.insert("state", &self.state);
        return self.fetch_access_token(&self.client_id, &self.client_secret, &payload);
    }
    /// gets the access_token for the app with given the code
    pub fn get_access_token(&self, code: &str) -> Option<TokenInfo> {
        if let Some(token_info) = self.get_access_token_without_cache(code) {
            match serde_json::to_string(&token_info) {
                Ok(token_info_string) => {
                    trace!("get_access_token->token_info[{:?}]", &token_info_string);
                    self.save_token_info(&token_info_string);
                    Some(token_info)
                }
                Err(why) => {
                    panic!(
                        "couldn't convert token_info to string: {} ",
                        why.to_string()
                    );
                }
            }
        } else {
            None
        }
    }
    /// fetch access_token
    fn fetch_access_token(
        &self,
        client_id: &str,
        client_secret: &str,
        payload: &HashMap<&str, &str>,
    ) -> Option<TokenInfo> {
        trace!("fetch_access_token->payload {:?}", &payload);
        fetch_access_token(client_id, client_secret, payload)
    }
    /// Parse the response code in the given response url
    pub fn parse_response_code(&self, url: &mut str) -> Option<String> {
        url.split("?code=")
            .nth(1)
            .and_then(|strs| strs.split('&').next())
            .map(|s| s.to_owned())
    }
    /// Gets the URL to use to authorize this app
    pub fn get_authorize_url(&self, state: Option<&str>, show_dialog: Option<bool>) -> String {
        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert("client_id", &self.client_id);
        payload.insert("response_type", "code");
        payload.insert("redirect_uri", &self.redirect_uri);
        payload.insert("scope", &self.scope);
        if let Some(state) = state {
            payload.insert("state", state);
        } else {
            payload.insert("state", &self.state);
        }
        if let Some(show_dialog) = show_dialog {
            if show_dialog {
                payload.insert("show_dialog", "true");
            }
        }

        let query_str = convert_map_to_string(&payload);
        let mut authorize_url = String::from("https://accounts.spotify.com/authorize?");
        authorize_url
            .push_str(&utf8_percent_encode(&query_str, PATH_SEGMENT_ENCODE_SET).to_string());
        trace!("{:?}", &authorize_url);
        authorize_url
    }

    /// refresh token without caching token.
    pub fn refresh_access_token_without_cache(&self, refresh_token: &str) -> Option<TokenInfo> {
        let mut payload = HashMap::new();
        payload.insert("refresh_token", refresh_token);
        payload.insert("grant_type", "refresh_token");
        return self.fetch_access_token(&self.client_id, &self.client_secret, &payload);
    }

    /// after refresh access_token, the response may be empty
    /// when refresh_token again
    pub fn refresh_access_token(&self, refresh_token: &str) -> Option<TokenInfo> {
        if let Some(token_info) = self.refresh_access_token_without_cache(refresh_token) {
            match serde_json::to_string(&token_info) {
                Ok(token_info_string) => {
                    self.save_token_info(&token_info_string);
                    Some(token_info)
                }
                Err(why) => {
                    panic!(
                        "couldn't convert token_info to string: {} ",
                        why.to_string()
                    );
                }
            }
        } else {
            None
        }
    }
    fn save_token_info(&self, token_info: &str) {
        save_token_info(token_info, self.cache_path.as_path())
    }
    fn is_scope_subset(needle_scope: &mut str, haystack_scope: &mut str) -> bool {
        let needle_vec: Vec<&str> = needle_scope.split_whitespace().collect();
        let haystack_vec: Vec<&str> = haystack_scope.split_whitespace().collect();
        let needle_set: HashSet<&str> = HashSet::from_iter(needle_vec);
        let haystack_set: HashSet<&str> = HashSet::from_iter(haystack_vec);
        // needle_set - haystack_set
        needle_set.is_subset(&haystack_set)
    }
    fn is_token_expired(&self, token_info: &TokenInfo) -> bool {
        is_token_expired(token_info)
    }
}

fn is_token_expired(token_info: &TokenInfo) -> bool {
    let now: DateTime<Utc> = Utc::now();
    // 10s as buffer time
    match token_info.expires_at {
        Some(expires_at) => now.timestamp() > expires_at - 10,
        None => true,
    }
}
fn save_token_info(token_info: &str, path: &Path) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap_or_else(|_| panic!("create file {:?} error", path.display()));
    file.set_len(0).unwrap_or_else(|_| {
        panic!(
            "clear original spoitfy-token-cache file [{:?}] failed",
            path.display()
        )
    });
    file.write_all(token_info.as_bytes())
        .expect("error when write file");
}

fn fetch_access_token(
    _client_id: &str,
    _client_secret: &str,
    payload: &HashMap<&str, &str>,
) -> Option<TokenInfo> {
    let client = Client::new();
    let client_id = _client_id.to_owned();
    let client_secret = _client_secret.to_owned();
    let url = "https://accounts.spotify.com/api/token";
    let mut response = client
        .post(url)
        .basic_auth(client_id, Some(client_secret))
        .form(&payload)
        .send()
        .expect("send request failed");
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("failed to read response");
    if response.status().is_success() {
        debug!("response content: {:?}", buf);
        let mut token_info: TokenInfo = serde_json::from_str(&buf).unwrap();
        // .expect("parsing response content to tokenInfo error");
        let expires_in = token_info.expires_in;
        token_info.set_expires_at(datetime_to_timestamp(expires_in));
        if token_info.refresh_token.is_none() {
            match payload.get("refresh_token") {
                Some(payload_refresh_token) => {
                    token_info.set_refresh_token(payload_refresh_token);
                    return Some(token_info);
                }
                None => {
                    debug!("could not find refresh_token");
                }
            }
        }
        Some(token_info)
    } else {
        error!("fetch access token request failed, payload:{:?}", &payload);
        error!("{:?}", response);
        None
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

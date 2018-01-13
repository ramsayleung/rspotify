use chrono::prelude::*;
use serde_json;
use reqwest::Client;
use reqwest::header::{Authorization, Basic, Bearer};

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::File;
use std::iter::FromIterator;
use std::io::prelude::*;
use std::fs::OpenOptions;

use super::util::{datetime_to_timestamp, generate_random_string};
pub struct SpotifyClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub token_info: TokenInfo,
}
#[derive(Builder)]
#[builder(setter(into))]
pub struct SpotifyOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub state: String,
    pub cache_path: PathBuf,
    pub scope: String,
    pub proxies: Option<String>,
}
#[derive(Debug)]
pub enum SpotifyAuthError {
    AuthorizateError,
    FileHandleError,
}

static CLIENT_ID: &'static str = "3a205160926f4b719170b1ad97c2ad01";
static CLIENT_SECRET: &'static str = "1449bf2c59164f2b97f21322362fe4cd";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";

#[derive(Clone,Debug,Serialize,Deserialize,Builder)]
#[builder(setter(into))]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: Option<i64>,
    pub refresh_token: String,
    pub scope: String,
}
impl TokenInfo {
    pub fn set_expires_at(&mut self, expires_at: &i64) {
        self.expires_at = Some(*expires_at);
    }
}

impl SpotifyClientCredentials {}

impl SpotifyOAuth {
    // spotify token example:
    // {
    //    "access_token": "NgCXRK...MzYjw",
    //    "token_type": "Bearer",
    //    "scope": "user-read-private user-read-email",
    //    "expires_in": 3600,
    //    "refresh_token": "NgAagA...Um_SHo"
    // }
    fn get_cached_token(&mut self) -> Option<TokenInfo> {
        let display = self.cache_path.display();
        let mut file = match File::open(&self.cache_path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut token_info_string = String::new();
        match file.read_to_string(&mut token_info_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => {
                let mut token_info: TokenInfo = serde_json::from_str(&token_info_string).unwrap();
                // Some(token_info)
                if !SpotifyOAuth::is_scope_subset(&mut self.scope, &mut token_info.scope) {
                    return None;
                } else {
                    if SpotifyOAuth::is_token_expired(&token_info) {
                        self.refresh_access_token(&token_info.refresh_token)
                    } else {
                        None
                    }
                }
            }
        }
    }
    fn refresh_access_token(&self, refresh_token: &str) -> Option<TokenInfo> {
        let mut payload = HashMap::new();
        payload.insert("refresh_token", refresh_token);
        payload.insert("grant_type", "refresh_token");
        let client = Client::new();
        let credentials = Basic {
            username: CLIENT_ID.to_owned(),
            password: Some(CLIENT_SECRET.to_owned()),
        };
        let url = "https://accounts.spotify.com/api/token";
        let mut response = client
            .post(url)
            .header(Authorization(credentials))
            .form(&payload)
            .send()
            .expect("send request failed");
        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            let mut token_info: TokenInfo =
                serde_json::from_str(&buf).expect("parsing response content to tokenInfo error");
            let expires_in = token_info.expires_in;
            token_info.set_expires_at(&datetime_to_timestamp(expires_in));
            match serde_json::to_string(&token_info) {
                Ok(token_info_string) => {
                    self.save_token_info(&token_info_string);
                    Some(token_info)
                }
                Err(why) => {
                    panic!("couldn't convert token_info to string: {} ",
                           why.description());
                }
            }
        } else {
            None
        }
    }
    fn save_token_info(&self, token_info: &str) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(self.cache_path.as_path())
            .expect("error");
        file.write_all(token_info.as_bytes())
            .expect("error when write file");
    }
    fn is_scope_subset(needle_scope: &mut str, haystack_scope: &mut str) -> bool {
        let needle_vec: Vec<&str> = needle_scope.split_whitespace().collect();
        let haystack_vec: Vec<&str> = haystack_scope.split_whitespace().collect();
        let needle_set: HashSet<&str> = HashSet::from_iter(needle_vec);
        let haystack_set: HashSet<&str> = HashSet::from_iter(haystack_vec);
        // needle_set - haystack_set
        needle_set.is_subset(&haystack_set)
    }
    fn is_token_expired(token_info: &TokenInfo) -> bool {
        let now: DateTime<Utc> = Utc::now();
        // 10s as buffer time
        match token_info.expires_at {
            Some(expires_at) => now.timestamp() > expires_at - 10,
            None => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use serde_json;
    #[test]
    fn test_is_scope_subset() {
        let mut needle_scope = String::from("1 2 3");
        let mut haystack_scope = String::from("1 2 3 4");
        let mut broken_scope = String::from("5 2 4");
        assert!(SpotifyOAuth::is_scope_subset(&mut needle_scope, &mut haystack_scope));
        assert!(!SpotifyOAuth::is_scope_subset(&mut broken_scope, &mut haystack_scope));
    }
    #[test]
    fn test_save_token_info() {
        let spotify_oauth = SpotifyOAuth {
            client_id: "this_is_test".to_owned(),
            client_secret: "this_is_test".to_owned(),
            redirect_uri: "this_is_test".to_owned(),
            state: "this_is_test".to_owned(),
            cache_path: PathBuf::from(".spotify_token_cache"),
            scope: "this_is_test".to_owned(),
            proxies: None,
        };
        let token_info = TokenInfo {
            access_token: "this-access_token".to_owned(),
            token_type: "code".to_owned(),
            expires_in: 3600,
            expires_at: Some(1515841743),
            refresh_token: "refresh-token".to_owned(),
            scope: "scope".to_owned(),
        };

        match serde_json::to_string(&token_info) {
            Ok(token_info_string) => {
                spotify_oauth.save_token_info(&token_info_string);
                let display = spotify_oauth.cache_path.display();
                let mut file = match File::open(&spotify_oauth.cache_path) {
                    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
                    Ok(file) => file,
                };
                let mut token_info_string_from_file = String::new();
                match file.read_to_string(&mut token_info_string_from_file) {
                    Err(why) => panic!("couldn't read {}: {}", display, why.description()),
                    Ok(_) => {
                        assert_eq!(token_info_string, token_info_string_from_file);
                    }
                }
            }
            Err(why) => {
                panic!("couldn't convert token_info to string: {} ",
                       why.description())
            }
        }
    }

}

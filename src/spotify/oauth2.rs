use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs::File;
use std::iter::FromIterator;
use std::io::prelude::*;
use serde_json;
pub struct SpotifyClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub token_info: TokenInfo,
}

pub struct SpotifyOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub state: String,
    pub cache_path: PathBuf,
    pub scope: String,
    pub proxies: String,
}

#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
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
    fn get_cached_token(&self) -> Option<TokenInfo> {
        let display = self.cache_path.display();
        let mut file = match File::open(&self.cache_path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut token_info_string = String::new();
        match file.read_to_string(&mut token_info_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => {
                let token_info: TokenInfo = serde_json::from_str(&token_info_string).unwrap();
                Some(token_info)
                // if (!self.is_scope_subset(&mut self.scope, &mut token_info.scope)) {
                //     // return None
                // }
            }
        }
    }
    fn is_scope_subset(&self, needle_scope: &mut str, haystack_scope: &mut str) -> bool {
        let needle_vec: Vec<&str> = needle_scope.split_whitespace().collect();
        let haystack_vec: Vec<&str> = haystack_scope.split_whitespace().collect();
        let needle_set: HashSet<&str> = HashSet::from_iter(needle_vec);
        let haystack_set: HashSet<&str> = HashSet::from_iter(haystack_vec);
        // needle_set - haystack_set
        needle_set.is_subset(&haystack_set)
    }
}

#[cfg(test)]
mod tests {
    use super::SpotifyOAuth;
    use std::path::PathBuf;
    #[test]
    fn test_is_scope_subset() {
        let spotify_oauth = SpotifyOAuth {
            client_id: "test-this-is-this".to_owned(),
            client_secret: "test-this-is-this".to_owned(),
            redirect_uri: "test-this-is-this".to_owned(),
            state: "test-this-is-this".to_owned(),
            cache_path: PathBuf::from(".."),
            scope: "test-this-is-this".to_owned(),
            proxies: "test-this-is-this".to_owned(),
        };

        let mut needle_scope = String::from("1 2 3");
        let mut haystack_scope = String::from("1 2 3 4");
        let mut broken_scope = String::from("5 2 4");
        assert!(spotify_oauth.is_scope_subset(&mut needle_scope, &mut haystack_scope));
        assert!(!spotify_oauth.is_scope_subset(&mut broken_scope, &mut haystack_scope));
    }
}

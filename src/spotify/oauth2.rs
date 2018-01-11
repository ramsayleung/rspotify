use std::collections::HashMap;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value, Error};
pub struct SpotifyClientCredentials {
    pub client_id: String,
    pub client_secret: String,
    pub token_info: HashMap,
}

pub struct SpotifyOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub state: String,
    pub cache_path: Path,
    pub scope: String,
    pub proxies: String,
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
    fn get_cached_token(&self) {
        token_info = HashMap::new();
        let display = self.cache_path.display();
        let mut file = match File::open(&self.cache_path) {
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };
        let mut token_info_string = String::new();
        match file.read_to_string(&mut token_info_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_) => {
                let token_info: Value = serde_json::from_str(token_info_string).unwrap();
                
            }
        }
    }
}

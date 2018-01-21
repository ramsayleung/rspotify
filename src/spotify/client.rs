use serde_json;
use reqwest::header::{Authorization, Bearer, ContentType, Headers};
use reqwest::Client;
use reqwest::Method;

//  built-in battery
use std::collections::HashMap;
use std::io::Read;

use super::oauth2::SpotifyClientCredentials;
pub struct Spotify {
    pub prefix: String,
    pub access_token: Option<String>,
    pub client_credentials_manager: Option<SpotifyClientCredentials>,
}
impl Spotify {
    pub fn default(&self) -> Spotify {
        Spotify {
            prefix: "https://api.spotify.com/v1/".to_owned(),
            access_token: None,
            client_credentials_manager: None,
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Spotify {
        self.prefix = prefix.to_owned();
        self
    }
    pub fn access_token(mut self, access_token: &str) -> Spotify {
        self.access_token = Some(access_token.to_owned());
        self
    }
    pub fn client_credentials_manager(mut self,
                                      client_credential_manager: SpotifyClientCredentials)
                                      -> Spotify {
        self.client_credentials_manager = Some(client_credential_manager);
        self
    }
    pub fn build(self) -> Spotify {
        if self.access_token.is_none() && self.client_credentials_manager.is_none() {
            panic!("access_token and client_credentials_manager are none!!!");
        }
        self
    }
    fn auth_headers(&self) -> Authorization<Bearer> {
        match self.access_token {
            Some(ref token) => Authorization(Bearer { token: token.to_owned() }),
            None => {
                match self.client_credentials_manager {
                    Some(ref client_credentials_manager) => {
                        let token = client_credentials_manager.get_access_token();
                        Authorization(Bearer { token: token })
                    }
                    None => panic!("client credentials manager is none"),
                }
            }
        }
    }
    fn internal_call(&self,
                     method: Method,
                     url: &mut str,
                     payload: Option<&HashMap<&str, &str>>,
                     params: &mut HashMap<&str, String>) {
        if !url.starts_with("http") {
            let mut prefix_url = self.prefix.clone();
            let url = prefix_url.push_str(url);
        }
        if let Some(data) = payload {
            match serde_json::to_string(&data) {
                Ok(payload_string) => {
                    params.insert("data", payload_string);
                }
                Err(why) => {
                    panic!("couldn't convert payload to string: {} ", why);
                }
            }
        }
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(self.auth_headers());
        headers.set(ContentType::json());
        let mut response = client
            .post(&url.to_owned())
            .headers(headers)
            .form(&payload)
            .send()
            .expect("send request failed");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            println!("{:?}", buf);
        }

    }
}

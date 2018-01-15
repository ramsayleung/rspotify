#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate log;
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate webbrowser;

extern crate dotenv;
use serde_json::Error;
extern crate url;
extern crate percent_encoding;
extern crate rand;
extern crate base64;
#[macro_use]
extern crate hyper;

#[macro_use]
extern crate derive_builder;
use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use rocket::response::Redirect;
use reqwest::header::{Authorization, Basic, Bearer};
use reqwest::Client;
use rocket_contrib::{Json, Value};
// use rand::Rng;

use dotenv::dotenv;
use std::env;

use std::io::Read;
use std::path::PathBuf;
use std::collections::HashMap;
mod spotify;
use spotify::oauth2::{TokenInfo, SpotifyOAuth};
use spotify::util::{generate_random_string, datetime_to_timestamp};
static CLIENT_ID: &'static str = "3a205160926f4b719170b1ad97c2ad01";
static CLIENT_SECRET: &'static str = "1449bf2c59164f2b97f21322362fe4cd";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> Redirect {
    let scope = "user-read-private user-read-email";
    let state = generate_random_string(16);
    let mut authorize_parameter: HashMap<&str, &str> = HashMap::new();
    authorize_parameter.insert("response_type", "code");
    authorize_parameter.insert("scope", scope);
    authorize_parameter.insert("redirect_uri", REDIRECT_URI);
    authorize_parameter.insert("client_id", CLIENT_ID);
    authorize_parameter.insert("state", &state);
    let query_str = convert_map_to_string(&authorize_parameter);
    let mut redirect_url = String::from("https://accounts.spotify.com/authorize?");
    redirect_url.push_str(&utf8_percent_encode(&query_str, PATH_SEGMENT_ENCODE_SET).to_string());
    Redirect::to(&redirect_url)
}

#[get("/callback?<query>")]
fn callback(query: &str) -> &str {
    println!("{:?}", query);
    let mut parameters: HashMap<&str, &str> = convert_string_to_map(query);
    parameters.insert("redirect_uri", REDIRECT_URI);
    parameters.insert("grant_type", "authorization_code");
    // parameters.insert("grant_type", "client_credentials");
    // parameters.insert("grant_type", "implicit_grant");
    parameters.remove("state");
    let client = Client::new();
    let credentials = Basic {
        username: CLIENT_ID.to_owned(),
        password: Some(CLIENT_SECRET.to_owned()),
    };
    let mut response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&parameters)
        .header(Authorization(credentials))
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
    response
        .read_to_string(&mut buf)
        .expect("failed to read response");
    if response.status().is_success() {
        let mut token_info: TokenInfo = serde_json::from_str(&buf)
            .expect("parsing response content to tokenInfo error");
        let expires_in = token_info.expires_in;
        token_info.set_expires_at(&datetime_to_timestamp(expires_in));
        let access_token = token_info.access_token;
        let bearer_credentials = Bearer { token: access_token };
        println!("expires_at:{:?}", token_info.expires_at);

        let oauth = SpotifyOAuth {
            client_id: CLIENT_ID.to_owned(),
            client_secret: CLIENT_SECRET.to_owned(),
            redirect_uri: REDIRECT_URI.to_owned(),
            state: generate_random_string(16),
            cache_path: PathBuf::from(".token_cache"),
            scope: "read_private".to_owned(),
            proxies: None,
        };
        if let Some(refresh_token) = token_info.refresh_token {
            oauth.refresh_access_token(&refresh_token);
        }
        let mut me_response = client
            .get("https://api.spotify.com/v1/me")
            .header(Authorization(bearer_credentials))
            .send()
            .expect("failed to sent get request");
        buf.clear();

        me_response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        println!("{:?}", buf);
    }
    println!("content: {}", buf);
    "helloworld"
}
#[get("/oauth")]
fn oauth() -> &'static str {
    let mut oauth = SpotifyOAuth {
        client_id: CLIENT_ID.to_owned(),
        client_secret: CLIENT_SECRET.to_owned(),
        redirect_uri: REDIRECT_URI.to_owned(),
        state: generate_random_string(16),
        cache_path: PathBuf::from(".token_cache"),
        scope: "user-read-email".to_owned(),
        proxies: None,
    };
    match oauth.get_cached_token() {
        Some(token_info) => {
            println!("{:?}", token_info);
        }
        None => println!("nothing"),
    }
    "helloworld"
}

fn convert_string_to_map(query: &str) -> HashMap<&str, &str> {
    let mut parameters: HashMap<&str, &str> = HashMap::new();
    for key_value_pair in query.split("&") {
        let tokens: Vec<&str> = key_value_pair.split("=").collect();
        parameters.insert(tokens[0], tokens[1]);
    }
    parameters
}

fn convert_map_to_string(map: &HashMap<&str, &str>) -> String {
    let mut string: String = String::new();
    for (key, &value) in map.iter() {
        string.push_str(key);
        string.push_str("=");
        string.push_str(value);
        string.push_str("&");
    }
    string
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, login, callback, oauth])
        .launch();
}

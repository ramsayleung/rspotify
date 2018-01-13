#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate log;
extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;
extern crate url;
extern crate percent_encoding;
extern crate rand;
extern crate base64;
#[macro_use]
extern crate hyper;

use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use rocket::response::Redirect;
use reqwest::header::{Authorization, Basic, Bearer};
use reqwest::Client;
use rand::Rng;

use std::io::Read;
use std::collections::HashMap;
mod spotify;
use spotify::oauth2;

static CLIENT_ID: &'static str = "3a205160926f4b719170b1ad97c2ad01";
static CLIENT_SECRET: &'static str = "1449bf2c59164f2b97f21322362fe4cd";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";

#[derive(Clone,Debug,Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub scope: String,
}

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

fn generate_random_string(length: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(length).collect()
}
#[derive(Debug,Clone)]
enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    ImplicitGrant,
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
        let token_info: TokenInfo = serde_json::from_str(&buf)
            .expect("parsing response content to tokenInfo error");
        let access_token = token_info.access_token;
        let refresh_token = token_info.refresh_token;
        let bearer_credentials = Bearer { token: access_token };
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
        .mount("/", routes![index, login, callback])
        .launch();
}

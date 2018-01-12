#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate log;
extern crate rocket;
extern crate reqwest;
extern crate serde_json;
extern crate url;
extern crate percent_encoding;
extern crate rand;
extern crate base64;
#[macro_use]
extern crate hyper;

use hyper::header::Headers;
use percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
use rocket::response::Redirect;
use reqwest::header::{Authorization, Basic, Scheme};
use reqwest::{Error, Client};
use rand::Rng;
use base64::{encode, decode};

use std::fmt::{self, Display};
use std::io::Read;
use std::str::{FromStr, from_utf8};
use std::collections::HashMap;

static CLIENT_ID: &'static str = "3a205160926f4b719170b1ad97c2ad01";
static CLIENT_SECRET: &'static str = "1449bf2c59164f2b97f21322362fe4cd";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";

#[derive(Clone, PartialEq, Debug)]
pub struct SpotifyBasic {
    pub client_id: String,
    pub client_secret: String,
}

impl Scheme for SpotifyBasic {
    fn scheme() -> Option<&'static str> {
        Some("SpotifyBasic")
    }

    fn fmt_scheme(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = self.client_id.clone();
        text.push(':');
        text.push_str(&self.client_secret);
        f.write_str(&encode(&text))
    }
}

impl FromStr for SpotifyBasic {
    type Err = hyper::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match decode(s) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(text) => {
                    let parts = &mut text.split(':');
                    let client_id = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(hyper::Error::Header)
                    };
                    let client_secret = match parts.next() {
                        Some(part) => part.to_owned(),
                        None => return Err(hyper::Error::Header)
                    };
                    Ok(SpotifyBasic {
                        client_id: client_id,
                        client_secret: client_secret,
                    })
                }
                Err(_) => {
                    debug!("SpotifyBasic::from_str utf8 error");
                    Err(hyper::Error::Header)
                }
            },
            Err(_) => {
                debug!("SpotifyBasic::from_str base64 error");
                Err(hyper::Error::Header)
            }
        }
    }
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
    redirect_url.push_str(&utf8_percent_encode(&query_str, PATH_SEGMENT_ENCODE_SET)
        .to_string());
    Redirect::to(&redirect_url)
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng().gen_ascii_chars().take(length).collect()
}

#[get("/callback?<query>")]
fn callback(query: &str) -> &str {
    println!("{:?}", query);
    let query="code=AQACEnFBZ5UWoJyukAvoa6y84VROmBEONRtow_9fdtAJFMqrBNcGUiJzdNzjRaoPU71wLg8F3k30erMieiT5HnqqKAA8BQG6dvcnSOy-y8_jtBo3viKeaE3mxjZIIQXSxLf87xkfLYyECNigupPBWsikCuPkDeyFTg80ziuWiMvvupHpS-fSs207LAfIz4gigHTJMzKSq0TrA2VzoLvUSCmlNzORYZKhnHrP94YQStQYaIDmwlDTAg&state=1h2qexRwJv1wcCI7";
    let mut parameters: HashMap<&str, &str> = convert_string_to_map(query);
    parameters.insert("redirect_url", REDIRECT_URI);
    parameters.insert("grant_type", "authorization_code");
    let client = Client::new();
    let credentials = SpotifyBasic {
        client_id: CLIENT_ID.to_string(),
        client_secret: CLIENT_SECRET.to_string(),
    };
    let mut response = client.get("https://accounts.spotify.com/api/token")
        .header(Authorization(credentials))
        .send()
        .expect("Failed to send request");
    let mut buf = String::new();
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

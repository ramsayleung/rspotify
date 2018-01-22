#![allow(dead_code)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate log;
extern crate env_logger;
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
pub mod spotify;
// use spotify::client::Spotify;
// use spotify::model::album_item::Item;
// use spotify::oauth2::{TokenInfo, SpotifyOAuth, SpotifyClientCredentials};
// use spotify::util::{generate_random_string, datetime_to_timestamp, prompt_for_user_token};
// use spotify::spotify_enum::{ALBUM_TYPE, TYPE};


#![feature(universal_impl_trait)]
#![allow(dead_code)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate webbrowser;
#[macro_use]
extern crate error_chain;

extern crate dotenv;
// use serde_json::Error;
extern crate url;
extern crate percent_encoding;
extern crate rand;
extern crate base64;
extern crate hyper;

pub mod spotify;
mod errors;
pub use errors::{Result, Error, ErrorKind};

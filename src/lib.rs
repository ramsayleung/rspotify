// #![feature(non_ascii_idents)]
#![allow(clippy::needless_doctest_main)]
#![allow(dead_code)]
//! ## Description
//! Rspotify is a lightweight wrapper for the [Spotify Web API](https://developer.spotify.com/web-api/) It includes helper functions for
//! **all Spotify's endpoints**, such as fetching metadata (search and look-up of
//! albums, artists, tracks, playlists, new releases) and user's information (follow
//! users, artists and playlists, and saved tracks management).
//! ## Features
//! *rspotify* supports all of the features of the Spotify Web API including access
//! to all end points, and support for user authorization, notes that before
//! accessing to any end points, you need to be authorized. For details on the
//! capabilities you are encouraged to review the [Spotify Web Endpoint
//! Reference](https://developer.spotify.com/web-api/endpoint-reference/)
//! documentation
//!
//! ## Getting Started
//! ### Authorization
//! Since all methods require user authorization now, you will need to
//! generate an authorization token that indicates that the user has granted
//! permission for your application to perform the given task.  You will need to
//! register your app to get the credentials necessary to make authorized calls.
//!
//! Even if your script does not have an accessible URL you need to specify one when
//! registering your application where the spotify authentication API will redirect
//! to after successful login. The URL doesn't need to work or be accessible, you
//! can specify "http://localhost/", after successful login you just need to copy
//! the "http://localhost/?code=..." URL from your browser and paste it to the
//! console where your application is running.
//!
//! ## Examples
//! If you have a use case you are intertested in, you could check the
//! [examples](./examples), which has all kinds of detailed examples. For example,
//! If you want to get recently played history, you could check
//! [current_user_recently_played](https://github.com/samrayleung/rspotify/blob/master/examples/current_user_recently_played.rs). This is
//! the example code:
//! ``` rust
//! extern crate rspotify;
//!
//! use rspotify::client::Spotify;
//! use rspotify::util::get_token;
//! use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
//!
//! #[tokio::main]
//! async fn main() {
//! // Set client_id and client_secret in .env file or
//! // export CLIENT_ID="your client_id"
//! // export CLIENT_SECRET="secret"
//! // export REDIRECT_URI=your-direct-uri
//!
//! // Or set client_id, client_secret,redirect_uri explictly
//! // let oauth = SpotifyOAuth::default()
//! //     .client_id("this-is-my-client-id")
//! //     .client_secret("this-is-my-client-secret")
//! //     .redirect_uri("http://localhost:8888/callback")
//! //     .build();
//!
//! let mut oauth = SpotifyOAuth::default()
//! .scope("user-read-recently-played")
//! .build();
//! match get_token(&mut oauth).await {
//! Some(token_info) => {
//! let client_credential = SpotifyClientCredentials::default()
//! .token_info(token_info)
//! .build();
//! // Or set client_id and client_secret explictly
//! // let client_credential = SpotifyClientCredentials::default()
//! //     .client_id("this-is-my-client-id")
//! //     .client_secret("this-is-my-client-secret")
//! //     .build();
//! let spotify = Spotify::default()
//! .client_credentials_manager(client_credential)
//! .build();
//! let history = spotify.current_user_recently_played(10).await;
//! println!("{:?}", history);
//! }
//! None => println!("auth failed"),
//! };
//! }
//!
//! ```

#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(any(feature = "default-tls", feature = "blocking"))]
extern crate reqwest_default_tls as reqwest;

#[cfg(any(feature = "native-tls-crate", feature = "native-tls-crate-blocking"))]
extern crate reqwest_native_tls as reqwest;

#[cfg(any(
    feature = "native-tls-vendored",
    feature = "native-tls-vendored-blocking"
))]
extern crate reqwest_native_tls_vendored as reqwest;

#[cfg(any(feature = "rustls-tls", feature = "rustls-tls-blocking"))]
extern crate reqwest_rustls_tls as reqwest;

extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate webbrowser;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;
extern crate dotenv;
// use serde_json::Error;
extern crate base64;
extern crate percent_encoding;
extern crate rand;
extern crate url;

#[cfg(feature = "blocking")]
pub mod blocking;

pub mod client;
pub mod model;
pub mod oauth2;
pub mod senum;
pub mod util;

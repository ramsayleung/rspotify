//! ### Blocking API
//!
//! The optional API that can be enabled for non-asynchronous requests with
//! the `blocking` feature. Its usage is exactly the same as the asynchronous
//! client:
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "0.10.0", features = ["blocking"] }
//! ```
//!
//! ```rust
//! use rspotify::blocking::client::Spotify;
//! use rspotify::blocking::oauth2::SpotifyClientCredentials;
//! use rspotify::enums::Country;
//!
//! fn main() {
//!     let client_credential = SpotifyClientCredentials::default().build();
//!     let spotify = Spotify::default()
//!         .client_credentials_manager(client_credential)
//!         .build();
//!     let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
//!     let tracks = spotify
//!         .artist_top_tracks(birdy_uri, Country::UnitedStates);
//!     println!("{:?}", tracks.unwrap());
//! }
//! ```

pub mod client;
pub mod oauth2;
pub mod util;

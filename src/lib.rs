//! Rspotify is a wrapper for the [Spotify Web API
//! ](https://developer.spotify.com/web-api/), inspired by [spotipy
//! ](https://github.com/plamere/spotipy). It includes support for all the
//! [authorization flows](https://developer.spotify.com/documentation/general/guides/authorization-guide/),
//! and helper methods for [all available endpoints
//! ](https://developer.spotify.com/documentation/web-api/reference/).
//!
//! ## Configuration
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rspotify = "0.10.0"
//! ```
//!
//! By default, Rspotify uses asynchronous programming with `async` and
//! `await`, but the `blocking` feature can be enabled to have access to the
//! [blocking](blocking/index.html) module, with non-async methods.
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "0.10.0", features = ["blocking"] }
//! ```
//!
//! Rspotify supports the [`dotenv` crate
//! ](https://github.com/dotenv-rs/dotenv), which allows you to save
//! credentials in a `.env` file by enabling the `env-file` feature. These
//! credentials will then be available as environmental values that Rspotify
//! will read when constructing types such as [`SpotifyClientCredentials`
//! ](oauth2/struct.SpotifyClientCredentials.html).
//!
//! Rspotify includes support to obtain access tokens with the
//! [`util::get_token`](util/fn.get_token.html), [`util::get_token_without_cache`
//! ](util/fn.get_token_without_cache.html) and [`util::request_token`
//! ](util/fn.request_token.html) functions. These require opening a browser
//! with the [`webbrowser` crate](https://github.com/amodm/webbrowser-rs) and
//! user interaction, which might not be necessary for non-CLI applications,
//! and can be disabled:
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "0.10.0", default-features = false, features = ["reqwest/default-tls", "browser"] }
//! ```
//!
//! Thanks to [`reqwest`](https://docs.rs/reqwest/0.10.1/reqwest/#proxies),
//! Rspotify supports system proxies by default. `reqwest` reads the
//! environment variables `HTTP_PROXY` and `HTTPS_PROXY` environmental
//! variables to set HTTP and HTTPS proxies, respectively.
//!
//! ## Getting Started
//!
//! ### Authorization
//!
//! Since all methods require user authorization, you will need to generate a
//! token that indicates that the user has granted permission for your
//! application to perform the given task. You will need to [register your app
//! to get the credentials necessary to make authorized calls
//! ](https://developer.spotify.com/dashboard/applications). Read the
//! [official guide for a detailed explanation of the different authorization
//! flows available](https://developer.spotify.com/documentation/general/guides/authorization-guide/).
//!
//! Even if your script does not have an accessible URL, you will have to
//! specify a redirect URI when registering your application where Spotify
//! will redirect to after a successful login. The URL doesn't need to work
//! or be accessible, you can use `http://localhost/`, and an [authorization
//! code](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
//! will be returned as the `code` HTTP parameter:
//! `http://localhost/?code=...`, which can be used by Rspotify to obtain an
//! access token for your requests. For example:
//!
//! ![demo](https://raw.githubusercontent.com/ramsayleung/rspotify/master/doc/images/rspotify.gif)
//!
//! In order to help other developers to get used to `rspotify`, I registered
//! a Spotify account with temporary email. You can test `rspotify` with this
//! account's `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`, check the
//! [`.env` file](https://github.com/ramsayleung/rspotify/blob/master/.env)
//! for more details.
//!
//! ### Examples
//!
//! There are some [available examples
//! ](https://github.com/ramsayleung/rspotify/tree/master/examples)
//! which can serve as a learning tool. The following snippet will obtain the
//! top tracks for an artist:
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "0.10.0" }
//! tokio = { version = "0.2", features = ["full"] }
//! ```
//!
//! ```rust
//! use rspotify::client::Spotify;
//! use rspotify::oauth2::SpotifyClientCredentials;
//! use rspotify::senum::Country;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Set client_id and client_secret in .env file or
//!     // export RSPOTIFY_CLIENT_ID="your client_id"
//!     // export RSPOTIFY_CLIENT_SECRET="secret"
//!     let client_credential = SpotifyClientCredentials::default().build();
//!
//!     // Or set client_id and client_secret explictly
//!     // let client_credential = SpotifyClientCredentials::default()
//!     //     .client_id("this-is-my-client-id")
//!     //     .client_secret("this-is-my-client-secret")
//!     //     .build();
//!     let spotify = Spotify::default()
//!         .client_credentials_manager(client_credential)
//!         .build();
//!     let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
//!     let tracks = spotify
//!         .artist_top_tracks(birdy_uri, Country::UnitedStates)
//!         .await;
//!     println!("{:?}", tracks.unwrap());
//! }
//! ```

pub mod client;
mod http;
pub mod model;
pub mod oauth2;
pub mod senum;
pub mod util;

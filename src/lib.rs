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
//! In a nutshell, these are the steps you need to make authenticated requests:
//! 1. Generate a request URL with `Spotify::get_authorize_request_url`
//! 2. The user logs in with the request URL, which redirects to the redirect
//!    URI and provides a code in the parameters.
//! 3. The code may be parsed with `Spotify::parse_response_code`.
//! 3. The code is sent to Spotify in order to obtain an access token with
//!    `Spotify::request_access_token` or
//!    `Spotify::request_access_token_without_cache`
//! 4. Finally, this access token can be used internally for the requests.
//!    This access token may expire relatively soon, so it can be refreshed
//!    with the refresh token (obtained in the third step as well) using
//!    `Spotify::refresh_access_token` or
//!    `Spotify::refresh_access_token_without_cache`. Otherwise, a new access
//!    token may be generated from scratch by repeating these steps, but the
//!    advantage of refreshing it is that this doesn't require the user to log
//!    in, and that it's a simpler procedure.
//!
//! See the `webapp` example for more details on how you can implement it for
//! something like a web server.
//!
//! If you're developing a CLI application, you might be interested in the
//! `cli` feature, which brings the `Spotify::prompt_for_user_token` and
//! `Spotify::prompt_for_user_token_without_cache` methods. These will
//! run all the authentication steps. The user wil log in by opening the
//! request URL in its default browser, and the requests will be performed
//! automatically.
//!
//! An example of the CLI authentication:
//!
//! ![demo](https://raw.githubusercontent.com/ramsayleung/rspotify/master/doc/images/rspotify.gif)
//!
//! Note: even if your script does not have an accessible URL, you will have to
//! specify a redirect URI. It doesn't need to work or be accessible, you can
//! use `http://localhost:8888/callback` for example, which will also have the
//! code appended like so: `http://localhost/?code=...`.
//!
//! In order to help other developers to get used to `rspotify`, there are
//! public credentials available for a dummy account. You can test `rspotify`
//! with this account's `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`
//! inside the [`.env` file
//! ](https://github.com/ramsayleung/rspotify/blob/master/.env) for more
//! details.
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

/// Reduce boilerplate when inserting new elements in a JSON object.
#[doc(hidden)]
#[macro_export]
macro_rules! json_insert {
    ($json:expr, $p1:expr, $p2:expr) => {
        // TODO: maybe into instead
        $json.as_object_mut().unwrap().insert($p1.to_string(), json!($p2))
    }
}

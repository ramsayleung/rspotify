//! Rspotify is a wrapper for the [Spotify Web API
//! ](https://developer.spotify.com/web-api/), inspired by [spotipy
//! ](https://github.com/plamere/spotipy). It includes support for all the
//! [authorization flows](https://developer.spotify.com/documentation/general/guides/authorization-guide/),
//! and helper methods for [all available endpoints
//! ](https://developer.spotify.com/documentation/web-api/reference/).
//!
//! ## Configuration
//!
//! By default, Rspotify uses the [`reqwest`](https://docs.rs/reqwest)
//! asynchronous HTTP client with its default TLS, but you can customize both
//! the HTTP client and the TLS with the following features:
//!
//! - `client-reqwest`, TLS available:
//!     + `reqwest-default-tls` (reqwest's default)
//!     + `reqwest-rustls-tls`
//!     + `reqwest-native-tls`
//!     + `reqwest-native-tls-vendored`
//! - `client-ureq`, TLS available:
//!     + `ureq-rustls-tls` (ureq's default)
//!
//! If you want to use a different client or TLS than the default ones, you'll
//! have to disable the default features and enable whichever you want. For
//! example, this would compile Rspotify with `reqwest` and the native TLS:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-reqwest", "reqwest-native-tls"]
//! }
//! ```
//! Rspotify uses [maybe_async](https://docs.rs/maybe-async/0.2.0/maybe_async/)
//! crate to switch between different HTTP clients to get different features.
//! In our case, `rspotify` supports async and blocking feature by triggering
//! `maybe_async` inside `Cargo.toml`, so we don't have maintain two sets of
//! code which are mostly same. By default, Rspotify uses asynchronous
//! programming with `async` and `await` by leveraging `reqwest` crate by
//! using `client-reqwest` feature. And the blocking IO feature can be
//! enabled to access with `client-ureq` feature, with non-async methods
//! and the `rustls` TLS:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-ureq", "ureq-rustls-tls"]
//! }
//! ```
//!
//! [`reqwest`](https://docs.rs/reqwest/#proxies) supports system proxies by
//! default. It reads the environment variables `HTTP_PROXY` and `HTTPS_PROXY`
//! environmental variables to set HTTP and HTTPS proxies, respectively.
//!
//! Rspotify supports the [`dotenv` crate
//! ](https://github.com/dotenv-rs/dotenv), which allows you to save
//! credentials in a `.env` file. These will then be available as environmental
//! values when using methods like [`CredentialsBuilder::from_env`
//! ](oauth2/struct.CredentialsBuilder.html#method.from_env):
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "...", features = ["env-file"] }
//! ```
//!
//! Rspotify includes support for CLI apps to obtain access tokens by prompting
//! the user, after enabling the `cli` feature. See the [Authorization
//! ](#authorization) section for more information.
//!
//! ## Getting Started
//!
//! ### Authorization
//!
//! All endpoints require authorization. You will need to generate a token
//! that indicates that the client has been granted permission to perform
//! requests. You will need to [register your app to get the necessary client
//! credentials](https://developer.spotify.com/dashboard/applications). Read
//! the [official guide for a detailed explanation of the different
//! authorization flows available
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/).
//!
//! The most basic authentication flow, named the [Client Credentials flow
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow),
//! consists on requesting a token to Spotify given some client credentials.
//! This can be done with [`Spotify::request_client_token`
//! ](client/struct.Spotify.html#method.request_client_token), as seen in
//! [this example
//! ](https://github.com/ramsayleung/rspotify/blob/master/examples/album.rs).
//!
//! Some of the available endpoints also require access to the user's personal
//! information, meaning that you have to follow the [Authorization Flow
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
//! instead. In a nutshell, these are the steps you need to make for this:
//!
//! 0. Generate a request URL with [`Spotify::get_authorize_url`
//!    ](client/struct.Spotify.html#method.get_authorize_url).
//! 1. The user logs in with the request URL, which redirects to the redirect
//!    URI and provides a code in the parameters. This happens on your side.
//! 2. The code obtained in the previous step is parsed with
//!    [`Spotify::parse_response_code`
//!    ](client/struct.Spotify.html#method.parse_response_code).
//! 3. The code is sent to Spotify in order to obtain an access token with
//!    [`Spotify::request_user_token`
//!    ](client/struct.Spotify.html#method.request_user_token) or
//!    [`Spotify::request_user_token_without_cache`
//!    ](client/struct.Spotify.html#method.prompt_for_user_token_without_cache).
//! 4. Finally, this access token can be used internally for the requests.
//!    This access token may expire relatively soon, so it can be refreshed
//!    with the refresh token (obtained in the third step as well) using
//!    [`Spotify::refresh_user_token`
//!    ](client/struct.Spotify.html#method.refresh_user_token) or
//!    [`Spotify::refresh_user_token_without_cache`
//!    ](client/struct.Spotify.html#method.refresh_user_token_without_cache).
//!    Otherwise, a new access token may be generated from scratch by repeating
//!    these steps, but the advantage of refreshing it is that this doesn't
//!    require the user to log in, and that it's a simpler procedure.
//!
//! See the [`webapp`
//! ](https://github.com/ramsayleung/rspotify/tree/master/examples/webapp)
//! example for more details on how you can implement it for something like a
//! web server.
//!
//! If you're developing a CLI application, you might be interested in the
//! `cli` feature, which brings the [`Spotify::prompt_for_user_token`
//! ](client/struct.Spotify.html#method.prompt_for_user_token) and
//! [`Spotify::prompt_for_user_token_without_cache`
//! ](client/struct.Spotify.html#method.prompt_for_user_token_without_cache)
//! methods. These will run all the authentication steps. The user wil log in
//! by opening the request URL in its default browser, and the requests will be
//! performed automatically.
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
//! which can serve as a learning tool.

// disable all modules when both client features are enabled,
// this way only the compile error below gets show
// instead of showing a whole list of confusing errors
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub mod client;
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
mod http;
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub mod model;
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub mod oauth2;
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub mod util;

#[cfg(all(feature = "client-reqwest", feature = "client-ureq"))]
compile_error!(
    "`client-reqwest` and `client-ureq` features cannot both be enabled at the same time, \
  if you want to use `client-ureq` you need to set `default-features = false`"
);

#[doc(hidden)]
mod macros {
    /// Reduce boilerplate when inserting new elements in a JSON object.
    #[macro_export]
    macro_rules! json_insert {
        ($json:expr, $p1:expr, $p2:expr) => {
            $json
                .as_object_mut()
                .unwrap()
                .insert($p1.to_string(), json!($p2))
        };
    }
}

//! Rspotify is a wrapper for the [Spotify Web API][spotify-main], inspired by
//! [spotipy][spotipy-github]. It includes support for all the [authorization
//! flows][spotify-auth-flows], and helper methods for [all available
//! endpoints][spotify-reference].
//!
//! ## Configuration
//!
//! ### HTTP Client
//!
//! By default, Rspotify uses the [reqwest][reqwest-docs] asynchronous HTTP
//! client with its default TLS, but you can customize both the HTTP client and
//! the TLS with the following features:
//!
//! - [reqwest][reqwest-docs]: enabling
//!   `client-reqwest`, TLS available:
//!     + `reqwest-default-tls` (reqwest's default)
//!     + `reqwest-rustls-tls`
//!     + `reqwest-native-tls`
//!     + `reqwest-native-tls-vendored`
//! - [ureq][ureq-docs]: enabling `client-ureq`, TLS
//!   available:
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
//!
//! [`maybe_async`] internally enables Rspotify to  use both synchronous and
//! asynchronous HTTP clients. You can also use `ureq`, a synchronous client,
//! like so:
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
//! ### Proxies
//!
//! [reqwest supports system proxies by default][reqwest-proxies]. It reads the
//! environment variables `HTTP_PROXY` and `HTTPS_PROXY` environmental variables
//! to set HTTP and HTTPS proxies, respectively.
//!
//! ### Environmental variables
//!
//! Rspotify supports the [`dotenv`] crate, which allows you to save credentials
//! in a `.env` file. These will then be automatically available as
//! environmental values when using methods like [`Credentials::from_env`].
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "...", features = ["env-file"] }
//! ```
//!
//! ### CLI utilities
//!
//! Rspotify includes basic support for Cli apps to obtain access tokens by
//! prompting the user, after enabling the `cli` feature. See the
//! [Authorization](#authorization) section for more information.
//!
//! ## Getting Started
//!
//! ### Authorization
//!
//! All endpoints require app authorization; you will need to generate a token
//! that indicates that the client has been granted permission to perform
//! requests. You can start by [registering your app to get the necessary client
//! credentials][spotify-register-app]. Read the [official guide for a detailed
//! explanation of the different authorization flows
//! available][spotify-auth-flows].
//!
//! Rspotify has a different client for each of the available authentication
//! flows. They may implement the endpoints in
//! [`BaseClient`](crate::clients::BaseClient) or
//! [`OAuthClient`](crate::clients::OAuthClient) according to what kind of
//! flow it is. Please refer to their documentation for more details:
//!
//! * [Client Credentials Flow][spotify-client-creds]: see
//!   [`ClientCredsSpotify`].
//! * [Authorization Code Flow][spotify-auth-code]: see [`AuthCodeSpotify`].
//! * [Authorization Code Flow with Proof Key for Code Exchange
//!   (PKCE)][spotify-auth-code-pkce]: see [`AuthCodePkceSpotify`].
//! * [Implicit Grant Flow][spotify-implicit-grant]: unimplemented, as Rspotify
//!   has not been tested on a browser yet. If you'd like support for it, let us
//!   know in an issue!
//!
//! In order to help other developers to get used to `rspotify`, there are
//! public credentials available for a dummy account. You can test `rspotify`
//! with this account's `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET` inside
//! the [`.env` file](https://github.com/ramsayleung/rspotify/blob/master/.env)
//! for more details.
//!
//! ### Examples
//!
//! There are some [available examples on the GitHub
//! repository][examples-github] which can serve as a learning tool.
//!
//! [spotipy-github]: https://github.com/plamere/spotipy
//! [reqwest-docs]: https://docs.rs/reqwest/
//! [reqwest-proxies]: https://docs.rs/reqwest/#proxies
//! [ureq-docs]: https://docs.rs/ureq/
//! [examples-github]: https://github.com/ramsayleung/rspotify/tree/master/examples
//! [spotify-main]: https://developer.spotify.com/documentation/web-api/
//! [spotify-auth-flows]: https://developer.spotify.com/documentation/general/guides/authorization/
//! [spotify-reference]: https://developer.spotify.com/documentation/web-api/reference/
//! [spotify-register-app]: https://developer.spotify.com/dashboard/applications
//! [spotify-client-creds]: https://developer.spotify.com/documentation/general/guides/authorization/client-credentials/
//! [spotify-auth-code]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
//! [spotify-auth-code-pkce]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
//! [spotify-implicit-grant]: https://developer.spotify.com/documentation/general/guides/authorization/implicit-grant

pub mod auth_code;
pub mod auth_code_pkce;
pub mod client_creds;
pub mod clients;

// Subcrate re-exports
pub use rspotify_http as http;
pub use rspotify_macros as macros;
pub use rspotify_model as model;
// Top-level re-exports
pub use auth_code::AuthCodeSpotify;
pub use auth_code_pkce::AuthCodePkceSpotify;
pub use client_creds::ClientCredsSpotify;
pub use macros::scopes;
pub use model::Token;

use crate::{http::HttpError, model::Id};

use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
};

use getrandom::getrandom;
use thiserror::Error;

pub mod prelude {
    pub use crate::clients::{BaseClient, OAuthClient};
    pub use crate::model::idtypes::{Id, PlayContextId, PlayableId};
}

/// Common headers as constants.
pub(in crate) mod params {
    pub const CLIENT_ID: &str = "client_id";
    pub const CODE: &str = "code";
    pub const GRANT_TYPE: &str = "grant_type";
    pub const GRANT_TYPE_AUTH_CODE: &str = "authorization_code";
    pub const GRANT_TYPE_CLIENT_CREDS: &str = "client_credentials";
    pub const GRANT_TYPE_REFRESH_TOKEN: &str = "refresh_token";
    pub const REDIRECT_URI: &str = "redirect_uri";
    pub const REFRESH_TOKEN: &str = "refresh_token";
    pub const RESPONSE_TYPE_CODE: &str = "code";
    pub const RESPONSE_TYPE: &str = "response_type";
    pub const SCOPE: &str = "scope";
    pub const SHOW_DIALOG: &str = "show_dialog";
    pub const STATE: &str = "state";
    pub const CODE_CHALLENGE: &str = "code_challenge";
    pub const CODE_VERIFIER: &str = "code_verifier";
    pub const CODE_CHALLENGE_METHOD: &str = "code_challenge_method";
    pub const CODE_CHALLENGE_METHOD_S256: &str = "S256";
}

/// Common alphabets for random number generation and similars
pub(in crate) mod alphabets {
    pub const ALPHANUM: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    /// From https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
    pub const PKCE_CODE_VERIFIER: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~";
}

pub(in crate) mod auth_urls {
    pub const AUTHORIZE: &str = "https://accounts.spotify.com/authorize";
    pub const TOKEN: &str = "https://accounts.spotify.com/api/token";
}

/// Possible errors returned from the `rspotify` client.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    // Note that this type is boxed because its size might be very large in
    // comparison to the rest. For more information visit:
    // https://rust-lang.github.io/rust-clippy/master/index.html#large_enum_variant
    #[error("http error: {0}")]
    Http(Box<HttpError>),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "cli")]
    #[error("cli error: {0}")]
    Cli(String),

    #[error("cache file error: {0}")]
    CacheFile(String),

    #[error("model error: {0}")]
    Model(#[from] model::ModelError),
}

// The conversion has to be done manually because it's in a `Box<T>`
impl From<HttpError> for ClientError {
    fn from(err: HttpError) -> Self {
        ClientError::Http(Box::new(err))
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

pub const DEFAULT_API_PREFIX: &str = "https://api.spotify.com/v1/";
pub const DEFAULT_CACHE_PATH: &str = ".spotify_token_cache.json";
pub const DEFAULT_PAGINATION_CHUNKS: u32 = 50;

/// Struct to configure the Spotify client.
#[derive(Debug, Clone)]
pub struct Config {
    /// The Spotify API prefix, [`DEFAULT_API_PREFIX`] by default.
    pub prefix: String,

    /// The cache file path, in case it's used. By default it's
    /// [`DEFAULT_CACHE_PATH`]
    pub cache_path: PathBuf,

    /// The pagination chunk size used when performing automatically paginated
    /// requests, like [`artist_albums`](crate::clients::BaseClient). This
    /// means that a request will be performed every `pagination_chunks` items.
    /// By default this is [`DEFAULT_PAGINATION_CHUNKS`].
    ///
    /// Note that most endpoints set a maximum to the number of items per
    /// request, which most times is 50.
    pub pagination_chunks: u32,

    /// Whether or not to save the authentication token into a JSON file,
    /// then reread the token from JSON file when launching the program without
    /// following the full auth process again
    pub token_cached: bool,

    /// Whether or not to check if the token has expired when sending a
    /// request with credentials, and in that case, automatically refresh it.
    pub token_refreshing: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            prefix: String::from(DEFAULT_API_PREFIX),
            cache_path: PathBuf::from(DEFAULT_CACHE_PATH),
            pagination_chunks: DEFAULT_PAGINATION_CHUNKS,
            token_cached: false,
            token_refreshing: false,
        }
    }
}

/// Generate `length` random chars from the Operating System.
///
/// It is assumed that system always provides high-quality cryptographically
/// secure random data, ideally backed by hardware entropy sources.
pub(in crate) fn generate_random_string(length: usize, alphabet: &[u8]) -> String {
    let mut buf = vec![0u8; length];
    getrandom(&mut buf).unwrap();
    let range = alphabet.len();

    buf.iter()
        .map(|byte| alphabet[*byte as usize % range] as char)
        .collect()
}

#[inline]
pub(in crate) fn join_ids<'a, T: Id + 'a + ?Sized>(ids: impl IntoIterator<Item = &'a T>) -> String {
    ids.into_iter().map(Id::id).collect::<Vec<_>>().join(",")
}

#[inline]
pub(in crate) fn join_scopes(scopes: &HashSet<String>) -> String {
    scopes
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Simple client credentials object for Spotify.
#[derive(Debug, Clone, Default)]
pub struct Credentials {
    pub id: String,
    /// PKCE doesn't require a client secret
    pub secret: Option<String>,
}

impl Credentials {
    /// Initialization with both the client ID and the client secret
    pub fn new(id: &str, secret: &str) -> Self {
        Credentials {
            id: id.to_owned(),
            secret: Some(secret.to_owned()),
        }
    }

    /// Initialization with just the client ID
    pub fn new_pkce(id: &str) -> Self {
        Credentials {
            id: id.to_owned(),
            secret: None,
        }
    }

    /// Parses the credentials from the environment variables
    /// `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`. You can optionally
    /// activate the `env-file` feature in order to read these variables from
    /// a `.env` file.
    pub fn from_env() -> Option<Self> {
        #[cfg(feature = "env-file")]
        {
            dotenv::dotenv().ok();
        }

        Some(Credentials {
            id: env::var("RSPOTIFY_CLIENT_ID").ok()?,
            secret: env::var("RSPOTIFY_CLIENT_SECRET").ok(),
        })
    }

    /// Generates an HTTP basic authorization header with proper formatting
    ///
    /// This will only work when the client secret is set to `Option::Some`.
    pub fn auth_headers(&self) -> Option<HashMap<String, String>> {
        let auth = "authorization".to_owned();
        let value = format!("{}:{}", self.id, self.secret.as_ref()?);
        let value = format!("Basic {}", base64::encode(value));

        let mut headers = HashMap::new();
        headers.insert(auth, value);
        Some(headers)
    }
}

/// Structure that holds the required information for requests with OAuth.
#[derive(Debug, Clone)]
pub struct OAuth {
    pub redirect_uri: String,
    /// The state is generated by default, as suggested by the OAuth2 spec:
    /// [Cross-Site Request Forgery](https://tools.ietf.org/html/rfc6749#section-10.12)
    pub state: String,
    /// You could use macro [scopes!](crate::scopes) to build it at compile time easily
    pub scopes: HashSet<String>,
    pub proxies: Option<String>,
}

impl Default for OAuth {
    fn default() -> Self {
        OAuth {
            redirect_uri: String::new(),
            state: generate_random_string(16, alphabets::ALPHANUM),
            scopes: HashSet::new(),
            proxies: None,
        }
    }
}

impl OAuth {
    /// Parses the credentials from the environment variable
    /// `RSPOTIFY_REDIRECT_URI`. You can optionally activate the `env-file`
    /// feature in order to read these variables from a `.env` file.
    pub fn from_env(scopes: HashSet<String>) -> Option<Self> {
        #[cfg(feature = "env-file")]
        {
            dotenv::dotenv().ok();
        }

        Some(OAuth {
            scopes,
            redirect_uri: env::var("RSPOTIFY_REDIRECT_URI").ok()?,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{alphabets, generate_random_string, Credentials};
    use std::collections::HashSet;

    #[test]
    fn test_generate_random_string() {
        let mut containers = HashSet::new();
        for _ in 1..101 {
            containers.insert(generate_random_string(10, alphabets::ALPHANUM));
        }
        assert_eq!(containers.len(), 100);
    }

    #[test]
    fn test_basic_auth() {
        let creds = Credentials::new_pkce("ramsay");
        let headers = creds.auth_headers();
        assert_eq!(headers, None);

        let creds = Credentials::new("ramsay", "123456");

        let headers = creds.auth_headers().unwrap();
        assert_eq!(headers.len(), 1);
        assert_eq!(
            headers.get("authorization"),
            Some(&"Basic cmFtc2F5OjEyMzQ1Ng==".to_owned())
        );
    }
}

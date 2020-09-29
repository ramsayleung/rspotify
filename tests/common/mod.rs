//! Some common methods, specially to be able to use the tests with different
//! clients.

use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder, Token};

use lazy_static::lazy_static;
use maybe_async::{async_impl, maybe_async, sync_impl};

#[cfg(feature = "__sync")]
pub use test as maybe_async_test;

#[cfg(feature = "__async")]
pub use tokio::test as maybe_async_test;

#[cfg(feature = "__sync")]
lazy_static! {
    pub static ref CREDS_TOKEN: Token = get_creds_token();
    pub static ref OAUTH_TOKEN: Token = get_oauth_token();
}

#[cfg(feature = "__async")]
lazy_static! {
    pub static ref CREDS_TOKEN: async_once::AsyncOnce<Token> =
        async_once::AsyncOnce::new(async { get_creds_token().await });
    pub static ref OAUTH_TOKEN: async_once::AsyncOnce<Token> =
        async_once::AsyncOnce::new(async { get_oauth_token().await });
}

/// Generating a new client for the requests.
#[sync_impl]
pub fn oauth_client() -> Spotify {
    SpotifyBuilder::default()
        .token(OAUTH_TOKEN.clone())
        .build()
        .unwrap()
}

#[async_impl]
pub async fn oauth_client() -> Spotify {
    SpotifyBuilder::default()
        .token(get_oauth_token().await)
        .build()
        .unwrap()
}

#[sync_impl]
pub fn creds_client() -> Spotify {
    SpotifyBuilder::default()
        .token(CREDS_TOKEN.clone())
        .build()
        .unwrap()
}

#[async_impl]
pub async fn creds_client() -> Spotify {
    SpotifyBuilder::default()
        .token(get_creds_token().await)
        .build()
        .unwrap()
}

/// Set client_id and client_secret in .env file (with the `env-file`
/// feature) or:
///
/// export RSPOTIFY_CLIENT_ID="your client_id"
/// export RSPOTIFY_CLIENT_SECRET="secret"
#[maybe_async]
pub async fn get_creds_token() -> Token {
    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = CredentialsBuilder::from_env().build().unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .build()
        .unwrap();

    spotify.request_client_token().await.unwrap();

    spotify.token.unwrap()
}

/// With so many tests, it's a better idea to authenticate only once at the
/// beginning. The `Spotify` instance needed here is for async requests,
/// so this uses `AsyncOnce` to work with `lazy_static`.
#[maybe_async]
pub async fn get_oauth_token() -> Token {
    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = CredentialsBuilder::from_env().build().unwrap();

    // Using every possible scope
    let oauth = OAuthBuilder::from_env()
        .scope(
            "user-read-email user-read-private user-top-read
             user-read-recently-played user-follow-read user-library-read
             user-read-currently-playing user-read-playback-state
             user-read-playback-position playlist-read-collaborative
             playlist-read-private user-follow-modify user-library-modify
             user-modify-playback-state playlist-modify-public
             playlist-modify-private ugc-image-upload",
        )
        .build()
        .unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    spotify.prompt_for_user_token().await.unwrap();

    spotify.token.unwrap()
}

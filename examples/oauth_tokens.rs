//! This test is specially useful for the OAuth tests. It simply obtains
//! an access token and a refresh token with all available scopes.
//!
//! Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and RSPOTIFY_REDIRECT_URI
//! in an .env file or export them manually as environmental variables for this
//! to work.

use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = CredentialsBuilder::from_env().build().unwrap();

    // Using every possible scope
    let oauth = OAuthBuilder::from_env()
        .scope(
            "user-read-email user-read-private user-top-read \
             user-read-recently-played user-follow-read user-library-read \
             user-read-currently-playing user-read-playback-state \
             user-read-playback-position playlist-read-collaborative \
             playlist-read-private user-follow-modify user-library-modify \
             user-modify-playback-state playlist-modify-public \
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

    let token = spotify.token.as_ref().unwrap();
    println!("Access token: {}", &token.access_token);
    println!("Refresh token: {}", token.refresh_token.as_ref().unwrap());
}

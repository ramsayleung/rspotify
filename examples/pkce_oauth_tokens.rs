//! This test is specially useful for the OAuth tests. It simply obtains
//! an access token and a refresh token with all available scopes.
//!
//! Change `creds.id` and `oauth.redirect_uri`
//! in `fn main` for this to work.

use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder, AuthorizationFlow};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = CredentialsBuilder::default()
        .id("f77f5697dff14094a3c4ef4dc83710e9")
        .build()
        .unwrap();

    // Using every possible scope
    let oauth = OAuthBuilder::default()
        .authorization_flow(AuthorizationFlow::AuthorizationCodeWithPKCE)
        .redirect_uri("http://localhost:8888/callback")
        .scope(r#"
            user-read-email
            user-read-private
            user-read-recently-played
            user-read-currently-playing
            user-read-playback-state
            user-read-playback-position
            user-top-read
            user-follow-read
            user-follow-modify
            user-library-read
            user-library-modify
            user-modify-playback-state
            playlist-read-collaborative
            playlist-read-private
            playlist-modify-public
            playlist-modify-private
            ugc-image-upload
        "#,)
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

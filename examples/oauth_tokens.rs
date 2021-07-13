//! This example is specially useful for the OAuth tests. It simply obtains an
//! access token and a refresh token with all available scopes.
//!
//! Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and RSPOTIFY_REDIRECT_URI in
//! an .env file or export them manually as environmental variables for this to
//! work.

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

use std::{io::{self, Write}, env};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!(
        "playlist-modify-private",
        "playlist-modify-public",
        "playlist-read-collaborative",
        "playlist-read-private",
        "ugc-image-upload",
        "user-follow-modify",
        "user-follow-read",
        "user-library-modify",
        "user-library-read",
        "user-modify-playback-state",
        "user-read-currently-playing",
        "user-read-email",
        "user-read-playback-position",
        "user-read-playback-state",
        "user-read-private",
        "user-read-recently-played",
        "user-top-read"
    );
    let oauth = OAuth::from_env(scopes).unwrap();

    let mut spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let token = spotify.token.as_ref().unwrap();
    let access_token = &token.access_token;
    let refresh_token = token.refresh_token.as_ref().unwrap();
    println!("Access token: {}", access_token);
    println!("Refresh token: {}", refresh_token);

    print!("Export to the environment? [y/N]: ");
    io::stdout().flush().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    let input = input.trim();
    if input == "" || input == "y" || input == "Y" {
        env::set_var("RSPOTIFY_ACCESS_TOKEN", access_token);
        env::set_var("RSPOTIFY_REFRESH_TOKEN", refresh_token);
        println!("Exported RSPOTIFY_ACCESS_TOKEN and RSPOTIFY_REFRESH_TOKEN");
    }
}

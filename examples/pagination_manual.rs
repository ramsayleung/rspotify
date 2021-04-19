//! This example shows how manual pagination works. It's what the raw API
//! returns, but harder to use than an iterator or stream.

use rspotify::client::SpotifyBuilder;
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};
use rspotify::scopes;

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and
    // RSPOTIFY_REDIRECT_URI in an .env file or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // let creds = CredentialsBuilder::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build()
    //     .unwrap();
    let creds = CredentialsBuilder::from_env().build().unwrap();

    // Or set the redirect_uri explictly:
    //
    // let oauth = OAuthBuilder::default()
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build()
    //     .unwrap();
    let oauth = OAuthBuilder::from_env()
        .scope(scopes!("user-library-read"))
        .build()
        .unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    // Obtaining the access token
    spotify.prompt_for_user_token().await.unwrap();

    // Manual pagination. You may choose the number of items returned per
    // iteration.
    let limit = 50;
    let mut offset = 0;
    println!("Items:");
    loop {
        let page = spotify
            .current_user_saved_tracks_manual(limit, offset)
            .await
            .unwrap();
        for item in page.items {
            println!("* {}", item.track.name);
        }

        // The iteration ends when the `next` field is `None`. Otherwise, the
        // Spotify API will keep returning empty lists from then on.
        if page.next.is_none() {
            break;
        }

        offset += limit;
    }
}

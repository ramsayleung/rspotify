//! This example shows how manual pagination works. It's what the raw API
//! returns, but harder to use than an iterator or stream.

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();
    let oauth = OAuth::from_env(scopes!("user-library-read")).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();

    // Manual pagination. You may choose the number of items returned per
    // iteration.
    let limit = 50;
    let mut offset = 0;
    println!("Items:");
    loop {
        let page = spotify
            .current_user_saved_tracks_manual(None, Some(limit), Some(offset))
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

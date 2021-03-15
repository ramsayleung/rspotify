//! This example showcases how streams can be used for asynchronous automatic
//! pagination.
//!
//! Asynchronous iteration is a bit uglier, since there's currently no
//! syntactic sugar for `for` loops. See this article for more information:
//!
//! https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html

use futures::stream::TryStreamExt;
use futures_util::pin_mut;
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

    // Executing the futures sequentially
    let stream = spotify.current_user_saved_tracks_stream();
    pin_mut!(stream);
    println!("Items (blocking):");
    while let Some(item) = stream.try_next().await.unwrap() {
        println!("* {}", item.track.name);
    }

    // Executing the futures concurrently
    let stream = spotify.current_user_saved_tracks_stream();
    println!("\nItems (concurrent):");
    stream
        .try_for_each_concurrent(10, |item| async move {
            println!("* {}", item.track.name);
            Ok(())
        })
        .await
        .unwrap();
}

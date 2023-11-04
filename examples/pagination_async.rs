//! This example showcases how streams can be used for asynchronous automatic
//! pagination.
//!
//! You'll need to run `cargo add futures futures_util` to get the `pin_mut!`
//! macro and the `stream.try_next()` method.
//!
//! Asynchronous iteration is a bit uglier, since there's currently no
//! syntactic sugar for `for` loops. See this article for more information:
//!
//! https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html

use futures::stream::TryStreamExt;
use futures_util::pin_mut;
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

    // Executing the futures sequentially
    let stream = spotify.current_user_saved_tracks(None);
    pin_mut!(stream);
    println!("Items (blocking):");
    while let Some(item) = stream.try_next().await.unwrap() {
        println!("* {}", item.track.name);
    }

    // Executing the futures concurrently
    let stream = spotify.current_user_saved_tracks(None);
    println!("\nItems (concurrent):");
    stream
        .try_for_each_concurrent(10, |item| async move {
            println!("* {}", item.track.name);
            Ok(())
        })
        .await
        .unwrap();
}

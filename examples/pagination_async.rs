//! This example showcases how streams can be used for asynchronous automatic
//! pagination.
//!
//! Asynchronous iteration is a bit uglier, since there's currently no
//! syntactic sugar for `for` loops. See this article for more information:
//!
//! https://rust-lang.github.io/async-book/05_streams/02_iteration_and_concurrency.html

use futures::stream::TryStreamExt;
use futures_util::pin_mut;
use rspotify::{scopes, CodeAuthSpotify, Credentials};

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    let creds = Credentials::from_env().unwrap();
    let mut oauth = OAuth::from_env().unwrap();
    oauth.scope = scopes!("user-library-read");

    let mut spotify = CodeAuthSpotify::new(creds, oauth);

    // Obtaining the access token
    let url = spotify.get_authorize_url(false);
    spotify.prompt_for_user_token(url).await.unwrap();

    // Executing the futures sequentially
    let stream = spotify.current_user_saved_tracks();
    pin_mut!(stream);
    println!("Items (blocking):");
    while let Some(item) = stream.try_next().await.unwrap() {
        println!("* {}", item.track.name);
    }

    // Executing the futures concurrently
    let stream = spotify.current_user_saved_tracks();
    println!("\nItems (concurrent):");
    stream
        .try_for_each_concurrent(10, |item| async move {
            println!("* {}", item.track.name);
            Ok(())
        })
        .await
        .unwrap();
}

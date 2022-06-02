//! This example shows how automatic pagination works for synchronous clients.
//!
//! Synchronous iteration is easier than the async method shown in the
//! `current_user_saved_tracks` example, but you can also use:
//!
//! ```
//! while let Some(item) = stream.next() {
//!     // ...
//! }
//! ```

use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};

fn main() {
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
    spotify.prompt_for_token(&url).unwrap();

    // Typical iteration, no extra boilerplate needed.
    let stream = spotify.current_user_saved_tracks(None);
    println!("Items:");
    for item in stream {
        println!("* {}", item.unwrap().track.name);
    }
}

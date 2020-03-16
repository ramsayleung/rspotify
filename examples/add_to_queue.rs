extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let birdy_uri = String::from("spotify:track:6rqhFgbbKwnb9MLmUQDhG6");
            let _res = spotify.add_item_to_queue(birdy_uri, None).await;
        }
        None => println!("auth failed"),
    }
}

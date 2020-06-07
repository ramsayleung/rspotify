extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::AdditionalType;
use rspotify::util::get_token;

#[tokio::main]
async fn main() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    // export REDIRECT_URI=your-direct-uri

    // Or set client_id, client_secret,redirect_uri explictly
    // let oauth = SpotifyOAuth::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build();

    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let additional_types = vec![AdditionalType::Episode];
            let result = spotify.current_playback(None, Some(additional_types)).await;
            match result {
                Ok(context) => match context {
                    Some(current_playing) => {
                        println!("get current_playback {:?}", current_playing);
                        println!(
                            "get currently_playing_type: {:?}",
                            current_playing.currently_playing_type
                        );
                    }
                    None => println!("Nothing is playbacking"),
                },
                Err(err) => println!("get current_playback error {:?}", err),
            }
        }
        None => println!("auth failed"),
    };
}

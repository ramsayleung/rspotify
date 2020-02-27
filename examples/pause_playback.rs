extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
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
        .scope("user-modify-playback-state")
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
            // this is the example device_id from spotify website,
            // so it will raise a 403 error, just change this device_id to yours
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            match spotify.pause_playback(Some(device_id)).await {
                Ok(_) => println!("pause playback successful"),
                Err(_) => eprintln!("pause playback failed"),
            }
        }
        None => println!("auth failed"),
    };
}

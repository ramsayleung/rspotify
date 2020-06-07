extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;
use rspotify::senum::Country;

fn main() {
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
        .scope("user-read-playback-position")
        .build();
    match get_token(&mut oauth) {
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

            let id = "38bS44xjbVVZ3No3ByF1dJ".to_owned();
            let result = spotify.get_shows_episodes(id, 10, 0, Some(Country::France));
            match result {
                Ok(episodes) => println!("{:?}", episodes),
                Err(_) => println!("get episodes of shows failed"),
            }
        }
        None => println!("auth failed"),
    };
}

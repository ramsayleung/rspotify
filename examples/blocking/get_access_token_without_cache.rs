extern crate rspotify;

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token_without_cache;

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

    let mut spotify_oauth = SpotifyOAuth::default().build();
    match get_token_without_cache(&mut spotify_oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info.to_owned())
                .build();

            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "spotify";
            let mut playlist_id = String::from("59ZbFPES4DQwEjBpWHzrtC");
            let playlists = spotify.user_playlist(user_id, Some(&mut playlist_id), None, None);
            println!("{:?}", playlists);
            match spotify_oauth
                .refresh_access_token_without_cache(&token_info.refresh_token.unwrap())
            {
                Some(refresh_token) => {
                    println!("refresh token: {:?}", refresh_token);
                }
                None => println!("refresh token failed"),
            }
        }
        None => println!("auth failed"),
    };
}

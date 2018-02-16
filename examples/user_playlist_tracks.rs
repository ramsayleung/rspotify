extern crate rspotify;

use rspotify::spotify::util::get_token;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};

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
    match get_token(&mut spotify_oauth) {
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
            let user_id = "spotify";
            let playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
            let playlists =
                spotify.user_playlist_tracks(user_id, &playlist_id, None, Some(2), None, None);
            println!("{:?}", playlists);

        }
        None => println!("auth failed"),
    };

}

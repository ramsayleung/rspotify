extern crate rspotify;
extern crate serde_json;

use serde_json::map::Map;
use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
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

    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
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
            //this is my(samray's) user_id and playlist_id, so just change
            // user_id and playlist_id to yours, or you will get a 403 forbidden error
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = String::from("5jAOgWXCBKuinsGiZxjDQ5");
            let mut tracks = vec![];
            let mut map1 = Map::new();
            let mut position1 = vec![];
            position1.push(0);
            position1.push(3);
            map1.insert("uri".to_string(),
                        "spotify:track:4iV5W9uYEdYUVa79Axb7Rh".into());
            map1.insert("position".to_string(), position1.into());
            tracks.push(map1);
            let mut map2 = Map::new();
            let mut position2 = vec![];
            position2.push(7);
            map2.insert("uri".to_string(),
                        "spotify:track:1301WleyT98MSxVHPZCA6M".into());
            map2.insert("position".to_string(), position2.into());
            tracks.push(map2);
            let result = spotify
                .user_playlist_remove_specific_occurrenes_of_tracks(user_id,
                                                                    &playlist_id,
                                                                    tracks,
                                                                    None);
            println!("result:{:?}", result);
        }
        None => println!("auth failed"),
    };

}

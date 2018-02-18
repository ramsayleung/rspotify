extern crate rspotify;
extern crate serde_json;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::util::get_token;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::Country;
use serde_json::map::Map;

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

    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
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
            let mut payload = Map::new();
            let seed_artists = vec!["4NHQUGzhtTLFvgF5SZesLK".to_owned()];
            let seed_tracks = vec!["0c6xIDDpzE81m2q797ordA".to_owned()];
            payload.insert("min_energy".to_owned(), 0.4.into());
            payload.insert("min_popularity".to_owned(), 50.into());
            let result = spotify.recommendations(Some(seed_artists),
                                                 None,
                                                 Some(seed_tracks),
                                                 10,
                                                 Some(Country::UnitedStates),
                                                 &payload);
            println!("search result:{:?}", result);
        }
        None => println!("auth failed"),
    };

}

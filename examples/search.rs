extern crate rspotify;

use rspotify::client::Spotify;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::{Country, SearchType};
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

    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
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
            let album_query = "album:arrival artist:abba";
            let result = spotify
                .search(album_query, SearchType::Album, 10, 0, None, None)
                .await;
            match result {
                Ok(album) => println!("searched album:{:?}", album),
                Err(err) => println!("search error!{:?}", err),
            }

            let artist_query = "tania bowra";
            let result = spotify
                .search(
                    artist_query,
                    SearchType::Artist,
                    10,
                    0,
                    Some(Country::UnitedStates),
                    None,
                )
                .await;
            match result {
                Ok(album) => println!("searched artist:{:?}", album),
                Err(err) => println!("search error!{:?}", err),
            }

            let playlist_query = "\"doom metal\"";
            let result = spotify
                .search(
                    playlist_query,
                    SearchType::Playlist,
                    10,
                    0,
                    Some(Country::UnitedStates),
                    None,
                )
                .await;
            match result {
                Ok(album) => println!("searched playlist:{:?}", album),
                Err(err) => println!("search error!{:?}", err),
            }

            let track_query = "abba";
            let result = spotify
                .search(
                    track_query,
                    SearchType::Track,
                    10,
                    0,
                    Some(Country::UnitedStates),
                    None,
                )
                .await;
            match result {
                Ok(album) => println!("searched track:{:?}", album),
                Err(err) => println!("search error!{:?}", err),
            }

            let show_query = "love";
            let result = spotify
                .search(show_query, SearchType::Show, 10, 0, None, None)
                .await;
            match result {
                Ok(show) => println!("searched show:{:?}", show),
                Err(err) => println!("search error!{:?}", err),
            }

            let episode_query = "love";
            let result = spotify
                .search(episode_query, SearchType::Episode, 10, 0, None, None)
                .await;
            match result {
                Ok(episode) => println!("searched episode:{:?}", episode),
                Err(err) => println!("search error!{:?}", err),
            }
        }
        None => println!("auth failed"),
    };
}

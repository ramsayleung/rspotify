use rspotify::client::SpotifyBuilder;
use rspotify::model::{Country, SearchType};
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};

fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // Set RSPOTIFY_CLIENT_ID, RSPOTIFY_CLIENT_SECRET and
    // RSPOTIFY_REDIRECT_URI in an .env file or export them manually:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    //
    // These will then be read with `from_env`.
    //
    // Otherwise, set client_id and client_secret explictly:
    //
    // let creds = CredentialsBuilder::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build()
    //     .unwrap();
    let creds = CredentialsBuilder::from_env().build().unwrap();

    // Or set the redirect_uri explictly:
    //
    // let oauth = OAuthBuilder::default()
    //     .redirect_uri("http://localhost:8888/callback")
    //     .build()
    //     .unwrap();
    let oauth = OAuthBuilder::from_env()
        .scope("user-read-playback-state")
        .build()
        .unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    // Obtaining the access token
    spotify.request_client_token().unwrap();

    let album_query = "album:arrival artist:abba";
    let result = spotify.search(album_query, SearchType::Album, 10, 0, None, None);
    match result {
        Ok(album) => println!("searched album:{:?}", album),
        Err(err) => println!("search error!{:?}", err),
    }

    let artist_query = "tania bowra";
    let result = spotify.search(
        artist_query,
        SearchType::Artist,
        10,
        0,
        Some(Country::UnitedStates),
        None,
    );
    match result {
        Ok(album) => println!("searched artist:{:?}", album),
        Err(err) => println!("search error!{:?}", err),
    }

    let playlist_query = "\"doom metal\"";
    let result = spotify.search(
        playlist_query,
        SearchType::Playlist,
        10,
        0,
        Some(Country::UnitedStates),
        None,
    );
    match result {
        Ok(album) => println!("searched playlist:{:?}", album),
        Err(err) => println!("search error!{:?}", err),
    }

    let track_query = "abba";
    let result = spotify.search(
        track_query,
        SearchType::Track,
        10,
        0,
        Some(Country::UnitedStates),
        None,
    );
    match result {
        Ok(album) => println!("searched track:{:?}", album),
        Err(err) => println!("search error!{:?}", err),
    }

    let show_query = "love";
    let result = spotify.search(show_query, SearchType::Show, 10, 0, None, None);
    match result {
        Ok(show) => println!("searched show:{:?}", show),
        Err(err) => println!("search error!{:?}", err),
    }

    let episode_query = "love";
    let result = spotify.search(episode_query, SearchType::Episode, 10, 0, None, None);
    match result {
        Ok(episode) => println!("searched episode:{:?}", episode),
        Err(err) => println!("search error!{:?}", err),
    }
}

use rspotify::{
    model::{Country, Market, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // May require the `env-file` feature enabled if the environment variables
    // aren't configured manually.
    let creds = Credentials::from_env().unwrap();
    let spotify = ClientCredsSpotify::new(creds);

    // Obtaining the access token
    spotify.request_token().unwrap();

    let album_query = "album:arrival artist:abba";
    let result = spotify.search(album_query, SearchType::Album, None, None, Some(10), None);
    match result {
        Ok(album) => println!("Searched album: {album:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }

    let artist_query = "tania bowra";
    let result = spotify.search(
        artist_query,
        SearchType::Artist,
        Some(Market::Country(Country::UnitedStates)),
        None,
        Some(10),
        None,
    );
    match result {
        Ok(album) => println!("Searched artist: {album:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }

    let playlist_query = "\"doom metal\"";
    let result = spotify.search(
        playlist_query,
        SearchType::Playlist,
        Some(Market::Country(Country::UnitedStates)),
        None,
        Some(10),
        None,
    );
    match result {
        Ok(album) => println!("Searched playlist: {album:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }

    let track_query = "abba";
    let result = spotify.search(
        track_query,
        SearchType::Track,
        Some(Market::Country(Country::UnitedStates)),
        None,
        Some(10),
        None,
    );
    match result {
        Ok(album) => println!("Searched track: {album:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }

    let show_query = "love";
    let result = spotify.search(show_query, SearchType::Show, None, None, Some(10), None);
    match result {
        Ok(show) => println!("Searched show: {show:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }

    let episode_query = "love";
    let result = spotify.search(
        episode_query,
        SearchType::Episode,
        None,
        None,
        Some(10),
        None,
    );
    match result {
        Ok(episode) => println!("Searched episode: {episode:?}"),
        Err(err) => println!("Search error! {err:?}"),
    }
}

use rspotify::client::Spotify;
use rspotify::enums::{AlbumType, Country};
use rspotify::oauth2::SpotifyClientCredentials;

use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    // Set client_id and client_secret in .env file or
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    static ref CLIENT_CREDENTIAL: Mutex<SpotifyClientCredentials>
        = Mutex::new(SpotifyClientCredentials::default().build());
}

fn async_client() -> Spotify {
    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    Spotify::default()
        .client_credentials_manager(CLIENT_CREDENTIAL.lock().unwrap().clone())
        .build()
}

#[tokio::test]
async fn test_album() {
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = async_client().album(birdy_uri).await;
    assert!(albums.is_ok());
}

#[tokio::test]
async fn test_albums() {
    let birdy_uri1 = "spotify:album:41MnTivkwTO3UUJ8DrqEJJ";
    let birdy_uri2 = "spotify:album:6JWc4iAiJ9FjyK0B59ABb4";
    let birdy_uri3 = "spotify:album:6UXCm6bOO4gFlDQZV5yL37";
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    let albums = async_client().albums(track_uris).await;
    assert!(albums.is_ok())
}

#[tokio::test]
async fn test_album_tracks() {
    let birdy_uri = "spotify:album:6akEvsycLGftJxYudPjmqK";
    let tracks = async_client().album_track(birdy_uri, Some(2), None).await;
    assert!(tracks.is_ok());
}

#[tokio::test]
async fn test_artist_related_artists() {
    let birdy_uri = "spotify:artist:43ZHCT0cAZBISjO8DG9PnE";
    let artist = async_client().artist_related_artists(birdy_uri).await;
    assert!(artist.is_ok())
}

#[tokio::test]
async fn test_artist() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let artist = async_client().artist(birdy_uri).await;
    assert!(artist.is_ok());
}

#[tokio::test]
async fn test_artists_albums() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let albums = async_client()
        .artist_albums(
            birdy_uri,
            Some(AlbumType::Album),
            Some(Country::UnitedStates),
            Some(10),
            None,
        )
        .await;
    assert!(albums.is_ok());
}

#[tokio::test]
async fn test_artists() {
    let birdy_uri1 = "spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy";
    let birdy_uri2 = "spotify:artist:3dBVyJ7JuOMt4GE9607Qin";
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    let artists = async_client().artists(artist_uris).await;
    assert!(artists.is_ok());
}

#[tokio::test]
async fn test_artist_top_tracks() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = async_client()
        .artist_top_tracks(birdy_uri, Country::UnitedStates)
        .await;
    assert!(tracks.is_ok());
}

#[tokio::test]
async fn test_audio_analysis() {
    let track = "06AKEBrKUckW0KREUWRnvT";
    let analysis = async_client().audio_analysis(track).await;
    assert!(analysis.is_ok());
}

#[tokio::test]
async fn test_audio_features() {
    let track = "spotify:track:06AKEBrKUckW0KREUWRnvT";
    let features = async_client().audio_features(track).await;
    assert!(features.is_ok());
}

#[tokio::test]
async fn test_audios_features() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4JpKVNYnVcJ8tuMKjAj50A";
    tracks_ids.push(track_id1);
    let track_id2 = "spotify:track:24JygzOLM0EmRQeGtFcIcG";
    tracks_ids.push(track_id2);
    let features = async_client().audios_features(tracks_ids).await;
    assert!(features.is_ok())
}

#[tokio::test]
async fn test_user() {
    let birdy_uri = String::from("tuggareutangranser");
    let user = async_client().user(&birdy_uri).await;
    assert!(user.is_ok());
}

#[tokio::test]
async fn test_track() {
    let birdy_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
    let track = async_client().track(birdy_uri).await;
    assert!(track.is_ok());
}

#[tokio::test]
async fn test_tracks() {
    let birdy_uri1 = "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp";
    let birdy_uri2 = "spotify:track:3twNvmDtFQtAd5gMKedhLD";
    let track_uris = vec![birdy_uri1, birdy_uri2];
    let tracks = async_client().tracks(track_uris, None).await;
    assert!(tracks.is_ok());
}

#[tokio::test]
async fn test_existing_playlist() {
    let playlist = async_client()
        .playlist("37i9dQZF1DZ06evO45P0Eo", None, None)
        .await;
    assert!(playlist.is_ok());
}

#[tokio::test]
async fn test_fake_playlist() {
    let playlist = async_client().playlist("fake_id", None, None).await;
    assert!(!playlist.is_ok());
}

#[tokio::test]
async fn test_add_queue() {
    let birdy_uri = String::from("spotify:track:6rqhFgbbKwnb9MLmUQDhG6");
    let res = async_client().add_item_to_queue(birdy_uri, None).await;
    assert!(!res.is_ok());
}

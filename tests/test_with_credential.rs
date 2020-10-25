mod common;

use common::maybe_async_test;
use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::model::{AlbumType, Country};
use rspotify::oauth2::CredentialsBuilder;

use maybe_async::maybe_async;

/// Generating a new basic client for the requests.
#[maybe_async]
pub async fn creds_client() -> Spotify {
    // The credentials must be available in the environment. Enable
    // `env-file` in order to read them from an `.env` file.
    let creds = CredentialsBuilder::from_env().build().unwrap();

    let mut spotify = SpotifyBuilder::default()
        .credentials(creds)
        .build()
        .unwrap();

    spotify.request_client_token().await.unwrap();

    spotify
}

#[maybe_async]
#[maybe_async_test]
async fn test_album() {
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    creds_client().await.album(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_albums() {
    let birdy_uri1 = "spotify:album:41MnTivkwTO3UUJ8DrqEJJ";
    let birdy_uri2 = "spotify:album:6JWc4iAiJ9FjyK0B59ABb4";
    let birdy_uri3 = "spotify:album:6UXCm6bOO4gFlDQZV5yL37";
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    creds_client().await.albums(track_uris).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_album_tracks() {
    let birdy_uri = "spotify:album:6akEvsycLGftJxYudPjmqK";
    creds_client()
        .await
        .album_track(birdy_uri, Some(2), None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist_related_artists() {
    let birdy_uri = "spotify:artist:43ZHCT0cAZBISjO8DG9PnE";
    creds_client()
        .await
        .artist_related_artists(birdy_uri)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    creds_client().await.artist(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artists_albums() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    creds_client()
        .await
        .artist_albums(
            birdy_uri,
            Some(AlbumType::Album),
            Some(Country::UnitedStates),
            Some(10),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artists() {
    let birdy_uri1 = "spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy";
    let birdy_uri2 = "spotify:artist:3dBVyJ7JuOMt4GE9607Qin";
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    creds_client().await.artists(artist_uris).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist_top_tracks() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    creds_client()
        .await
        .artist_top_tracks(birdy_uri, Country::UnitedStates)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audio_analysis() {
    let track = "06AKEBrKUckW0KREUWRnvT";
    creds_client().await.track_analysis(track).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audio_features() {
    let track = "spotify:track:06AKEBrKUckW0KREUWRnvT";
    creds_client().await.track_features(track).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audios_features() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4JpKVNYnVcJ8tuMKjAj50A";
    tracks_ids.push(track_id1);
    let track_id2 = "spotify:track:24JygzOLM0EmRQeGtFcIcG";
    tracks_ids.push(track_id2);
    creds_client()
        .await
        .tracks_features(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_user() {
    let birdy_uri = String::from("tuggareutangranser");
    creds_client().await.user(&birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_track() {
    let birdy_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
    creds_client().await.track(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_tracks() {
    let birdy_uri1 = "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp";
    let birdy_uri2 = "spotify:track:3twNvmDtFQtAd5gMKedhLD";
    let track_uris = vec![birdy_uri1, birdy_uri2];
    creds_client().await.tracks(track_uris, None).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_existing_playlist() {
    creds_client()
        .await
        .playlist("37i9dQZF1DZ06evO45P0Eo", None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_fake_playlist() {
    let playlist = creds_client().await.playlist("fake_id", None, None).await;
    assert!(!playlist.is_ok());
}

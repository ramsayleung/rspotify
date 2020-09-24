use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::oauth2::{CredentialsBuilder, Token};
use rspotify::senum::{AlbumType, Country};

use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    // Set client_id and client_secret in .env file (with the `env-file`
    // feature) or:
    //
    // export RSPOTIFY_CLIENT_ID="your client_id"
    // export RSPOTIFY_CLIENT_SECRET="secret"
    static ref AUTH_TOKEN: AsyncOnce<Token> = AsyncOnce::new(async {
        // The credentials must be available in the environment. Enable
        // `env-file` in order to read them from an `.env` file.
        let creds = CredentialsBuilder::from_env()
            .build()
            .unwrap();

        let mut spotify = SpotifyBuilder::default()
            .credentials(creds)
            .build()
            .unwrap();

        spotify.request_client_token().await.unwrap();

        spotify.token.unwrap()
    });
}

/// Generating a new client for the requests.
async fn async_client() -> Spotify {
    SpotifyBuilder::default()
        .token(AUTH_TOKEN.get().await)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_album() {
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    async_client().await.album(birdy_uri).await.unwrap();
}

#[tokio::test]
async fn test_albums() {
    let birdy_uri1 = String::from("spotify:album:41MnTivkwTO3UUJ8DrqEJJ");
    let birdy_uri2 = String::from("spotify:album:6JWc4iAiJ9FjyK0B59ABb4");
    let birdy_uri3 = String::from("spotify:album:6UXCm6bOO4gFlDQZV5yL37");
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    async_client().await.albums(track_uris).await.unwrap();
}

#[tokio::test]
async fn test_album_tracks() {
    let birdy_uri = "spotify:album:6akEvsycLGftJxYudPjmqK";
    async_client()
        .await
        .album_track(birdy_uri, Some(2), None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_artist_related_artists() {
    let birdy_uri = "spotify:artist:43ZHCT0cAZBISjO8DG9PnE";
    async_client()
        .await
        .artist_related_artists(birdy_uri)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_artist() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    async_client().await.artist(birdy_uri).await.unwrap();
}

#[tokio::test]
async fn test_artists_albums() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    async_client()
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

#[tokio::test]
async fn test_artists() {
    let birdy_uri1 = String::from("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy");
    let birdy_uri2 = String::from("spotify:artist:3dBVyJ7JuOMt4GE9607Qin");
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    async_client().await.artists(artist_uris).await.unwrap();
}

#[tokio::test]
async fn test_artist_top_tracks() {
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    async_client()
        .await
        .artist_top_tracks(birdy_uri, Country::UnitedStates)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_audio_analysis() {
    let track = "06AKEBrKUckW0KREUWRnvT";
    async_client().await.audio_analysis(track).await.unwrap();
}

#[tokio::test]
async fn test_audio_features() {
    let track = "spotify:track:06AKEBrKUckW0KREUWRnvT";
    async_client().await.audio_features(track).await.unwrap();
}

#[tokio::test]
async fn test_audios_features() {
    let mut tracks_ids = vec![];
    let track_id1 = String::from("spotify:track:4JpKVNYnVcJ8tuMKjAj50A");
    tracks_ids.push(track_id1);
    let track_id2 = String::from("spotify:track:24JygzOLM0EmRQeGtFcIcG");
    tracks_ids.push(track_id2);
    async_client()
        .await
        .audios_features(&tracks_ids)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_user() {
    let birdy_uri = String::from("tuggareutangranser");
    async_client().await.user(&birdy_uri).await.unwrap();
}

#[tokio::test]
async fn test_track() {
    let birdy_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
    async_client().await.track(birdy_uri).await.unwrap();
}

#[tokio::test]
async fn test_tracks() {
    let birdy_uri1 = "spotify:track:3n3Ppam7vgaVa1iaRUc9Lp";
    let birdy_uri2 = "spotify:track:3twNvmDtFQtAd5gMKedhLD";
    let track_uris = vec![birdy_uri1, birdy_uri2];
    async_client().await.tracks(track_uris, None).await.unwrap();
}

#[tokio::test]
async fn test_existing_playlist() {
    async_client()
        .await
        .playlist("37i9dQZF1DZ06evO45P0Eo", None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_fake_playlist() {
    async_client()
        .await
        .playlist("fake_id", None, None)
        .await
        .unwrap();
}

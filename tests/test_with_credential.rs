mod common;

use common::maybe_async_test;
use rspotify::oauth2::CredentialsBuilder;
use rspotify::{
    client::{Spotify, SpotifyBuilder},
    model::{AlbumType, Country, Id, Market},
};

use maybe_async::maybe_async;

/// Generating a new basic client for the requests.
#[maybe_async]
pub async fn creds_client() -> Spotify {
    // The credentials must be available in the environment.
    let creds = CredentialsBuilder::from_env().build().unwrap_or_else(|_| {
        panic!(
            "No credentials configured. Make sure that either the `env-file` \
            feature is enabled, or that the required environment variables are \
            exported (`RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET`)"
        )
    });

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
    let birdy_uri = Id::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    creds_client().await.album(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_albums() {
    let birdy_uri1 = Id::from_uri("spotify:album:41MnTivkwTO3UUJ8DrqEJJ").unwrap();
    let birdy_uri2 = Id::from_uri("spotify:album:6JWc4iAiJ9FjyK0B59ABb4").unwrap();
    let birdy_uri3 = Id::from_uri("spotify:album:6UXCm6bOO4gFlDQZV5yL37").unwrap();
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    creds_client().await.albums(track_uris).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_album_tracks() {
    let birdy_uri = Id::from_uri("spotify:album:6akEvsycLGftJxYudPjmqK").unwrap();
    creds_client()
        .await
        .album_track_manual(birdy_uri, Some(2), None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist_related_artists() {
    let birdy_uri = Id::from_uri("spotify:artist:43ZHCT0cAZBISjO8DG9PnE").unwrap();
    creds_client()
        .await
        .artist_related_artists(birdy_uri)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist() {
    let birdy_uri = Id::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client().await.artist(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artists_albums() {
    let birdy_uri = Id::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client()
        .await
        .artist_albums_manual(
            birdy_uri,
            Some(AlbumType::Album),
            Some(&Market::Country(Country::UnitedStates)),
            Some(10),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artists() {
    let birdy_uri1 = Id::from_uri("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy").unwrap();
    let birdy_uri2 = Id::from_uri("spotify:artist:3dBVyJ7JuOMt4GE9607Qin").unwrap();
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    creds_client().await.artists(artist_uris).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_artist_top_tracks() {
    let birdy_uri = Id::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client()
        .await
        .artist_top_tracks(birdy_uri, Market::Country(Country::UnitedStates))
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audio_analysis() {
    let track = Id::from_id("06AKEBrKUckW0KREUWRnvT").unwrap();
    creds_client().await.track_analysis(track).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audio_features() {
    let track = Id::from_uri("spotify:track:06AKEBrKUckW0KREUWRnvT").unwrap();
    creds_client().await.track_features(track).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_audios_features() {
    let mut tracks_ids = vec![];
    let track_id1 = Id::from_uri("spotify:track:4JpKVNYnVcJ8tuMKjAj50A").unwrap();
    tracks_ids.push(track_id1);
    let track_id2 = Id::from_uri("spotify:track:24JygzOLM0EmRQeGtFcIcG").unwrap();
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
    let birdy_uri = Id::from_id("tuggareutangranser").unwrap();
    creds_client().await.user(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_track() {
    let birdy_uri = Id::from_uri("spotify:track:6rqhFgbbKwnb9MLmUQDhG6").unwrap();
    creds_client().await.track(birdy_uri).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_tracks() {
    let birdy_uri1 = Id::from_uri("spotify:track:3n3Ppam7vgaVa1iaRUc9Lp").unwrap();
    let birdy_uri2 = Id::from_uri("spotify:track:3twNvmDtFQtAd5gMKedhLD").unwrap();
    let track_uris = vec![birdy_uri1, birdy_uri2];
    creds_client().await.tracks(track_uris, None).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_existing_playlist() {
    creds_client()
        .await
        .playlist(Id::from_id("37i9dQZF1DZ06evO45P0Eo").unwrap(), None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_fake_playlist() {
    let playlist = creds_client()
        .await
        .playlist(Id::from_id("fakeid").unwrap(), None, None)
        .await;
    assert!(!playlist.is_ok());
}

mod test_pagination {
    use super::*;

    static ALBUM: &str = "spotify:album:2T7DdrOvsqOqU9bGTkjBYu";
    static SONG_NAMES: &[&str; 10] = &[
        "Human After All",
        "The Prime Time of Your Life",
        "Robot Rock",
        "Steam Machine",
        "Make Love",
        "The Brainwasher",
        "On / Off",
        "Television Rules the Nation",
        "Technologic",
        "Emotion",
    ];

    /// This test iterates a request of 10 items, with 5 requests of 2 items.
    #[cfg(feature = "__sync")]
    #[test]
    fn test_pagination_sync() {
        let mut client = creds_client();
        client.pagination_chunks = 2;
        let album = Id::from_uri(ALBUM).unwrap();

        let names = client
            .album_track(&album)
            .map(|track| track.unwrap().name)
            .collect::<Vec<_>>();

        assert_eq!(names, SONG_NAMES);
    }

    /// This test iterates a request of 10 items, with 5 requests of 2 items.
    #[cfg(feature = "__async")]
    #[tokio::test]
    async fn test_pagination_async() {
        use futures_util::StreamExt;

        let mut client = creds_client().await;
        client.pagination_chunks = 2;
        let album = Id::from_uri(ALBUM).unwrap();

        let names = client
            .album_track(&album)
            .map(|track| track.unwrap().name)
            .collect::<Vec<_>>()
            .await;

        assert_eq!(names, SONG_NAMES);
    }
}

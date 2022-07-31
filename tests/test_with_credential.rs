use rspotify::{
    model::{AlbumId, AlbumType, ArtistId, Country, Market, PlaylistId, TrackId, UserId},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

use maybe_async::maybe_async;

/// Generating a new basic client for the requests.
#[maybe_async]
pub async fn creds_client() -> ClientCredsSpotify {
    // The credentials must be available in the environment.
    let creds = Credentials::from_env().unwrap_or_else(|| {
        panic!(
            "No credentials configured. Make sure that either the `env-file` \
            feature is enabled, or that the required environment variables are \
            exported (`RSPOTIFY_CLIENT_ID`, `RSPOTIFY_CLIENT_SECRET`)"
        )
    });

    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();
    spotify
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_album() {
    let birdy_uri = AlbumId::from_uri("spotify:album:0sNOF9WDwhWunNAHPD3Baj").unwrap();
    creds_client().await.album(birdy_uri).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_albums() {
    let track_uris = [
        AlbumId::from_uri("spotify:album:41MnTivkwTO3UUJ8DrqEJJ").unwrap(),
        AlbumId::from_uri("spotify:album:6JWc4iAiJ9FjyK0B59ABb4").unwrap(),
        AlbumId::from_uri("spotify:album:6UXCm6bOO4gFlDQZV5yL37").unwrap(),
    ];
    creds_client().await.albums(track_uris).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_album_tracks() {
    let birdy_uri = AlbumId::from_uri("spotify:album:6akEvsycLGftJxYudPjmqK").unwrap();
    creds_client()
        .await
        .album_track_manual(&birdy_uri, Some(2), None)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_artist_related_artists() {
    let birdy_uri = ArtistId::from_uri("spotify:artist:43ZHCT0cAZBISjO8DG9PnE").unwrap();
    creds_client()
        .await
        .artist_related_artists(birdy_uri)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_artist() {
    let birdy_uri = ArtistId::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client().await.artist(birdy_uri).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_artists_albums() {
    let birdy_uri = ArtistId::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client()
        .await
        .artist_albums_manual(
            &birdy_uri,
            Some(AlbumType::Album),
            Some(Market::Country(Country::UnitedStates)),
            Some(10),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_artists() {
    let artist_uris = [
        ArtistId::from_uri("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy").unwrap(),
        ArtistId::from_uri("spotify:artist:3dBVyJ7JuOMt4GE9607Qin").unwrap(),
    ];
    creds_client().await.artists(artist_uris).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_artist_top_tracks() {
    let birdy_uri = ArtistId::from_uri("spotify:artist:2WX2uTcsvV5OnS0inACecP").unwrap();
    creds_client()
        .await
        .artist_top_tracks(birdy_uri, Market::Country(Country::UnitedStates))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_audio_analysis() {
    let track = TrackId::from_id("06AKEBrKUckW0KREUWRnvT").unwrap();
    creds_client().await.track_analysis(track).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_audio_features() {
    let track = TrackId::from_uri("spotify:track:06AKEBrKUckW0KREUWRnvT").unwrap();
    creds_client().await.track_features(track).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_audios_features() {
    let mut tracks_ids = vec![];
    let track_id1 = TrackId::from_uri("spotify:track:4JpKVNYnVcJ8tuMKjAj50A").unwrap();
    tracks_ids.push(track_id1);
    let track_id2 = TrackId::from_uri("spotify:track:24JygzOLM0EmRQeGtFcIcG").unwrap();
    tracks_ids.push(track_id2);
    creds_client()
        .await
        .tracks_features(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_user() {
    let birdy_uri = UserId::from_id("tuggareutangranser").unwrap();
    creds_client().await.user(birdy_uri).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_track() {
    let birdy_uri = TrackId::from_uri("spotify:track:6rqhFgbbKwnb9MLmUQDhG6").unwrap();
    creds_client().await.track(birdy_uri).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_tracks() {
    let track_uris = [
        TrackId::from_uri("spotify:track:3n3Ppam7vgaVa1iaRUc9Lp").unwrap(),
        TrackId::from_uri("spotify:track:3twNvmDtFQtAd5gMKedhLD").unwrap(),
    ];
    creds_client().await.tracks(track_uris, None).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_existing_playlist() {
    let playlist_id = PlaylistId::from_id("37i9dQZF1DZ06evO45P0Eo").unwrap();
    creds_client()
        .await
        .playlist(playlist_id, None, None)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
async fn test_fake_playlist() {
    let playlist_id = PlaylistId::from_id("fakeid").unwrap();
    let playlist = creds_client().await.playlist(playlist_id, None, None).await;
    assert!(playlist.is_err());
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
        client.config.pagination_chunks = 2;
        let album = AlbumId::from_uri(ALBUM).unwrap();

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
        client.config.pagination_chunks = 2;
        let album = AlbumId::from_uri(ALBUM).unwrap();

        let names = client
            .album_track(&album)
            .map(|track| track.unwrap().name)
            .collect::<Vec<_>>()
            .await;

        assert_eq!(names, SONG_NAMES);
    }
}

//! Most of tests currently require a Spotify Premium account where the tests
//! can be ran, which will be ignored in the CI for now. The tests are written
//! so that no user data is modified (or at least minimizing the changes done to
//! the account).
//!
//! You can run all of them with:
//!
//!   cargo test --features=cli,env-file -- --ignored --test-threads=1
//!
//! The access token must be obtained previously, and this test file will try
//! to authenticate with the access token from the `RSPOTIFY_ACCESS_TOKEN`
//! environment variable or the refresh token from `RSPOTIFY_REFRESH_TOKEN`.
//! These tokens must have been generated for all available scopes, for example
//! with the `oauth_tokens` example:
//!
//!   cargo run --example oauth_tokens --features=env-file,cli

use rspotify::{
    clients::pagination::Paginator,
    model::{
        AlbumId, ArtistId, Country, CurrentPlaybackContext, Device, EpisodeId, FullPlaylist,
        ItemPositions, Market, Offset, PlaylistId, RecommendationsAttribute, RepeatState,
        SearchType, ShowId, TimeLimits, TimeRange, TrackId, UserId,
    },
    prelude::*,
    scopes, AuthCodeSpotify, ClientResult, Credentials, OAuth, Token,
};

use std::env;

use chrono::{prelude::*, Duration};
use maybe_async::maybe_async;

/// Generating a new OAuth client for the requests.
#[maybe_async]
pub async fn oauth_client() -> AuthCodeSpotify {
    if let Ok(access_token) = env::var("RSPOTIFY_ACCESS_TOKEN") {
        let tok = Token {
            access_token,
            ..Default::default()
        };

        AuthCodeSpotify::from_token(tok)
    } else if let Ok(refresh_token) = env::var("RSPOTIFY_REFRESH_TOKEN") {
        // The credentials must be available in the environment. Enable
        // `env-file` in order to read them from an `.env` file.
        let creds = Credentials::from_env().unwrap_or_else(|| {
            panic!(
                "No credentials configured. Make sure that either the \
                `env-file` feature is enabled, or that the required \
                environment variables are exported (`RSPOTIFY_CLIENT_ID`, \
                `RSPOTIFY_CLIENT_SECRET`)."
            )
        });

        let scopes = scopes!(
            "playlist-modify-private",
            "playlist-modify-public",
            "playlist-read-collaborative",
            "playlist-read-private",
            "ugc-image-upload",
            "user-follow-modify",
            "user-follow-read",
            "user-library-modify",
            "user-library-read",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "user-read-email",
            "user-read-playback-position",
            "user-read-playback-state",
            "user-read-private",
            "user-read-recently-played",
            "user-top-read"
        );
        // Using every possible scope
        let oauth = OAuth::from_env(scopes).unwrap();

        // Creating a token with only the refresh token in order to obtain the
        // access token later.
        let token = Token {
            refresh_token: Some(refresh_token),
            ..Default::default()
        };

        let spotify = AuthCodeSpotify::new(creds, oauth);
        *spotify.token.lock().await.unwrap() = Some(token);
        spotify.refresh_token().await.unwrap();
        spotify
    } else {
        panic!(
            "No access tokens configured. Please set `RSPOTIFY_ACCESS_TOKEN` \
             or `RSPOTIFY_REFRESH_TOKEN`, which can be obtained with the \
             `oauth_tokens` example."
        )
    }
}

#[maybe_async]
async fn fetch_all<T>(paginator: Paginator<'_, ClientResult<T>>) -> Vec<T> {
    #[cfg(feature = "__async")]
    {
        use futures::stream::TryStreamExt;

        paginator.try_collect::<Vec<_>>().await.unwrap()
    }

    #[cfg(feature = "__sync")]
    {
        paginator.filter_map(|a| a.ok()).collect::<Vec<_>>()
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_categories() {
    oauth_client()
        .await
        .categories_manual(
            None,
            Some(&Market::Country(Country::UnitedStates)),
            Some(10),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_category_playlists() {
    oauth_client()
        .await
        .category_playlists_manual(
            "pop",
            Some(&Market::Country(Country::UnitedStates)),
            Some(10),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_playback() {
    oauth_client()
        .await
        .current_playback(None, None::<&[_]>)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_playing() {
    oauth_client()
        .await
        .current_playing(None, None::<&[_]>)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_followed_artists() {
    oauth_client()
        .await
        .current_user_followed_artists(None, Some(10))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_playing_track() {
    oauth_client()
        .await
        .current_user_playing_item()
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_recently_played() {
    let limit = TimeLimits::After(Utc::now() - Duration::days(2));
    oauth_client()
        .await
        .current_user_recently_played(Some(10), Some(limit))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_saved_albums() {
    let album_ids = [
        &AlbumId::from_id("6akEvsycLGftJxYudPjmqK").unwrap(),
        &AlbumId::from_id("628oezqK2qfmCjC6eXNors").unwrap(),
    ];

    let client = oauth_client().await;

    // First adding the albums
    client
        .current_user_saved_albums_add(album_ids)
        .await
        .unwrap();

    // Making sure the new albums appear
    let all_albums = fetch_all(client.current_user_saved_albums(None)).await;
    // Can handle albums without available_market
    let _albums_from_token =
        fetch_all(client.current_user_saved_albums(Some(&Market::FromToken))).await;
    let all_uris = all_albums
        .into_iter()
        .map(|a| a.album.id)
        .collect::<Vec<_>>();
    assert!(
        album_ids
            .iter()
            .all(|item| all_uris.contains(&(*item).to_owned())),
        "couldn't find the new saved albums"
    );

    // And then removing them
    client
        .current_user_saved_albums_delete(album_ids)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_saved_tracks_add() {
    let client = oauth_client().await;
    let tracks_ids = [
        &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        &TrackId::from_uri("spotify:track:1301WleyT98MSxVHPZCA6M").unwrap(),
    ];
    client
        .current_user_saved_tracks_add(tracks_ids)
        .await
        .unwrap();

    let contains = client
        .current_user_saved_tracks_contains(tracks_ids)
        .await
        .unwrap();
    // Every track should be saved
    assert!(contains.into_iter().all(|x| x));

    let all = fetch_all(client.current_user_saved_tracks(None)).await;
    let all = all
        .into_iter()
        .filter_map(|saved| saved.track.id)
        .collect::<Vec<_>>();
    // All the initial tracks should appear
    assert!(tracks_ids.iter().all(|track| all.contains(track)));

    client
        .current_user_saved_tracks_delete(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_top_artists() {
    oauth_client()
        .await
        .current_user_top_artists_manual(Some(&TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_top_tracks() {
    oauth_client()
        .await
        .current_user_top_tracks_manual(Some(&TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_featured_playlists() {
    let now: DateTime<Utc> = Utc::now();
    oauth_client()
        .await
        .featured_playlists(None, None, Some(&now), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_me() {
    oauth_client().await.me().await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_new_releases() {
    oauth_client()
        .await
        .new_releases_manual(Some(&Market::Country(Country::Sweden)), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_new_releases_with_from_token() {
    oauth_client()
        .await
        .new_releases_manual(Some(&Market::FromToken), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_playback() {
    let client = oauth_client().await;
    let uris: [&dyn PlayableId; 3] = [
        &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        &TrackId::from_uri("spotify:track:2DzSjFQKetFhkFCuDWhioi").unwrap(),
        &EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
    ];
    let devices = client.device().await.unwrap();

    // Save current playback data to be restored later
    // NOTE: unfortunately it's impossible to revert the entire queue, this will
    // just restore the song playing at the moment.
    let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    for (i, device) in devices.iter().enumerate() {
        let device_id = device.id.as_ref().unwrap();
        let next_device_id = devices
            .get(i + 1)
            .unwrap_or(&devices[0])
            .id
            .as_ref()
            .unwrap();

        // Starting playback of some songs
        client
            .start_uris_playback(uris, Some(device_id), Some(Offset::for_position(0)), None)
            .await
            .unwrap();

        for i in 0..uris.len() - 1 {
            client.next_track(Some(device_id)).await.unwrap();

            // Also trying to go to the previous track
            if i != 0 {
                client.previous_track(Some(device_id)).await.unwrap();
                client.next_track(Some(device_id)).await.unwrap();
            }

            // Making sure pause/resume also works
            let playback = client.current_playback(None, None::<&[_]>).await.unwrap();
            if let Some(playback) = playback {
                if playback.is_playing {
                    client.pause_playback(Some(device_id)).await.unwrap();
                    client.resume_playback(None, None).await.unwrap();
                } else {
                    client.resume_playback(None, None).await.unwrap();
                    client.pause_playback(Some(device_id)).await.unwrap();
                }
            }
        }

        client
            .transfer_playback(next_device_id, Some(true))
            .await
            .unwrap();
    }

    // Restore the original playback data
    if let Some(backup) = &backup {
        let uri = backup.item.as_ref().map(|item| item.id());
        if let Some(uri) = uri {
            let offset = None;
            let device = backup.device.id.as_deref();
            let position = backup.progress.map(|p| p.as_millis() as u32);
            client
                .start_uris_playback(uri, device, offset, position)
                .await
                .unwrap();
        }
    }
    // Pause the playback by default, unless it was playing before
    if !backup.map(|b| b.is_playing).unwrap_or(false) {
        client.pause_playback(None).await.unwrap();
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_recommendations() {
    let seed_artists = [&ArtistId::from_id("4NHQUGzhtTLFvgF5SZesLK").unwrap()];
    let seed_tracks = [&TrackId::from_id("0c6xIDDpzE81m2q797ordA").unwrap()];
    let attributes = [
        RecommendationsAttribute::MinEnergy(0.4),
        RecommendationsAttribute::MinPopularity(50),
    ];

    oauth_client()
        .await
        .recommendations(
            attributes,
            Some(seed_artists),
            None::<Vec<&str>>,
            Some(seed_tracks),
            Some(&Market::Country(Country::UnitedStates)),
            Some(10),
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_repeat() {
    let client = oauth_client().await;

    // Saving the previous state to restore it later
    let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    client.repeat(&RepeatState::Off, None).await.unwrap();

    if let Some(backup) = backup {
        client.repeat(&backup.repeat_state, None).await.unwrap()
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_album() {
    let query = "album:arrival artist:abba";
    oauth_client()
        .await
        .search(query, &SearchType::Album, None, None, Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_artist() {
    let query = "tania bowra";
    oauth_client()
        .await
        .search(
            query,
            &SearchType::Artist,
            Some(&Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            Some(0),
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_playlist() {
    let query = "\"doom metal\"";
    oauth_client()
        .await
        .search(
            query,
            &SearchType::Playlist,
            Some(&Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            Some(0),
        )
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_track() {
    let query = "abba";
    oauth_client()
        .await
        .search(
            query,
            &SearchType::Track,
            Some(&Market::Country(Country::UnitedStates)),
            None,
            Some(10),
            Some(0),
        )
        .await
        .unwrap();
}

// This also tests percentage signs in search queries to avoid regressions of
// https://github.com/ramsayleung/rspotify/issues/141
#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_show() {
    let query = "99% invisible";
    oauth_client()
        .await
        .search(query, &SearchType::Show, None, None, None, Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_seek_track() {
    let client = oauth_client().await;

    // Saving the previous state to restore it later
    let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    client.seek_track(25000, None).await.unwrap();

    if let Some(CurrentPlaybackContext {
        progress: Some(progress),
        ..
    }) = backup
    {
        client
            .seek_track(progress.as_millis() as u32, None)
            .await
            .unwrap();
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_shuffle() {
    let client = oauth_client().await;

    // Saving the previous state to restore it later
    let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    client.shuffle(true, None).await.unwrap();

    if let Some(backup) = backup {
        client.shuffle(backup.shuffle_state, None).await.unwrap();
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_user_follow_artist() {
    let client = oauth_client().await;
    let artists = [
        &ArtistId::from_id("74ASZWbe4lXaubB36ztrGX").unwrap(),
        &ArtistId::from_id("08td7MxkoHQkXnWAYD8d6Q").unwrap(),
    ];

    client.user_follow_artists(artists).await.unwrap();
    client.user_unfollow_artists(artists).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_user_follow_users() {
    let client = oauth_client().await;
    let users = [
        &UserId::from_id("exampleuser01").unwrap(),
        &UserId::from_id("john").unwrap(),
    ];

    client.user_follow_users(users).await.unwrap();
    client.user_unfollow_users(users).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_user_follow_playlist() {
    let client = oauth_client().await;
    let playlist_id = PlaylistId::from_id("2v3iNvBX8Ay1Gt2uXtUKUT").unwrap();

    client
        .playlist_follow(&playlist_id, Some(true))
        .await
        .unwrap();

    client.playlist_unfollow(&playlist_id).await.unwrap();
}

#[maybe_async]
async fn check_playlist_create(client: &AuthCodeSpotify) -> FullPlaylist {
    let user = client.me().await.unwrap();
    let name = "A New Playlist";

    // First creating the base playlist over which the tests will be ran
    let playlist = client
        .user_playlist_create(&user.id, name, Some(false), None, None)
        .await
        .unwrap();

    // Making sure that the playlist has been added to the user's profile
    let fetched_playlist = client
        .user_playlist(&user.id, Some(&playlist.id), None)
        .await
        .unwrap();
    assert_eq!(playlist.id, fetched_playlist.id);
    let user_playlists = fetch_all(client.user_playlists(&user.id)).await;
    let current_user_playlists = fetch_all(client.current_user_playlists()).await;
    assert_eq!(user_playlists.len(), current_user_playlists.len());

    // Modifying the playlist details
    let name = "A New Playlist-update";
    let description = "A random description";
    client
        .playlist_change_detail(
            &playlist.id,
            Some(name),
            Some(true),
            Some(description),
            Some(false),
        )
        .await
        .unwrap();

    playlist
}

#[maybe_async]
async fn check_num_tracks(client: &AuthCodeSpotify, playlist_id: &PlaylistId, num: i32) {
    let fetched_tracks = fetch_all(client.playlist_items(playlist_id, None, None)).await;
    assert_eq!(fetched_tracks.len() as i32, num);
}

#[maybe_async]
async fn check_playlist_tracks(client: &AuthCodeSpotify, playlist: &FullPlaylist) {
    let many_track_ids = [
        "spotify:track:5F5LlqM7TC4On8aSTNQ708",
        "spotify:track:6p6uByaFRKLblH2deQQN1R",
        "spotify:track:460AIngb50zaaAwx88i5tU",
        "spotify:track:0FuyNZ6v3M6fQC6Vn6uRmx",
        "spotify:track:3PLwHMVcnwOpB3wlIyhz1w",
        "spotify:track:0PI8YPuWLuBB0zlfPRsb0f",
        "spotify:track:6ZlmOahw11fPit1cfZp3Sz",
        "spotify:track:0xoIRdIMQmmgC7Jsklz7SW",
        "spotify:track:0EH6RZdJ4gnh1rsnHjakrI",
        "spotify:track:62PNy4tLDEwZZCFR0sm60B",
        "spotify:track:5wvZuPV09LmPQxivk0np4T",
        "spotify:track:0oIsNC7YcF48ozlsBacKRY",
        "spotify:track:4VC17AsjxU6sMmN15bYB1B",
        "spotify:track:6OBxDWinlB7U69GJaU4Vl7",
        "spotify:track:5V4kzJFdwb3go8Khdg8iup",
        "spotify:track:292TrokiRkPzSrZ59BPvYm",
        "spotify:track:26vNIXrv6BI6l0gHHk81Hg",
        "spotify:track:7cWBQY8BF6neLZNqsNzAp1",
        "spotify:track:6iYeSyDFfP4Mg5laSXxpeG",
        "spotify:track:1XY65rHJS742fF7hDBskDr",
        "spotify:track:70tGEIwJB5woeqIlnYHH5D",
        "spotify:track:2AGvQwCaZwTO2Jmd9ATgHP",
        "spotify:track:6wIKwsRC9DEmfAllgBIGI0",
        "spotify:track:0xd8BPE9x80Eaf8CWGusRA",
        "spotify:track:3mcMFQcn0RhjuAcCyTITR3",
        "spotify:track:6V92c2sNQz5zRtky1MkPv7",
        "spotify:track:130MSkElbCeCKHZLs2xB71",
        "spotify:track:4WZwDceyzLf3TBCdVWVZfF",
        "spotify:track:2mZps6eeN4oYVrzNJRfEQl",
        "spotify:track:1z79pqZqUoBzPwkvulT7zX",
        "spotify:track:3fzyPZrU8wpJR67eJ0BpLa",
        "spotify:track:2WAjGVNwKFZAxeKovo7OHR",
        "spotify:track:3fd3ceryGrscyHzjBrqbhj",
        "spotify:track:2L0WyaIQSU9WVwPMLpnLJi",
        "spotify:track:2tENFE3ujnaLOP5UM9fIpP",
        "spotify:track:0hOMvjHWYHaLXTTLMkjMNn",
        "spotify:track:21MW2uNHXoMnd67KcQnOUQ",
        "spotify:track:438sTebGykD66NV578Co40",
        "spotify:track:3j3t2KzhUx5PDPqzvGONyy",
        "spotify:track:1UWCMN9hGMcD8lm2dlaqsM",
        "spotify:track:6NV4MvIuQHjnl0fE9km1Y6",
        "spotify:track:09X7TIIgN3RS8nLHzrDBDN",
        "spotify:track:3S8pudUsTC4oNeXJxAZQyu",
        "spotify:track:2ynEUxUfifmNVcS7roMR9m",
        "spotify:track:1eSdKaEBqHARKUD1HTxWlU",
        "spotify:track:6hbM2VUWfHtbxnp6a35k9P",
        "spotify:track:4U3SGkIeLdOI0e5nJ8xqfz",
        "spotify:track:5AhEuMxTCTihSherpyXwA8",
        "spotify:track:5p36wI3HIbtxGJqgW9IfWV",
        "spotify:track:2imUtxBhnnecR5Bed3sTg9",
        "spotify:track:22v5K7N5uZoCbFJGVemJvA",
        "spotify:track:1uLezU4Q25S9XfJoMcLOkc",
        "spotify:track:3qCach4fkarjb4nHoSZBAT",
        "spotify:track:5dXiqfx1SBBIXLmoo7EGFY",
        "spotify:track:2kC5QE8R6M2x0jp5m3vVq2",
        "spotify:track:5UQQPiO4o7uBruwANgNMzT",
        "spotify:track:3gCvOeBnNrIN6jGPazydD4",
        "spotify:track:3Z1pGqeKSz2hTBlHpPophv",
        "spotify:track:322xMrek7Oye4t7s23u7cM",
        "spotify:track:1l6Smf5KiQAljgggvHwvOm",
        "spotify:track:1h8RChLQSntaQrwW8uNT85",
        "spotify:track:0MwTwnIxzBDvqi9unDzULA",
        "spotify:track:3MP5IoFCUGJFLDTH5H9hK0",
        "spotify:track:1203nRC7IhzdpJMe06tfos",
        "spotify:track:3b90xbd4Hmk2TUQkRTw1MH",
        "spotify:track:0qzdtjtQGxBwusN0Romg3l",
        "spotify:track:40g48eSbs8g3EN1LLpd6mm",
        "spotify:track:55mdqimXLmIBR5ZNF5YtOr",
        "spotify:track:1usHkrG8vZ9ZxIpsp6v94E",
        "spotify:track:4cEsN4TO0fSySeAzxNNQzj",
        "spotify:track:02CPZ6zrZETvdbnJs5z7xB",
        "spotify:track:0eoY80QymwM6nQ61mmfqmU",
        "spotify:track:2YCfjYSMpEiHh22qoVnR57",
        "spotify:track:76P5doQZFQQUZjYgeyzo8v",
        "spotify:track:35bQ4rel7GNSjY8bTSioXQ",
        "spotify:track:1OoUh2tv0oNHcbvXn7hx8B",
        "spotify:track:5z9HMBVT4lvmhIU51GbiDI",
        "spotify:track:1NA0DUu1I0pcPRcpg3aO87",
        "spotify:track:7dOo4iQERaNrwgGS8Zgeub",
        "spotify:track:3nwnKXCnznll8MhCI5pkDb",
        "spotify:track:1BBacoPQkJTyD9udpbIgAa",
        "spotify:track:66aVNocFLApjTfudNYVEiF",
        "spotify:track:68RA8ykuWDwTgAn7ojJ5Kc",
        "spotify:track:2BrYwGeLTN2MBxDU1ZaHnS",
        "spotify:track:2dQxDxpUmXQwkdyorjrD8x",
        "spotify:track:5YMLp3LDDY0T5nRaLbLzOH",
        "spotify:track:41dRP2ShBhLcqfXZR79q7u",
        "spotify:track:44iLbJnC8K5pZK56sjSjrn",
        "spotify:track:2WrH04r76ta9WZXhSdidI8",
        "spotify:track:0pPIIewNGzWHaxQ9h0sHLi",
        "spotify:track:0dAbnZaG3iH64VD3CY0Ghn",
        "spotify:track:3uyR6xvl4wgTmWIAx2J0nu",
        "spotify:track:2K3qgAktdl7qYf767Zqsa0",
        "spotify:track:1BalrpO4Jm1U9OEYjfRHS3",
        "spotify:track:6kOSnyXVOLI8pApZLfQKvG",
        "spotify:track:7DB3z4Xo8ix6vVaccEyv9h",
        "spotify:track:6saT45aGfPjGvcZAEDwHuV",
        "spotify:track:1zhD9NcPGOABy2fqUl2781",
        "spotify:track:6f7FvVoOD9uKlofm4jLXn6",
        "spotify:track:3c3WtmLV34X9XQkT1n8jn5",
        "spotify:track:2gybgezceHzfhyiYCrOeEW",
        "spotify:track:4RxMGnlScLGqIcwkjZKsEV",
        "spotify:track:13Q866vT8Ses7iEtolBRrZ",
        "spotify:track:4Z5s1LESqgQsOPl36VewAh",
        "spotify:track:0OXPlvymmkGdhZ8i3AU4mu",
        "spotify:track:0BI3nY3K6ip7sRzB4oWGzt",
        "spotify:track:2gNEhoP1WmyZOz4xfWkiwX",
        "spotify:track:1WVE6fsHpmoM6i2jw1FjYH",
        "spotify:track:7w9pj2cU3Ukq4Ia8Pq8J9K",
        "spotify:track:27M5yORKNX6iyF98xFpuEh",
        "spotify:track:719MjPs3Y3fNymStOD8aMy",
        "spotify:track:6AhINsGEM5M2GtnQ2Zxzeq",
        "spotify:track:4UMj6HAftBnEs6mm1sSIWG",
        "spotify:track:6XIlWm6lvkBbeoTakCkzoJ",
        "spotify:track:4uiG8a0H9CYHRGI5EbeLj0",
        "spotify:track:1RPCxEvotPwTW8xJhocOmV",
        "spotify:track:3wdTUSziTXa4sTaTMURsQP",
        "spotify:track:243I2WdcGsG3d6ydsNcaam",
        "spotify:track:1KPqe4yuggQoMzlHt4JLmj",
        "spotify:track:55TVHFjh5ZaAt8dOZaecZo",
        "spotify:track:0cHBeAXAtnQ37lv8X8mbxv",
        "spotify:track:2Adw90S7yANAgtMhiDXFE1",
        "spotify:track:5oKjKxXLzNfqiPelCb2mz7",
        "spotify:track:3s6jJkYwFwEWuRa4JwvJ7n",
        "spotify:track:3oask4B0HkR3blqgwglM4r",
        "spotify:track:0vDLTM0N18UugNYgt8STIw",
        "spotify:track:2VPc5B4x6FMaOxGHeTPit4",
        "spotify:track:6bM1z5bhEEsxttO5QbrZtC",
        "spotify:track:2MhSc9MBA9Mfk7bARgtwWn",
        "spotify:track:1AVCY0tHengz0OAoLIj36C",
        "spotify:track:2LcRmvHA1lwHwaFtzbm0d2",
        "spotify:track:6eQzqPniS4bBZfG5ej2TUL",
        "spotify:track:1poyDHeaV8WAf29cR1Zk3L",
        "spotify:track:0Cq6dYbYdUYXEtxVfhSZ52",
        "spotify:track:5ekgk9E5jCzxqSUJxcEAVk",
        "spotify:track:7qYZIYFVEk2lQGIg3ZB0fA",
        "spotify:track:7aI6uXhu9Nl1oXGLIcu5Y8",
        "spotify:track:1Jz5mZyOMeSogVpdvFKhKB",
        "spotify:track:2BhOueZk9NQ269XuD2tnMg",
        "spotify:track:19WGviHv5DKhvoqqJmrKRO",
        "spotify:track:3Zdt2zbenPVsGDQfOGnbgP",
        "spotify:track:4YS7cziBoUrO0Dqtczy8ls",
        "spotify:track:5nVkaIXaqQtoMX4V1jncdO",
        "spotify:track:6kZmXmoKAqzh7Dd3ttqkvi",
        "spotify:track:3VqhFQevhOb4xDaLFeyEX0",
        "spotify:track:0XDJ0moke0YRNG7sOup0MB",
        "spotify:track:2H4lvngSfVLM2DxUqLZg5d",
        "spotify:track:3pDDWhOWCYPPCjp7GIzGzJ",
        "spotify:track:3plBrR6SS2rsJVRQDUuMJZ",
        "spotify:track:5YB5OP1DF5LeLPP4kvY1r2",
        "spotify:track:19yWSIWamGXck8GqPlwnbI",
        "spotify:track:1SNpVIHlYIwmkOG7o7dAGl",
        "spotify:track:3QtJFO9oOxF39ba6UuqUQG",
        "spotify:track:20qaixxAwIDtdraAd9Wbtq",
        "spotify:track:76Q0uXQEAJ79nU58HgySa8",
        "spotify:track:6Ad4PTD20fKOvRJt5GN2Gi",
        "spotify:track:5TDny4zqmIUHbPqOU2piRP",
        "spotify:track:1EGeBH0HH2tQSm3COh89wI",
        "spotify:track:1eb1pBq8imZoE9m7I2JTaM",
        "spotify:track:5urFDgiRTQmE2rL5b5GvY9",
        "spotify:track:54QxrEFZESIfuW7Ex7YwSd",
        "spotify:track:3rrubeU6np8Yeya9mwtedg",
        "spotify:track:0MOOHqSaflxhjMuw3XhhgU",
        "spotify:track:4tDzwyAQAbKX6Kxlzuyve5",
        "spotify:track:7eTsm0Msg9ddxgK65ZUs4K",
        "spotify:track:5UeLuozsBQbBj1MmB6joHz",
        "spotify:track:3Ov5GS35RJi2Sa5j2S7ai5",
        "spotify:track:4O0hFdFxhsX7bjc13zunsb",
        "spotify:track:3vTJP27ZGQMhrBqmXICfGB",
        "spotify:track:7zUUo57PVdasUxG4Lrt37K",
        "spotify:track:1KibqiW18YSYSCnhorGAh2",
        "spotify:track:4wMMiSEzBgqpkTX4XyTNOU",
        "spotify:track:7iFsFe1dvApvOmqqbPyW9t",
        "spotify:track:2KzOl8UDgssvilD8c0cD1j",
        "spotify:track:6C4Wugot6Cfw3H8VkiwIjD",
        "spotify:track:3fdtNgNiwcwzluIQHU0l0N",
        "spotify:track:7oLfES7gAl8sWILYrIfqSd",
        "spotify:track:51CeWFqX7G69e98uHWH7qo",
        "spotify:track:6NK1H3w4eY3JpjhysYBlmf",
        "spotify:track:73YXGkN7g9oljiJKRNtgYs",
        "spotify:track:2uquC1imZKe4GkfP7CuKNa",
        "spotify:track:7oIie45AkGl7IA4yBu898V",
        "spotify:track:0Lg4Vv8yT2kIu13MiCFFRH",
        "spotify:track:05OUzuprHAYW7qx6WMPyAb",
        "spotify:track:5be5sXDDKPAHIVz2S59v0P",
        "spotify:track:2UfwzZdmafOdb03D5qhc2w",
        "spotify:track:2leY1jnZ1B9OUjvjMB4MJX",
        "spotify:track:26I6M5K8pGqYo72huex9XH",
        "spotify:track:5S7vkxR89rq8kxx3BQnU3R",
        "spotify:track:428BWdDsS3N5GlphJGPgZL",
        "spotify:track:1E50pMxLo0YL4MGefwrN2O",
        "spotify:track:0OJ3iSa0gHbmar91N80vVa",
        "spotify:track:4jseVFPy24FCzXPGzIZyWY",
        "spotify:track:1ncOLMcTu9rxE18fmcGwnd",
        "spotify:track:7okXi589OVY6VFszHe6JO5",
        "spotify:track:1dLz1RKcIqw5bms62CY8dQ",
        "spotify:track:7KzXghoY2PTH7iqexeKHmM",
        "spotify:track:5jjDPdHqxeHZMvIbSqCrX8",
    ];

    let track_ids: Vec<TrackId> = many_track_ids
        .iter()
        .map(|uri| TrackId::from_uri(uri).unwrap())
        .collect();

    let playable_ids: Vec<&dyn PlayableId> =
        track_ids.iter().map(|id| id as &dyn PlayableId).collect();

    // The tracks in the playlist, some of them repeated
    let _tracks: [&dyn PlayableId; 4] = [
        &TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap(),
        &TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap(),
        &EpisodeId::from_uri("spotify/episode/381XrGKkcdNkLwfsQ4Mh5y").unwrap(),
        &EpisodeId::from_uri("spotify/episode/6O63eWrfWPvN41CsSyDXve").unwrap(),
    ];

    // Firstly adding some tracks
    client
        .playlist_add_items(&playlist.id, playable_ids.into_iter(), None)
        .await
        .unwrap();
    check_num_tracks(client, &playlist.id, track_ids.len() as i32).await;

    // Reordering some tracks
    client
        .playlist_reorder_items(&playlist.id, Some(0), Some(3), Some(2), None)
        .await
        .unwrap();
    // Making sure the number of tracks is the same
    check_num_tracks(client, &playlist.id, track_ids.len() as i32).await;

    // Replacing the tracks
    let replaced_tracks: [&dyn PlayableId; 7] = [
        &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        &TrackId::from_uri("spotify:track:1301WleyT98MSxVHPZCA6M").unwrap(),
        &EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
        &TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap(),
        &EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap(),
        &TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap(),
    ];
    client
        .playlist_replace_items(&playlist.id, replaced_tracks)
        .await
        .unwrap();
    // Making sure the number of tracks is updated
    check_num_tracks(client, &playlist.id, replaced_tracks.len() as i32).await;

    // Removes a few specific tracks
    let tracks = [
        ItemPositions {
            id: &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
            positions: &[0],
        },
        ItemPositions {
            id: &TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap(),
            positions: &[4, 6],
        },
    ];
    client
        .playlist_remove_specific_occurrences_of_items(&playlist.id, tracks, None)
        .await
        .unwrap();
    // Making sure three tracks were removed
    check_num_tracks(client, &playlist.id, replaced_tracks.len() as i32 - 3).await;

    // Removes all occurrences of two tracks
    let to_remove: [&dyn PlayableId; 2] = [
        &TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        &EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
    ];
    client
        .playlist_remove_all_occurrences_of_items(&playlist.id, to_remove, None)
        .await
        .unwrap();
    // Making sure two more tracks were removed
    check_num_tracks(client, &playlist.id, replaced_tracks.len() as i32 - 5).await;
}

#[maybe_async]
async fn check_playlist_follow(client: &AuthCodeSpotify, playlist: &FullPlaylist) {
    let user_ids = [
        &UserId::from_id("possan").unwrap(),
        &UserId::from_id("elogain").unwrap(),
    ];

    // It's a new playlist, so it shouldn't have any followers
    let following = client
        .playlist_check_follow(&playlist.id, &user_ids)
        .await
        .unwrap();
    assert_eq!(following, vec![false, false]);

    // Finally unfollowing the playlist in order to clean it up
    client.playlist_unfollow(&playlist.id).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_playlist() {
    let client = oauth_client().await;

    let playlist = check_playlist_create(&client).await;
    check_playlist_tracks(&client, &playlist).await;
    check_playlist_follow(&client, &playlist).await;
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_volume() {
    let client = oauth_client().await;

    // Saving the previous state to restore it later
    let backup = client.current_playback(None, None::<&[_]>).await.unwrap();

    client.volume(78, None).await.unwrap();

    if let Some(CurrentPlaybackContext {
        device: Device {
            volume_percent: Some(volume),
            ..
        },
        ..
    }) = backup
    {
        client.volume(volume as u8, None).await.unwrap();
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_add_queue() {
    // NOTE: unfortunately it's impossible to revert this test

    let birdy_uri = TrackId::from_uri("spotify:track:6rqhFgbbKwnb9MLmUQDhG6").unwrap();
    oauth_client()
        .await
        .add_item_to_queue(&birdy_uri, None)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_get_several_shows() {
    let shows = [
        &ShowId::from_id("5CfCWKI5pZ28U0uOzXkDHe").unwrap(),
        &ShowId::from_id("5as3aKmN2k11yfDDDSrvaZ").unwrap(),
    ];

    oauth_client()
        .await
        .get_several_shows(shows, None)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_get_several_episodes() {
    let episodes = [
        &EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
        &EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap(),
    ];
    oauth_client()
        .await
        .get_several_episodes(episodes, None)
        .await
        .unwrap();
}

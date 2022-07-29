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
            Some(Market::Country(Country::UnitedStates)),
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
            Some(Market::Country(Country::UnitedStates)),
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
        AlbumId::from_id("6akEvsycLGftJxYudPjmqK").unwrap(),
        AlbumId::from_id("628oezqK2qfmCjC6eXNors").unwrap(),
    ];

    let client = oauth_client().await;

    // First adding the albums
    client
        .current_user_saved_albums_add(album_ids.iter().map(AlbumId::as_ref))
        .await
        .unwrap();

    // Making sure the new albums appear
    let all_albums = fetch_all(client.current_user_saved_albums(None)).await;
    // Can handle albums without available_market
    let _albums_from_token =
        fetch_all(client.current_user_saved_albums(Some(Market::FromToken))).await;
    let all_uris = all_albums
        .into_iter()
        .map(|a| a.album.id)
        .collect::<Vec<_>>();
    assert!(
        album_ids.iter().all(|item| all_uris.contains(item)),
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
        TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
        TrackId::from_uri("spotify:track:1301WleyT98MSxVHPZCA6M").unwrap(),
    ];
    client
        .current_user_saved_tracks_add(tracks_ids.iter().map(TrackId::as_ref))
        .await
        .unwrap();

    let contains = client
        .current_user_saved_tracks_contains(tracks_ids.iter().map(TrackId::as_ref))
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
        .current_user_top_artists_manual(Some(TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_current_user_top_tracks() {
    oauth_client()
        .await
        .current_user_top_tracks_manual(Some(TimeRange::ShortTerm), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_featured_playlists() {
    let now: DateTime<Utc> = Utc::now();
    oauth_client()
        .await
        .featured_playlists(None, None, Some(now), Some(10), Some(0))
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
        .new_releases_manual(Some(Market::Country(Country::Sweden)), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_new_releases_with_from_token() {
    oauth_client()
        .await
        .new_releases_manual(Some(Market::FromToken), Some(10), Some(0))
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_playback() {
    let client = oauth_client().await;
    let uris = [
        PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:2DzSjFQKetFhkFCuDWhioi").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
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
            .start_uris_playback(
                uris.iter().map(PlayableId::as_ref),
                Some(device_id),
                Some(Offset::for_position(0)),
                None,
            )
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
    let seed_artists = [ArtistId::from_id("4NHQUGzhtTLFvgF5SZesLK").unwrap()];
    let seed_tracks = [TrackId::from_id("0c6xIDDpzE81m2q797ordA").unwrap()];
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
            Some(Market::Country(Country::UnitedStates)),
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

    client.repeat(RepeatState::Off, None).await.unwrap();

    if let Some(backup) = backup {
        client.repeat(backup.repeat_state, None).await.unwrap()
    }
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_search_album() {
    let query = "album:arrival artist:abba";
    oauth_client()
        .await
        .search(query, SearchType::Album, None, None, Some(10), Some(0))
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
            SearchType::Artist,
            Some(Market::Country(Country::UnitedStates)),
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
            SearchType::Playlist,
            Some(Market::Country(Country::UnitedStates)),
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
            SearchType::Track,
            Some(Market::Country(Country::UnitedStates)),
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
        .search(query, SearchType::Show, None, None, None, Some(0))
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
        ArtistId::from_id("74ASZWbe4lXaubB36ztrGX").unwrap(),
        ArtistId::from_id("08td7MxkoHQkXnWAYD8d6Q").unwrap(),
    ];

    client
        .user_follow_artists(artists.iter().map(ArtistId::as_ref))
        .await
        .unwrap();
    client.user_unfollow_artists(artists).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_user_follow_users() {
    let client = oauth_client().await;
    let users = [
        UserId::from_id("exampleuser01").unwrap(),
        UserId::from_id("john").unwrap(),
    ];

    client
        .user_follow_users(users.iter().map(UserId::as_ref))
        .await
        .unwrap();
    client.user_unfollow_users(users).await.unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_user_follow_playlist() {
    let client = oauth_client().await;
    let playlist_id = PlaylistId::from_id("2v3iNvBX8Ay1Gt2uXtUKUT").unwrap();

    client
        .playlist_follow(playlist_id.as_ref(), Some(true))
        .await
        .unwrap();

    client.playlist_unfollow(playlist_id).await.unwrap();
}

#[maybe_async]
async fn check_playlist_create(client: &AuthCodeSpotify) -> FullPlaylist {
    let user = client.me().await.unwrap();
    let name = "A New Playlist";

    // First creating the base playlist over which the tests will be ran
    let playlist = client
        .user_playlist_create(user.id.as_ref(), name, Some(false), None, None)
        .await
        .unwrap();

    // Making sure that the playlist has been added to the user's profile
    let fetched_playlist = client
        .user_playlist(user.id.as_ref(), Some(playlist.id.as_ref()), None)
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
            playlist.id.as_ref(),
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
async fn check_num_tracks(client: &AuthCodeSpotify, playlist_id: PlaylistId<'_>, num: i32) {
    let fetched_tracks = fetch_all(client.playlist_items(&playlist_id.as_ref(), None, None)).await;
    assert_eq!(fetched_tracks.len() as i32, num);
}

#[maybe_async]
async fn check_playlist_tracks(client: &AuthCodeSpotify, playlist: &FullPlaylist) {
    // The tracks in the playlist, some of them repeated
    let tracks = [
        PlayableId::Track(TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:5iKndSu1XI74U2OZePzP8L").unwrap()),
        PlayableId::Episode(EpisodeId::from_uri("spotify/episode/381XrGKkcdNkLwfsQ4Mh5y").unwrap()),
        PlayableId::Episode(EpisodeId::from_uri("spotify/episode/6O63eWrfWPvN41CsSyDXve").unwrap()),
    ];

    // Firstly adding some tracks
    client
        .playlist_add_items(
            playlist.id.as_ref(),
            tracks.iter().map(PlayableId::as_ref),
            None,
        )
        .await
        .unwrap();
    check_num_tracks(client, playlist.id.as_ref(), tracks.len() as i32).await;

    // Reordering some tracks
    client
        .playlist_reorder_items(playlist.id.as_ref(), Some(0), Some(3), Some(2), None)
        .await
        .unwrap();
    // Making sure the number of tracks is the same
    check_num_tracks(client, playlist.id.as_ref(), tracks.len() as i32).await;

    // Replacing the tracks
    let replaced_tracks = [
        PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:1301WleyT98MSxVHPZCA6M").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap()),
        PlayableId::Track(TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap()),
    ];
    client
        .playlist_replace_items(
            playlist.id.as_ref(),
            replaced_tracks.iter().map(|t| t.as_ref()),
        )
        .await
        .unwrap();
    // Making sure the number of tracks is updated
    check_num_tracks(client, playlist.id.as_ref(), replaced_tracks.len() as i32).await;

    // Removes a few specific tracks
    let tracks = [
        ItemPositions {
            id: PlayableId::Track(
                TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap(),
            ),
            positions: &[0],
        },
        ItemPositions {
            id: PlayableId::Track(
                TrackId::from_uri("spotify:track:5m2en2ndANCPembKOYr1xL").unwrap(),
            ),
            positions: &[4, 6],
        },
    ];
    client
        .playlist_remove_specific_occurrences_of_items(playlist.id.as_ref(), tracks, None)
        .await
        .unwrap();
    // Making sure three tracks were removed
    check_num_tracks(
        client,
        playlist.id.as_ref(),
        replaced_tracks.len() as i32 - 3,
    )
    .await;

    // Removes all occurrences of two tracks
    let to_remove = vec![
        PlayableId::Track(TrackId::from_uri("spotify:track:4iV5W9uYEdYUVa79Axb7Rh").unwrap()),
        PlayableId::Episode(EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap()),
    ];
    client
        .playlist_remove_all_occurrences_of_items(playlist.id.as_ref(), to_remove, None)
        .await
        .unwrap();
    // Making sure two more tracks were removed
    check_num_tracks(
        client,
        playlist.id.as_ref(),
        replaced_tracks.len() as i32 - 5,
    )
    .await;
}

#[maybe_async]
async fn check_playlist_follow(client: &AuthCodeSpotify, playlist: &FullPlaylist) {
    let user_ids = [
        UserId::from_id("possan").unwrap(),
        UserId::from_id("elogain").unwrap(),
    ];

    // It's a new playlist, so it shouldn't have any followers
    let following = client
        .playlist_check_follow(playlist.id.as_ref(), &user_ids)
        .await
        .unwrap();
    assert_eq!(following, vec![false, false]);

    // Finally unfollowing the playlist in order to clean it up
    client
        .playlist_unfollow(playlist.id.as_ref())
        .await
        .unwrap();
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

    let birdy_uri =
        PlayableId::Track(TrackId::from_uri("spotify:track:6rqhFgbbKwnb9MLmUQDhG6").unwrap());
    oauth_client()
        .await
        .add_item_to_queue(birdy_uri, None)
        .await
        .unwrap();
}

#[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
#[ignore]
async fn test_get_several_shows() {
    let shows = [
        ShowId::from_id("5CfCWKI5pZ28U0uOzXkDHe").unwrap(),
        ShowId::from_id("5as3aKmN2k11yfDDDSrvaZ").unwrap(),
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
        EpisodeId::from_id("0lbiy3LKzIY2fnyjioC11p").unwrap(),
        EpisodeId::from_id("4zugY5eJisugQj9rj8TYuh").unwrap(),
    ];
    oauth_client()
        .await
        .get_several_episodes(episodes, None)
        .await
        .unwrap();
}

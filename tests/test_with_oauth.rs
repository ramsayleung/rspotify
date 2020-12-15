//! Most of tests currently require a Spotify Premium account where the tests
//! can be ran, which will be ignored in the CI for now. The tests are written
//! so that no user data is modified.
//!
//! You can run all of them with:
//!
//! ```
//! $ cargo test --features=cli,env-file -- --ignored
//! ```
//!
//! The access token must be obtained previously, and this test file will try
//! to authenticate with the access token from the `RSPOTIFY_ACCESS_TOKEN`
//! environment variable or the refresh token from `RSPOTIFY_REFRESH_TOKEN`
//! (these tokens must have been generated for all available scopes, see
//! the `oauth_tokens` example).

mod common;

use common::maybe_async_test;
use rspotify::client::{Spotify, SpotifyBuilder};
use rspotify::model::offset::for_position;
use rspotify::model::{Country, RepeatState, SearchType, TimeRange};
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder, TokenBuilder};

use std::env;

use chrono::prelude::*;
use maybe_async::maybe_async;
use serde_json::map::Map;

/// Generating a new OAuth client for the requests.
#[maybe_async]
pub async fn oauth_client() -> Spotify {
    if let Ok(access_token) = env::var("RSPOTIFY_ACCESS_TOKEN") {
        let tok = TokenBuilder::default()
            .access_token(access_token)
            .build()
            .unwrap();

        SpotifyBuilder::default().token(tok).build().unwrap()
    } else if let Ok(refresh_token) = env::var("RSPOTIFY_REFRESH_TOKEN") {
        // The credentials must be available in the environment. Enable
        // `env-file` in order to read them from an `.env` file.
        let creds = CredentialsBuilder::from_env().build().unwrap();

        // Using every possible scope
        let oauth = OAuthBuilder::from_env()
            .scope(
                "user-read-email user-read-private user-top-read \
                 user-read-recently-played user-follow-read user-library-read \
                 user-read-currently-playing user-read-playback-state \
                 user-read-playback-position playlist-read-collaborative \
                 playlist-read-private user-follow-modify user-library-modify \
                 user-modify-playback-state playlist-modify-public \
                 playlist-modify-private ugc-image-upload",
            )
            .build()
            .unwrap();

        let mut spotify = SpotifyBuilder::default()
            .credentials(creds)
            .oauth(oauth)
            .build()
            .unwrap();

        spotify.refresh_user_token(&refresh_token).await.unwrap();

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
#[maybe_async_test]
#[ignore]
async fn test_categories() {
    oauth_client()
        .await
        .categories(None, Some(Country::UnitedStates), 10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_category_playlists() {
    oauth_client()
        .await
        .category_playlists("pop", Some(Country::UnitedStates), 10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_playback() {
    oauth_client()
        .await
        .current_playback(None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_playing() {
    oauth_client()
        .await
        .current_playing(None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_followed_artists() {
    oauth_client()
        .await
        .current_user_followed_artists(10, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_playing_track() {
    oauth_client()
        .await
        .current_user_playing_track()
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_playlists() {
    oauth_client()
        .await
        .current_user_playlists(10, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_recently_played() {
    oauth_client()
        .await
        .current_user_recently_played(10)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_albums_add() {
    let mut album_ids = vec![];
    let album_id1 = "6akEvsycLGftJxYudPjmqK";
    let album_id2 = "628oezqK2qfmCjC6eXNors";
    album_ids.push(album_id2);
    album_ids.push(album_id1);
    oauth_client()
        .await
        .current_user_saved_albums_add(album_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_albums_delete() {
    let mut album_ids = vec![];
    let album_id1 = "6akEvsycLGftJxYudPjmqK";
    let album_id2 = "628oezqK2qfmCjC6eXNors";
    album_ids.push(album_id2);
    album_ids.push(album_id1);
    oauth_client()
        .await
        .current_user_saved_albums_delete(album_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_albums() {
    oauth_client()
        .await
        .current_user_saved_albums(10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_tracks_add() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    oauth_client()
        .await
        .current_user_saved_tracks_add(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_tracks_contains() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    oauth_client()
        .await
        .current_user_saved_tracks_contains(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_tracks_delete() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    oauth_client()
        .await
        .current_user_saved_tracks_delete(tracks_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_saved_tracks() {
    oauth_client()
        .await
        .current_user_saved_tracks(10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_top_artists() {
    oauth_client()
        .await
        .current_user_top_artists(10, 0, TimeRange::ShortTerm)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_current_user_top_tracks() {
    oauth_client()
        .await
        .current_user_top_tracks(10, 0, TimeRange::ShortTerm)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_device() {
    oauth_client().await.device().await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_featured_playlists() {
    let now: DateTime<Utc> = Utc::now();
    oauth_client()
        .await
        .featured_playlists(None, None, Some(now), 10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_me() {
    oauth_client().await.me().await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_new_releases() {
    oauth_client()
        .await
        .new_releases(Some(Country::Sweden), 10, 0)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_next_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    oauth_client()
        .await
        .next_track(Some(device_id))
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_pause_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    oauth_client()
        .await
        .pause_playback(Some(device_id))
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_previous_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    oauth_client()
        .await
        .previous_track(Some(device_id))
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_recommendations() {
    let mut payload = Map::new();
    let seed_artists = vec!["4NHQUGzhtTLFvgF5SZesLK".to_owned()];
    let seed_tracks = vec!["0c6xIDDpzE81m2q797ordA".to_owned()];
    payload.insert("min_energy".to_owned(), 0.4.into());
    payload.insert("min_popularity".to_owned(), 50.into());
    oauth_client()
        .await
        .recommendations(
            Some(seed_artists),
            None,
            Some(seed_tracks),
            10,
            Some(Country::UnitedStates),
            &payload,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_repeat() {
    oauth_client()
        .await
        .repeat(RepeatState::Context, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_search_album() {
    let query = "album:arrival artist:abba";
    oauth_client()
        .await
        .search(query, SearchType::Album, 10, 0, None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_search_artist() {
    let query = "tania bowra";
    oauth_client()
        .await
        .search(
            query,
            SearchType::Artist,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_search_playlist() {
    let query = "\"doom metal\"";
    oauth_client()
        .await
        .search(
            query,
            SearchType::Playlist,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_search_track() {
    let query = "abba";
    oauth_client()
        .await
        .search(
            query,
            SearchType::Track,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_seek_track() {
    oauth_client().await.seek_track(25000, None).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_shuffle() {
    oauth_client().await.shuffle(true, None).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_start_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    let uris = vec!["spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_owned()];
    oauth_client()
        .await
        .start_playback(Some(device_id), None, Some(uris), for_position(0), None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_transfer_playback() {
    let device_id = "74ASZWbe4lXaubB36ztrGX";
    oauth_client()
        .await
        .transfer_playback(device_id, true)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_follow_artist() {
    let mut artists = vec![];
    let artist_id1 = "74ASZWbe4lXaubB36ztrGX";
    let artist_id2 = "08td7MxkoHQkXnWAYD8d6Q";
    artists.push(artist_id2);
    artists.push(artist_id1);
    oauth_client()
        .await
        .user_follow_artists(artists)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_unfollow_artist() {
    let mut artists = vec![];
    let artist_id1 = "74ASZWbe4lXaubB36ztrGX";
    let artist_id2 = "08td7MxkoHQkXnWAYD8d6Q";
    artists.push(artist_id2);
    artists.push(artist_id1);
    oauth_client()
        .await
        .user_unfollow_artists(artists)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_follow_users() {
    let mut users = vec![];
    let user_id1 = "exampleuser01";
    users.push(user_id1);
    oauth_client().await.user_follow_users(users).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_unfollow_users() {
    let mut users = vec![];
    let user_id1 = "exampleuser01";
    users.push(user_id1);
    oauth_client()
        .await
        .user_unfollow_users(users)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_add_tracks() {
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    tracks_ids.push(track_id1);
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    oauth_client()
        .await
        .playlist_add_tracks(playlist_id, tracks_ids, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_change_detail() {
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let playlist_name = "A New Playlist-update";
    oauth_client()
        .await
        .playlist_change_detail(playlist_id, Some(playlist_name), Some(false), None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_check_follow() {
    let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
    let mut user_ids: Vec<String> = vec![];
    let user_id1 = String::from("possan");
    user_ids.push(user_id1);
    let user_id2 = String::from("elogain");
    user_ids.push(user_id2);
    oauth_client()
        .await
        .playlist_check_follow(playlist_id, &user_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_playlist_create() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_name = "A New Playlist";
    oauth_client()
        .await
        .user_playlist_create(user_id, playlist_name, false, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_follow_playlist() {
    let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
    oauth_client()
        .await
        .playlist_follow(playlist_id, true)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_recorder_tracks() {
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let range_start = 0;
    let insert_before = 1;
    let range_length = 1;
    oauth_client()
        .await
        .playlist_reorder_tracks(playlist_id, range_start, range_length, insert_before, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_remove_all_occurrences_of_tracks() {
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    oauth_client()
        .await
        .playlist_remove_all_occurrences_of_tracks(playlist_id, tracks_ids, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_remove_specific_occurrences_of_tracks() {
    let playlist_id = String::from("5jAOgWXCBKuinsGiZxjDQ5");
    let mut tracks = vec![];
    let mut map1 = Map::new();
    let mut position1 = vec![];
    position1.push(0);
    position1.push(3);
    map1.insert(
        "uri".to_string(),
        "spotify:track:4iV5W9uYEdYUVa79Axb7Rh".into(),
    );
    map1.insert("position".to_string(), position1.into());
    tracks.push(map1);
    let mut map2 = Map::new();
    let mut position2 = vec![];
    position2.push(7);
    map2.insert(
        "uri".to_string(),
        "spotify:track:1301WleyT98MSxVHPZCA6M".into(),
    );
    map2.insert("position".to_string(), position2.into());
    tracks.push(map2);
    oauth_client()
        .await
        .playlist_remove_specific_occurrences_of_tracks(&playlist_id, tracks, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_replace_tracks() {
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    oauth_client()
        .await
        .playlist_replace_tracks(playlist_id, tracks_ids)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_playlist() {
    let user_id = "spotify";
    let mut playlist_id = String::from("59ZbFPES4DQwEjBpWHzrtC");
    oauth_client()
        .await
        .user_playlist(user_id, Some(&mut playlist_id), None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_user_playlists() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    oauth_client()
        .await
        .user_playlists(user_id, Some(10), None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_tracks() {
    let playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
    oauth_client()
        .await
        .playlist_tracks(&playlist_id, None, Some(2), None, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_playlist_unfollow() {
    let playlist_id = "65V6djkcVRyOStLd8nza8E";
    oauth_client()
        .await
        .playlist_unfollow(playlist_id)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_volume() {
    oauth_client().await.volume(78, None).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_add_queue() {
    let birdy_uri = String::from("spotify:track:6rqhFgbbKwnb9MLmUQDhG6");
    oauth_client()
        .await
        .add_item_to_queue(birdy_uri, None)
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
#[ignore]
async fn test_get_several_shows() {
    oauth_client()
        .await
        .get_several_shows(
            vec!["5CfCWKI5pZ28U0uOzXkDHe", "5as3aKmN2k11yfDDDSrvaZ"],
            None,
        )
        .await
        .unwrap();
}

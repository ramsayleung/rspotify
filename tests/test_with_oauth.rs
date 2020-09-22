//! These tests currently require user interaction to authenticate an account
//! where the tests can be ran, so they are ran manually instead of with
//! Continuous Integration for now. The tests are written so that no account
//! data is modified.
//!
//! You can run them manually with `cargo test -- --ignored`.

use async_once::AsyncOnce;
use chrono::prelude::*;
use lazy_static::lazy_static;
use serde_json::map::Map;

use rspotify::client::Spotify;
use rspotify::enums::{Country, RepeatState, SearchType, TimeRange};
use rspotify::model::offset::for_position;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

lazy_static! {
    // With so many tests, it's a better idea to authenticate only once at the
    // beginning. The `Spotify` instance needed here is for async requests,
    // so this uses `AsyncOnce` to work with `lazy_static`.
    static ref CLIENT_CREDENTIAL: AsyncOnce<SpotifyClientCredentials> = AsyncOnce::new(async {
        // Using every possible scope
        let mut oauth = SpotifyOAuth::default()
            .scope(
                "user-read-email user-read-private user-top-read
                 user-read-recently-played user-follow-read user-library-read
                 user-read-currently-playing user-read-playback-state
                 user-read-playback-position playlist-read-collaborative
                 playlist-read-private user-follow-modify user-library-modify
                 user-modify-playback-state playlist-modify-public
                 playlist-modify-private ugc-image-upload"
            )
            .build();

        let token = get_token(&mut oauth).await.unwrap();
        SpotifyClientCredentials::default()
            .token_info(token)
            .build()
    });
}

// Even easier to use and change in the future by using a macro.
async fn async_client() -> Spotify {
    let creds = CLIENT_CREDENTIAL.get().await;
    Spotify::default().client_credentials_manager(creds).build()
}

#[tokio::test]
#[ignore]
async fn test_categories() {
    let categories = async_client()
        .await
        .categories(None, Some(Country::UnitedStates), 10, 0)
        .await;
    assert!(categories.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_playback() {
    let context = async_client().await.current_playback(None, None).await;
    assert!(context.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_playing() {
    let context = async_client().await.current_playing(None, None).await;
    assert!(context.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_followed_artists() {
    let artists = async_client()
        .await
        .current_user_followed_artists(10, None)
        .await;
    assert!(artists.is_ok())
}

#[tokio::test]
#[ignore]
async fn test_current_user_playing_track() {
    let playing = async_client().await.current_user_playing_track().await;
    assert!(playing.is_ok())
}

#[tokio::test]
#[ignore]
async fn test_current_user_playlists() {
    let playlists = async_client().await.current_user_playlists(10, None).await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_recently_played() {
    let history = async_client().await.current_user_recently_played(10).await;
    assert!(history.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums_add() {
    let mut album_ids = vec![];
    let album_id1 = "6akEvsycLGftJxYudPjmqK";
    let album_id2 = "628oezqK2qfmCjC6eXNors";
    album_ids.push(album_id2);
    album_ids.push(album_id1);
    let result = async_client()
        .await
        .current_user_saved_albums_add(album_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums_delete() {
    let mut album_ids = vec![];
    let album_id1 = "6akEvsycLGftJxYudPjmqK";
    let album_id2 = "628oezqK2qfmCjC6eXNors";
    album_ids.push(album_id2);
    album_ids.push(album_id1);
    let result = async_client()
        .await
        .current_user_saved_albums_delete(album_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums() {
    let albums = async_client().await.current_user_saved_albums(10, 0).await;
    assert!(albums.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_add() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    let result = async_client()
        .await
        .current_user_saved_tracks_add(tracks_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_contains() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    let result = async_client()
        .await
        .current_user_saved_tracks_contains(tracks_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_delete() {
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    let result = async_client()
        .await
        .current_user_saved_tracks_delete(tracks_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks() {
    let tracks = async_client().await.current_user_saved_tracks(10, 0).await;
    assert!(tracks.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_top_artists() {
    let artist = async_client()
        .await
        .current_user_top_artists(10, 0, TimeRange::ShortTerm)
        .await;
    assert!(artist.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_current_user_top_tracks() {
    let tracks = async_client()
        .await
        .current_user_top_tracks(10, 0, TimeRange::ShortTerm)
        .await;
    assert!(tracks.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_device() {
    let devices = async_client().await.device().await;
    assert!(devices.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_featured_playlists() {
    let now: DateTime<Utc> = Utc::now();
    let playlists = async_client()
        .await
        .featured_playlists(None, None, Some(now), 10, 0)
        .await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_me() {
    let user = async_client().await.me().await;
    assert!(user.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_new_releases() {
    let albums = async_client()
        .await
        .new_releases(Some(Country::Sweden), 10, 0)
        .await;
    assert!(albums.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_next_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    let result = async_client().await.next_track(Some(device_id)).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_pause_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    let result = async_client().await.pause_playback(Some(device_id)).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_previous_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    let result = async_client().await.previous_track(Some(device_id)).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_recommendations() {
    let mut payload = Map::new();
    let seed_artists = vec!["4NHQUGzhtTLFvgF5SZesLK".to_owned()];
    let seed_tracks = vec!["0c6xIDDpzE81m2q797ordA".to_owned()];
    payload.insert("min_energy".to_owned(), 0.4.into());
    payload.insert("min_popularity".to_owned(), 50.into());
    let result = async_client()
        .await
        .recommendations(
            Some(seed_artists),
            None,
            Some(seed_tracks),
            10,
            Some(Country::UnitedStates),
            &payload,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_repeat() {
    let result = async_client()
        .await
        .repeat(RepeatState::Context, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_album() {
    let query = "album:arrival artist:abba";
    let result = async_client()
        .await
        .search(query, SearchType::Album, 10, 0, None, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_artist() {
    let query = "tania bowra";
    let result = async_client()
        .await
        .search(
            query,
            SearchType::Artist,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_playlist() {
    let query = "\"doom metal\"";
    let result = async_client()
        .await
        .search(
            query,
            SearchType::Playlist,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_search_track() {
    let query = "abba";
    let result = async_client()
        .await
        .search(
            query,
            SearchType::Track,
            10,
            0,
            Some(Country::UnitedStates),
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_seek_track() {
    let result = async_client().await.seek_track(25000, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_shuffle() {
    let result = async_client().await.shuffle(true, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_start_playback() {
    let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
    let uris = vec!["spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_owned()];
    let result = async_client()
        .await
        .start_playback(Some(device_id), None, Some(uris), for_position(0), None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_transfer_playback() {
    let device_id = "74ASZWbe4lXaubB36ztrGX";
    let result = async_client()
        .await
        .transfer_playback(device_id, true)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_follow_artist() {
    let mut artists = vec![];
    let artist_id1 = "74ASZWbe4lXaubB36ztrGX";
    let artist_id2 = "08td7MxkoHQkXnWAYD8d6Q";
    artists.push(artist_id2);
    artists.push(artist_id1);
    let result = async_client().await.user_follow_artists(artists).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_unfollow_artist() {
    let mut artists = vec![];
    let artist_id1 = "74ASZWbe4lXaubB36ztrGX";
    let artist_id2 = "08td7MxkoHQkXnWAYD8d6Q";
    artists.push(artist_id2);
    artists.push(artist_id1);
    let result = async_client().await.user_unfollow_artists(artists).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_follow_users() {
    let mut users = vec![];
    let user_id1 = "exampleuser01";
    users.push(user_id1);
    let result = async_client().await.user_follow_users(users).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_unfollow_users() {
    let mut users = vec![];
    let user_id1 = "exampleuser01";
    users.push(user_id1);
    let result = async_client().await.user_unfollow_users(users).await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_add_tracks() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    tracks_ids.push(track_id1);
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    let result = async_client()
        .await
        .user_playlist_add_tracks(user_id, playlist_id, tracks_ids, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_change_detail() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let playlist_name = "A New Playlist-update";
    let result = async_client()
        .await
        .user_playlist_change_detail(
            user_id,
            playlist_id,
            Some(playlist_name),
            Some(false),
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_check_follow() {
    let owner_id = "jmperezperez";
    let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
    let mut user_ids: Vec<String> = vec![];
    let user_id1 = String::from("possan");
    user_ids.push(user_id1);
    let user_id2 = String::from("elogain");
    user_ids.push(user_id2);
    let result = async_client()
        .await
        .user_playlist_check_follow(owner_id, playlist_id, &user_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_create() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_name = "A New Playlist";
    let playlists = async_client()
        .await
        .user_playlist_create(user_id, playlist_name, false, None)
        .await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_follow_playlist() {
    let owner_id = "jmperezperez";
    let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
    let result = async_client()
        .await
        .user_playlist_follow_playlist(owner_id, playlist_id, true)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_recorder_tracks() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let range_start = 0;
    let insert_before = 1;
    let range_length = 1;
    let result = async_client()
        .await
        .user_playlist_recorder_tracks(
            user_id,
            playlist_id,
            range_start,
            range_length,
            insert_before,
            None,
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_remove_all_occurrences_of_tracks() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    let result = async_client()
        .await
        .user_playlist_remove_all_occurrences_of_tracks(user_id, playlist_id, tracks_ids, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_remove_specific_occurrences_of_tracks() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
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
    let result = async_client()
        .await
        .user_playlist_remove_specific_occurrences_of_tracks(user_id, &playlist_id, tracks, None)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_replace_tracks() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
    let mut tracks_ids = vec![];
    let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
    let track_id2 = "spotify:track:1301WleyT98MSxVHPZCA6M";
    tracks_ids.push(track_id2);
    tracks_ids.push(track_id1);
    async_client()
        .await
        .user_playlist_replace_tracks(user_id, playlist_id, tracks_ids)
        .await
        .expect("replace tracks in a playlist failed");
    assert!(true);
}

#[tokio::test]
#[ignore]
async fn test_user_playlist() {
    let user_id = "spotify";
    let mut playlist_id = String::from("59ZbFPES4DQwEjBpWHzrtC");
    let playlists = async_client()
        .await
        .user_playlist(user_id, Some(&mut playlist_id), None, None)
        .await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlists() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlists = async_client()
        .await
        .user_playlists(user_id, Some(10), None)
        .await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_tracks() {
    let user_id = "spotify";
    let playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
    let playlists = async_client()
        .await
        .user_playlist_tracks(user_id, &playlist_id, None, Some(2), None, None)
        .await;
    assert!(playlists.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_unfollow() {
    let user_id = "2257tjys2e2u2ygfke42niy2q";
    let playlist_id = "65V6djkcVRyOStLd8nza8E";
    let result = async_client()
        .await
        .user_playlist_unfollow(user_id, playlist_id)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_volume() {
    let result = async_client().await.volume(78, None).await;
    assert!(result.is_ok());
}

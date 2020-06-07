extern crate chrono;
extern crate rspotify;
extern crate serde_json;

use chrono::prelude::*;
use serde_json::map::Map;

use rspotify::client::Spotify;
use rspotify::model::offset::for_position;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::senum::{Country, RepeatState, SearchType, TimeRange};
use rspotify::util::get_token;

// Because of all these tests need to poll up the browser, it is impossible to
// run in a CI envirnment like travis, so I just ignore them all. You could
// delete the #[ignore] tag, and run it on your local machine.

#[tokio::test]
#[ignore]
async fn test_categories() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let categories = spotify
                .categories(None, Some(Country::UnitedStates), 10, 0)
                .await;
            assert!(categories.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_current_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let context = spotify.current_playback(None, None).await;
            assert!(context.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_current_playing() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-currently-playing")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let context = spotify.current_playing(None, None).await;
            assert!(context.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_followed_artists() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let artists = spotify.current_user_followed_artists(10, None).await;
            assert!(artists.is_ok())
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_playing_track() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-currently-playing user-read-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let playing = spotify.current_user_playing_track().await;
            assert!(playing.is_ok())
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_playlists() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-read-private")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let playlists = spotify.current_user_playlists(10, None).await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_current_user_recently_played() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-recently-played")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let history = spotify.current_user_recently_played(10).await;
            assert!(history.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums_add() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut album_ids = vec![];
            let album_id1 = String::from("6akEvsycLGftJxYudPjmqK");
            let album_id2 = String::from("628oezqK2qfmCjC6eXNors");
            album_ids.push(album_id2);
            album_ids.push(album_id1);
            let result = spotify.current_user_saved_albums_add(&album_ids).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums_delete() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut album_ids = vec![];
            let album_id1 = String::from("6akEvsycLGftJxYudPjmqK");
            let album_id2 = String::from("628oezqK2qfmCjC6eXNors");
            album_ids.push(album_id2);
            album_ids.push(album_id1);
            let result = spotify.current_user_saved_albums_delete(&album_ids).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_albums() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let albums = spotify.current_user_saved_albums(10, 0).await;
            assert!(albums.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_add() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify.current_user_saved_tracks_add(&tracks_ids).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_contains() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify
                .current_user_saved_tracks_contains(&tracks_ids)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks_delete() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify.current_user_saved_tracks_delete(&tracks_ids).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_current_user_saved_tracks() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let tracks = spotify.current_user_saved_tracks(10, 0).await;
            assert!(tracks.is_ok());
        }
        None => assert!(false),
    }
}
#[tokio::test]
#[ignore]
async fn test_current_user_top_artists() {
    let mut oauth = SpotifyOAuth::default().scope("user-top-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let artist = spotify
                .current_user_top_artists(10, 0, TimeRange::ShortTerm)
                .await;
            assert!(artist.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_current_user_top_tracks() {
    let mut oauth = SpotifyOAuth::default().scope("user-top-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let tracks = spotify
                .current_user_top_tracks(10, 0, TimeRange::ShortTerm)
                .await;
            assert!(tracks.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_device() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let devices = spotify.device().await;
            assert!(devices.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_featured_playlists() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let now: DateTime<Utc> = Utc::now();
            let playlists = spotify
                .featured_playlists(None, None, Some(now), 10, 0)
                .await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_me() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-birthdate user-read-private user-read-email")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user = spotify.me().await;
            assert!(user.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_new_releases() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let albums = spotify.new_releases(Some(Country::Sweden), 10, 0).await;
            assert!(albums.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_next_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.next_track(Some(device_id)).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_pause_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.pause_playback(Some(device_id)).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_previous_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.previous_track(Some(device_id)).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_recommendations() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut payload = Map::new();
            let seed_artists = vec!["4NHQUGzhtTLFvgF5SZesLK".to_owned()];
            let seed_tracks = vec!["0c6xIDDpzE81m2q797ordA".to_owned()];
            payload.insert("min_energy".to_owned(), 0.4.into());
            payload.insert("min_popularity".to_owned(), 50.into());
            let result = spotify
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
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_repeat() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let result = spotify.repeat(RepeatState::Context, None).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_search_album() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let query = "album:arrival artist:abba";
            let result = spotify
                .search(query, SearchType::Album, 10, 0, None, None)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_search_artist() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let query = "tania bowra";
            let result = spotify
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
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_search_playlist() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let query = "\"doom metal\"";
            let result = spotify
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
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_search_track() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let query = "abba";
            let result = spotify
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
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_seek_track() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let result = spotify.seek_track(25000, None).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_shuffle() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let result = spotify.shuffle(true, None).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_start_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let uris = vec!["spotify:track:4iV5W9uYEdYUVa79Axb7Rh".to_owned()];
            let result = spotify
                .start_playback(Some(device_id), None, Some(uris), for_position(0), None)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_transfer_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = "74ASZWbe4lXaubB36ztrGX";
            let result = spotify.transfer_playback(device_id, true).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_follow_artist() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut artists = vec![];
            let artist_id1 = String::from("74ASZWbe4lXaubB36ztrGX");
            let artist_id2 = String::from("08td7MxkoHQkXnWAYD8d6Q");
            artists.push(artist_id2);
            artists.push(artist_id1);
            let result = spotify.user_follow_artists(&artists).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_unfollow_artist() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut artists = vec![];
            let artist_id1 = String::from("74ASZWbe4lXaubB36ztrGX");
            let artist_id2 = String::from("08td7MxkoHQkXnWAYD8d6Q");
            artists.push(artist_id2);
            artists.push(artist_id1);
            let result = spotify.user_unfollow_artists(&artists).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_follow_users() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut users = vec![];
            let user_id1 = String::from("exampleuser01");
            users.push(user_id1);
            let result = spotify.user_follow_users(&users).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_unfollow_users() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-modify").build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let mut users = vec![];
            let user_id1 = String::from("exampleuser01");
            users.push(user_id1);
            let result = spotify.user_unfollow_users(&users).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_add_tracks() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            tracks_ids.push(track_id1);
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            let result = spotify
                .user_playlist_add_tracks(user_id, playlist_id, &tracks_ids, None)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_change_detail() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let playlist_name = "A New Playlist-update";
            let result = spotify
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
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_user_playlist_check_follow() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let owner_id = "jmperezperez";
            let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
            let mut user_ids: Vec<String> = vec![];
            let user_id1 = String::from("possan");
            user_ids.push(user_id1);
            let user_id2 = String::from("elogain");
            user_ids.push(user_id2);
            let result = spotify
                .user_playlist_check_follow(owner_id, playlist_id, &user_ids)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_user_playlist_create() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_name = "A New Playlist";
            let playlists = spotify
                .user_playlist_create(user_id, playlist_name, false, None)
                .await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_follow_playlist() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let owner_id = "jmperezperez";
            let playlist_id = "2v3iNvBX8Ay1Gt2uXtUKUT";
            let result = spotify
                .user_playlist_follow_playlist(owner_id, playlist_id, true)
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_recorder_tracks() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let range_start = 0;
            let insert_before = 1;
            let range_length = 1;
            let result = spotify
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
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_remove_all_occurrences_of_tracks() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify
                .user_playlist_remove_all_occurrences_of_tracks(
                    user_id,
                    playlist_id,
                    &tracks_ids,
                    None,
                )
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_remove_specific_occurrenes_of_tracks() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
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
            let result = spotify
                .user_playlist_remove_specific_occurrenes_of_tracks(
                    user_id,
                    &playlist_id,
                    tracks,
                    None,
                )
                .await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_replace_tracks() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "5jAOgWXCBKuinsGiZxjDQ5";
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            spotify
                .user_playlist_replace_tracks(user_id, playlist_id, &tracks_ids)
                .await
                .expect("replace tracks in a playlist failed");
            assert!(true);
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_user_playlist() {
    let mut spotify_oauth = SpotifyOAuth::default().build();
    match get_token(&mut spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "spotify";
            let mut playlist_id = String::from("59ZbFPES4DQwEjBpWHzrtC");
            let playlists = spotify
                .user_playlist(user_id, Some(&mut playlist_id), None, None)
                .await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlists() {
    let mut spotify_oauth = SpotifyOAuth::default()
        .scope("playlist-read-private, playlist-read-collaborative")
        .build();
    match get_token(&mut spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlists = spotify.user_playlists(user_id, Some(10), None).await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_user_playlist_tracks() {
    let mut spotify_oauth = SpotifyOAuth::default().build();
    match get_token(&mut spotify_oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "spotify";
            let playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
            let playlists = spotify
                .user_playlist_tracks(user_id, &playlist_id, None, Some(2), None, None)
                .await;
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}
#[tokio::test]
#[ignore]
async fn test_user_playlist_unfollow() {
    let mut oauth = SpotifyOAuth::default()
        .scope("playlist-modify-private playlist-modify-public")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user_id = "2257tjys2e2u2ygfke42niy2q";
            let playlist_id = "65V6djkcVRyOStLd8nza8E";
            let result = spotify.user_playlist_unfollow(user_id, playlist_id).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[tokio::test]
#[ignore]
async fn test_volume() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth).await {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let result = spotify.volume(78, None).await;
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

extern crate rspotify;
extern crate chrono;
extern crate serde_json;

use chrono::prelude::*;
use serde_json::map::Map;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::spotify::senum::{AlbumType, Country, TimeRange, RepeatState};
use rspotify::spotify::util::get_token;

#[test]
fn test_album() {
    // Set client_id and client_secret in .env file or
    // export CLIENT_ID="your client_id"
    // export CLIENT_SECRET="secret"
    let client_credential = SpotifyClientCredentials::default().build();

    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:album:0sNOF9WDwhWunNAHPD3Baj";
    let albums = spotify.album(birdy_uri);
    assert!(albums.is_ok());
}
#[test]
fn test_albums() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri1 = String::from("spotify:album:41MnTivkwTO3UUJ8DrqEJJ");
    let birdy_uri2 = String::from("spotify:album:6JWc4iAiJ9FjyK0B59ABb4");
    let birdy_uri3 = String::from("spotify:album:6UXCm6bOO4gFlDQZV5yL37");
    let track_uris = vec![birdy_uri1, birdy_uri2, birdy_uri3];
    let albums = spotify.albums(track_uris);
    assert!(albums.is_ok())
}
#[test]
fn test_album_tracks() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:album:6akEvsycLGftJxYudPjmqK";
    let tracks = spotify.album_track(birdy_uri, Some(2), None);
    assert!(tracks.is_ok());
}

#[test]
fn test_artist_related_artists() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:43ZHCT0cAZBISjO8DG9PnE";
    let artist = spotify.artist_related_artists(birdy_uri);
    assert!(artist.is_ok())
}

#[test]
fn test_artist() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let artist = spotify.artist(birdy_uri);
    assert!(artist.is_ok());
}

#[test]
fn test_artists_albums() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let albums = spotify.artist_albums(birdy_uri,
                                       Some(AlbumType::Album),
                                       Some(Country::UnitedStates),
                                       Some(10),
                                       None);
    assert!(albums.is_ok());
}
#[test]
fn test_artists() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri1 = String::from("spotify:artist:0oSGxfWSnnOXhD2fKuz2Gy");
    let birdy_uri2 = String::from("spotify:artist:3dBVyJ7JuOMt4GE9607Qin");
    let artist_uris = vec![birdy_uri1, birdy_uri2];
    let artists = spotify.artists(artist_uris);
    assert!(artists.is_ok());
}

#[test]
fn test_artist_top_tracks() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let birdy_uri = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
    let tracks = spotify.artist_top_tracks(birdy_uri, Country::UnitedStates);
    assert!(tracks.is_ok());
}
#[test]
fn test_audio_analysis() {
    let client_credential = SpotifyClientCredentials::default().build();

    // Or set client_id and client_secret explictly
    // let client_credential = SpotifyClientCredentials::default()
    //     .client_id("this-is-my-client-id")
    //     .client_secret("this-is-my-client-secret")
    //     .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let track = "3JIxjvbbDrA9ztYlNcp3yL";
    let analysis = spotify.audio_analysis(track);
    assert!(analysis.is_ok());
}

#[test]
fn test_audio_features() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let track = "spotify:track:06AKEBrKUckW0KREUWRnvT";
    let features = spotify.audio_features(track);
    assert!(features.is_ok());
}

#[test]
fn test_audios_features() {
    let client_credential = SpotifyClientCredentials::default().build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    let mut tracks_ids = vec![];
    let track_id1 = String::from("spotify:track:4JpKVNYnVcJ8tuMKjAj50A");
    tracks_ids.push(track_id1);
    let track_id2 = String::from("spotify:track:24JygzOLM0EmRQeGtFcIcG");
    tracks_ids.push(track_id2);
    let features = spotify.audios_features(&tracks_ids);
    assert!(features.is_ok())
}

#[test]
fn test_categories() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let categories = spotify.categories(None, Some(Country::UnitedStates), 10, 0);
            assert!(categories.is_ok());
        }
        None => assert!(false),
    };
}
#[test]
fn test_current_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let context = spotify.current_playback(None);
            assert!(context.is_ok());
        }
        None => assert!(false),
    };

}
#[test]
fn test_current_playing() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-currently-playing")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let context = spotify.current_playing(None);
            assert!(context.is_ok());
        }
        None => assert!(false),
    };

}
#[test]
fn test_current_user_followed_artists() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let artists = spotify.current_user_followed_artists(10, None);
            assert!(artists.is_ok())
        }
        None => assert!(false),
    };

}

#[test]
fn test_current_user_playing_track() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-currently-playing user-read-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let playing = spotify.current_user_playing_track();
            assert!(playing.is_ok())
        }
        None => assert!(false),
    };
}

#[test]
fn test_current_user_playlists() {
    let mut spotify_oauth = SpotifyOAuth::default()
        .scope("playlist-read-private")
        .build();
    match get_token(&mut spotify_oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();

            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let playlists = spotify.current_user_playlists(10, None);
            assert!(playlists.is_ok());

        }
        None => assert!(false),
    };
}
#[test]
fn test_current_user_recently_played() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-recently-played")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let history = spotify.current_user_recently_played(10);
            assert!(history.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_current_user_saved_albums_add() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth) {
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
            let result = spotify.current_user_saved_albums_add(&album_ids);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };

}
#[test]
fn test_current_user_saved_albums() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let albums = spotify.current_user_saved_albums(10, 0);
            assert!(albums.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_current_user_saved_tracks_add() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth) {
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
            let result = spotify.current_user_saved_tracks_add(&tracks_ids);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_current_user_saved_tracks_contains() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            //this is my(samray's) user_id and playlist_id, so just change
            // user_id and playlist_id to yours, or you will get a 403 forbidden error
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify.current_user_saved_tracks_contains(&tracks_ids);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };

}

#[test]
fn test_current_user_saved_tracks_delete() {
    let mut oauth = SpotifyOAuth::default().scope("user-library-modify").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            // Or set client_id and client_secret explictly
            // let client_credential = SpotifyClientCredentials::default()
            //     .client_id("this-is-my-client-id")
            //     .client_secret("this-is-my-client-secret")
            //     .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            //this is my(samray's) user_id and playlist_id, so just change
            // user_id and playlist_id to yours, or you will get a 403 forbidden error
            let mut tracks_ids = vec![];
            let track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
            let track_id2 = String::from("spotify:track:1301WleyT98MSxVHPZCA6M");
            tracks_ids.push(track_id2);
            tracks_ids.push(track_id1);
            let result = spotify.current_user_saved_tracks_delete(tracks_ids);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };

}
#[test]
fn test_current_user_saved_tracks() {

    let mut oauth = SpotifyOAuth::default().scope("user-library-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let tracks = spotify.current_user_saved_tracks(10, 0);
            assert!(tracks.is_ok());
        }
        None => assert!(false),
    }
}
#[test]
fn test_current_user_top_artists() {
    let mut oauth = SpotifyOAuth::default().scope("user-top-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let artist = spotify.current_user_top_artists(10, 0, TimeRange::ShortTerm);
            assert!(artist.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_current_user_top_tracks() {
    let mut oauth = SpotifyOAuth::default().scope("user-top-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let tracks = spotify.current_user_top_tracks(10, 0, TimeRange::ShortTerm);
            assert!(tracks.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_device() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let devices = spotify.device();
            assert!(devices.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_featured_playlists() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let now: DateTime<Utc> = Utc::now();
            let playlists = spotify.featured_playlists(None, None, Some(now), 10, 0);
            assert!(playlists.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_me() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-birthdate user-read-private user-read-email")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let user = spotify.me();
            assert!(user.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_new_releases() {
    let mut oauth = SpotifyOAuth::default().scope("user-follow-read").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();

            let albums = spotify.new_releases(Some(Country::Sweden), 10, 0);
            assert!(albums.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_next_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.next_track(Some(device_id));
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_pause_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.pause_playback(Some(device_id));
            assert!(result.is_ok());

        }
        None => assert!(false),
    };
}

#[test]
fn test_previous_playback() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let device_id = String::from("74ASZWbe4lXaubB36ztrGX");
            let result = spotify.previous_track(Some(device_id));
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_recommendations() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth) {
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
            let result = spotify.recommendations(Some(seed_artists),
                                                 None,
                                                 Some(seed_tracks),
                                                 10,
                                                 Some(Country::UnitedStates),
                                                 &payload);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn test_repeat() {
    let mut oauth = SpotifyOAuth::default()
        .scope("user-modify-playback-state")
        .build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let result = spotify.repeat(RepeatState::Context, None);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

#[test]
fn search_album() {
    let mut oauth = SpotifyOAuth::default().scope("user-read-private").build();
    match get_token(&mut oauth) {
        Some(token_info) => {
            let client_credential = SpotifyClientCredentials::default()
                .token_info(token_info)
                .build();
            let spotify = Spotify::default()
                .client_credentials_manager(client_credential)
                .build();
            let query = "album:arrival artist:abba";
            let result = spotify.search_album(query, 10, 0, None);
            assert!(result.is_ok());
        }
        None => assert!(false),
    };
}

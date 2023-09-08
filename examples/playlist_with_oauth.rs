use rspotify::{prelude::*,
    clients::pagination::Paginator,
    model::{
        EpisodeId, FullPlaylist,
        ItemPositions, PlaylistId, 
         TrackId, UserId,
    },
     scopes,ClientResult, AuthCodeSpotify, Credentials, OAuth};
use base64::{engine::general_purpose, Engine as _};
use maybe_async::maybe_async;
use reqwest;

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

#[maybe_async]
async fn check_num_tracks(client: &AuthCodeSpotify, playlist_id: PlaylistId<'_>, num: i32) {
    let fetched_tracks = fetch_all(client.playlist_items(playlist_id, None, None)).await;
    assert_eq!(fetched_tracks.len() as i32, num);
}

async fn check_playlist_cover(client: &AuthCodeSpotify, playlist_id: PlaylistId<'_>) {
    let img_url = "https://images.dog.ceo/breeds/poodle-toy/n02113624_8936.jpg";
    let img_bytes = tokio::task::spawn_blocking(move || {reqwest::blocking::get(img_url).unwrap().bytes().unwrap()}).await.unwrap();
    let playlist_cover_base64 = general_purpose::URL_SAFE_NO_PAD.encode(img_bytes.clone());

    println!("playlist id : {}", playlist_id);

    // check cover image
    let cover_res = client
        .playlist_cover_image(playlist_id.as_ref())
        .await
        .unwrap()
        .unwrap();

    println!("cover_res pre upload: {:?}", cover_res);

    // add playlist cover image
    client
        .playlist_upload_cover_image(playlist_id.as_ref(), &playlist_cover_base64)
        .await
        .unwrap();

    // check cover image
    let cover_res = client
        .playlist_cover_image(playlist_id.as_ref())
        .await
        .unwrap()
        .unwrap();

    println!("cover_res post upload: {:?}", cover_res);
}


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
    let user_playlists = fetch_all(client.user_playlists(user.id)).await;
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

async fn test_playlist(client: AuthCodeSpotify) {
    
    let playlist = check_playlist_create(&client).await;
    check_playlist_tracks(&client, &playlist).await;
    check_playlist_follow(&client, &playlist).await;
    check_playlist_cover(&client, playlist.id).await;
}

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.
    env_logger::init();

    // The credentials must be available in the environment. Enable the
    // `env-file` feature in order to read them from an `.env` file.
    let creds = Credentials::from_env().unwrap();

    // Using every possible scope
    let scopes = scopes!(
        "user-read-email",
        "user-read-private",
        "user-top-read",
        "user-library-read",
        "playlist-read-collaborative",
        "playlist-read-private",
        "ugc-image-upload",
        "playlist-modify-public",
        "playlist-modify-private"
    );

    let oauth = OAuth::from_env(scopes).unwrap();

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();
    // This function requires the `cli` feature enabled.
    spotify.prompt_for_token(&url).await.unwrap();
    test_playlist(spotify).await; 
}

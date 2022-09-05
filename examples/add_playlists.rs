// cargo run --example add_playlists --features="env-file cli client-reqwest"

use itertools::Itertools;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use rspotify_model::{playlist, PlaylistId, TrackId, UserId};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MusicBackuperPlaylist {
    pub r#type: String,
    pub title: String,
    pub items: Vec<MusicBackuperPlaylistItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct MusicBackuperPlaylistItem {
    pub album_id: Option<String>,
    pub artist: Option<String>,
    pub album_name: Option<String>,

    pub track_id: Option<String>,
    #[serde(rename = "ISRC")]
    pub isrc: Option<String>,
    pub title: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let creds = Credentials::from_env().unwrap();
    let scopes = scopes!("playlist-modify-public", "playlist-modify-private");
    let oauth = OAuth::from_env(scopes).unwrap();
    let spotify = AuthCodeSpotify::new(creds, oauth);
    let url = spotify.get_authorize_url(false).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let user_id = UserId::from_id(env::var("RSPOTIFY_USER_ID").ok()?)?;
    let json = r#"2022-09-03_Spotify_Music_Backuper.json"#;

    let backuper: Vec<MusicBackuperPlaylist> =
        serde_json::from_reader(std::fs::read(json)?.deref())?;
    let playlists = backuper.into_iter().filter(|item| item.r#type == "Playlist");
    for playlist in playlists {
            let playlist2 = spotify.user_playlist_create(user_id.clone(), &playlist.title, Some(false), Some(false), None).await?;

            let tracks: Vec<PlayableId> = playlist
                .items
                .iter()
                .flat_map(|item| {
                    item.track_id
                        .as_ref()
                        .map(|id| PlayableId::Track(TrackId::from_id(id).unwrap()))
                })
                .collect();

            println!("playlist {} has {} tracks", &playlist.title, tracks.len());

            // let tracks: Vec<Vec<PlayableId>> = tracks
            //     .into_iter()
            //     .chunks(100) // itertools = "0.10.2"
            //     .into_iter()
            //     .map(|chunk| chunk.collect())
            //     .collect();

            // 1 track at a time and ignore the errors
            for track in tracks {
                spotify.playlist_add_items(playlist2.id.clone(),[track],None).await;
            }
    }

    Ok(())
}

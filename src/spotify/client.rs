// 3rd-part library
use serde_json;
use serde_json::Value;
use serde_json::map::Map;
use serde::de::Deserialize;
use reqwest::header::{Authorization, Bearer, ContentType, Headers};
use reqwest::Client;
use reqwest::Method::{self, Delete, Get, Post, Put};
use chrono::prelude::*;

//  built-in battery
use std::collections::HashMap;
use std::io::Read;
use std::string::String;
use std::borrow::Cow;

use errors::{Result, ResultExt};
use super::oauth2::SpotifyClientCredentials;
use super::spotify_enum::{AlbumType, Type, TimeRange, Country};
use super::model::album::{FullAlbum, FullAlbums, SimplifiedAlbum, PageSimpliedAlbums};
use super::model::page::{Page, CursorBasedPage};
use super::model::track::{FullTrack, FullTracks, SimplifiedTrack, SavedTrack};
use super::model::artist::{FullArtist, FullArtists, CursorPageFullArtists};
use super::model::user::{PublicUser, PrivateUser};
use super::model::playlist::{FullPlaylist, PlaylistTrack, SimplifiedPlaylist, FeaturedPlaylists};
use super::model::cud_result::CUDResult;
use super::model::playing::{Playing, PlayHistory};
use super::model::category::PageCategory;
// use super::model::recommend::Recommendations;
use super::model::audio::{AudioFeatures, AudioFeaturesPayload};
use super::model::device::DevicePayload;
use super::model::context::{FullPlayingContext, SimplifiedPlayingContext};
use super::util::convert_map_to_string;
pub struct Spotify {
    pub prefix: String,
    pub access_token: Option<String>,
    pub client_credentials_manager: Option<SpotifyClientCredentials>,
}
impl Spotify {
    pub fn default() -> Spotify {
        Spotify {
            prefix: "https://api.spotify.com/v1/".to_owned(),
            access_token: None,
            client_credentials_manager: None,
        }
    }

    // pub fn prefix(mut self, prefix: &str) -> Spotify {
    pub fn prefix(mut self, prefix: &str) -> Spotify {
        self.prefix = prefix.to_owned();
        self
    }

    pub fn access_token(mut self, access_token: &str) -> Spotify {
        self.access_token = Some(access_token.to_owned());
        self
    }

    pub fn client_credentials_manager(mut self,
                                      client_credential_manager: SpotifyClientCredentials)
                                      -> Spotify {
        self.client_credentials_manager = Some(client_credential_manager);
        self
    }

    pub fn build(self) -> Spotify {
        if self.access_token.is_none() && self.client_credentials_manager.is_none() {
            panic!("access_token and client_credentials_manager are none!!!");
        }
        self
    }

    fn auth_headers(&self) -> Authorization<Bearer> {
        match self.access_token {
            Some(ref token) => Authorization(Bearer { token: token.to_owned() }),
            None => {
                match self.client_credentials_manager {
                    Some(ref client_credentials_manager) => {
                        let token = client_credentials_manager.get_access_token();
                        Authorization(Bearer { token: token })
                    }
                    None => panic!("client credentials manager is none"),
                }
            }
        }
    }

    fn internal_call(&self, method: Method, url: &str, payload: Value) -> Result<String> {
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.spotify.com/v1/", &url].concat().into();
        }
        let client = Client::new();

        let mut headers = Headers::new();
        headers.set(self.auth_headers());
        headers.set(ContentType::json());
        let mut response = client
            .request(method, &url.into_owned())
            .headers(headers)
            .json(&payload)
            .send()
            .expect("send request failed");

        let mut buf = String::new();
        response
            .read_to_string(&mut buf)
            .expect("failed to read response");
        if response.status().is_success() {
            Ok(buf)
        } else {
            eprintln!("parameters: {:?}\n", &payload);
            eprintln!("response: {:?}", &response);
            eprintln!("content: {:?}", &buf);
            bail!("send request failed, http code:{}, error message:{}",
                  response.status(),
                  &buf);
        }
    }
    ///send get request
    fn get(&self, url: &str, params: &mut HashMap<&str, String>) -> Result<String> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = String::from(url.to_owned());
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.internal_call(Get, &url_with_params, json!({}))
        } else {
            self.internal_call(Get, url, json!({}))
        }
    }

    ///send post request
    fn post(&self, url: &str, payload: Value) -> Result<String> {
        self.internal_call(Post, url, payload)
    }
    ///send put request
    fn put(&self, url: &str, payload: Value) -> Result<String> {
        self.internal_call(Put, url, payload)
    }

    /// send delete request
    fn delete(&self, url: &str, payload: Value) -> Result<String> {
        self.internal_call(Delete, url, payload)
    }

    ///https://developer.spotify.com/web-api/get-track/
    ///returns a single track given the track's ID, URI or URL
    ///Parameters:
    ///- track_id - a spotify URI, URL or ID
    pub fn track(&self, track_id: &str) -> Result<FullTrack> {
        let trid = self.get_id(Type::Track, track_id);
        let url = format!("tracks/{}", trid);
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullTrack>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-tracks/
    ///returns a list of tracks given a list of track IDs, URIs, or URLs
    ///Parameters:
    ///- track_ids - a list of spotify URIs, URLs or IDs
    ///- market - an ISO 3166-1 alpha-2 country code.
    pub fn tracks(&self, track_ids: Vec<&str>, market: Option<&str>) -> Result<FullTracks> {
        let mut ids: Vec<String> = vec![];
        for track_id in track_ids {
            ids.push(self.get_id(Type::Track, track_id));
        }
        let url = format!("tracks/?ids={}", ids.join(","));
        // url.push_str(&ids.join(","));
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_market) = market {
            params.insert("market", _market.to_owned());
        }
        println!("{:?}", &url);
        let result = self.get(&url, &mut params);
        self.convert_result::<FullTracks>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-artist/
    ///returns a single artist given the artist's ID, URI or URL
    ///Parameters:
    ///- artist_id - an artist ID, URI or URL
    pub fn artist(&self, artist_id: &str) -> Result<FullArtist> {
        let trid = self.get_id(Type::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        let result = self.get(&mut url, &mut HashMap::new());
        self.convert_result::<FullArtist>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-artists/
    ///returns a list of artists given the artist IDs, URIs, or URLs
    ///Parameters:
    ///- artist_ids - a list of  artist IDs, URIs or URLs
    pub fn artists(&self, artist_ids: Vec<String>) -> Result<FullArtists> {
        let mut ids: Vec<String> = vec![];
        for artist_id in artist_ids {
            ids.push(self.get_id(Type::Artist, &artist_id));
        }
        let url = format!("artists/?ids={}", ids.join(","));
        // url.push_str(&ids.join(","));
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullArtists>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-artists-albums/
    ///  Get Spotify catalog information about an artist's albums
    /// - artist_id - the artist ID, URI or URL
    /// - album_type - 'album', 'single', 'appears_on', 'compilation'
    /// - country - limit the response to one particular country.
    /// - limit  - the number of albums to return
    /// - offset - the index of the first album to return
    pub fn artist_albums(&self,
                         artist_id: &str,
                         album_type: Option<AlbumType>,
                         country: Option<Country>,
                         limit: Option<u32>,
                         offset: Option<u32>)
                         -> Result<Page<SimplifiedAlbum>> {
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_limit) = limit {
            params.insert("limit", _limit.to_string());
        }
        if let Some(_album_type) = album_type {
            params.insert("album_type", _album_type.as_str().to_owned());
        }
        if let Some(_offset) = offset {
            params.insert("offset", _offset.to_string());
        }
        if let Some(_country) = country {
            params.insert("country", _country.as_str().to_owned());
        }
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/albums", trid);
        // url.push_str(&trid);
        // url.push_str("/albums");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<SimplifiedAlbum>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-artists-top-tracks/
    /// Get Spotify catalog information about an artist's top 10 tracks by country.
    ///    Parameters:
    ///        - artist_id - the artist ID, URI or URL
    ///        - country - limit the response to one particular country.
    pub fn artist_top_tracks(&self,
                             artist_id: &str,
                             country: impl Into<Option<Country>>)
                             -> Result<FullTracks> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("country",
                      country
                          .into()
                          .unwrap_or(Country::UnitedStates)
                          .as_str()
                          .to_owned());
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/top-tracks", trid);
        // url.push_str(&trid);
        // url.push_str("/top-tracks");

        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullTracks>(&result.unwrap_or_default())
        // match self.get(&mut url, &mut params) {
        //     Ok(result) => {
        //         // let mut albums: Albums = ;
        //         match serde_json::from_str::<FullTracks>(&result) {
        //             Ok(_tracks) => Some(_tracks),
        //             Err(why) => {
        //                 eprintln!("convert albums from String to Albums failed {:?}", why);
        //                 None
        //             }
        //         }
        //     }
        //     Err(_) => None,
        // }
    }

    ///https://developer.spotify.com/web-api/get-related-artists/
    ///Get Spotify catalog information about artists similar to an
    ///identified artist. Similarity is based on analysis of the
    ///Spotify community's listening history.
    ///Parameters:
    ///- artist_id - the artist ID, URI or URL
    pub fn artist_related_artists(&self, artist_id: &str) -> Result<FullArtists> {
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/related-artists", trid);
        // url.push_str(&trid);
        // url.push_str("/related-artists");
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullArtists>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-album/
    ///returns a single album given the album's ID, URIs or URL
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    pub fn album(&self, album_id: &str) -> Result<FullAlbum> {
        let trid = self.get_id(Type::Album, album_id);
        let url = format!("albums/{}", trid);
        // url.push_str(&trid);
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullAlbum>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-albums/
    ///returns a list of albums given the album IDs, URIs, or URLs
    ///Parameters:
    ///- albums_ids - a list of  album IDs, URIs or URLs
    pub fn albums(&self, album_ids: Vec<String>) -> Result<FullAlbums> {
        let mut ids: Vec<String> = vec![];
        for album_id in album_ids {
            ids.push(self.get_id(Type::Album, &album_id));
        }
        let url = format!("albums/?ids={}", ids.join(","));
        // url.push_str(&ids.join(","));
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<FullAlbums>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-albums-tracks/
    ///Get Spotify catalog information about an album's tracks
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn album_track(&self,
                       album_id: &str,
                       limit: impl Into<Option<u32>>,
                       offset: impl Into<Option<u32>>)
                       -> Result<Page<SimplifiedTrack>> {
        let mut params = HashMap::new();
        let trid = self.get_id(Type::Album, album_id);
        let url = format!("albums/{}/tracks", trid);
        // url.push_str(&trid);
        // url.push_str("/tracks");
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<SimplifiedTrack>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-users-profile/
    ///Gets basic profile information about a Spotify User
    ///Parameters:
    ///- user - the id of the usr
    pub fn user(&self, user_id: &str) -> Result<PublicUser> {
        let mut url = format!("users/{}", user_id);
        let result = self.get(&url, &mut HashMap::new());
        self.convert_result::<PublicUser>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-a-list-of-current-users-playlists/
    ///Get current user playlists without required getting his profile
    ///Parameters:
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn current_user_playlists(&self,
                                  limit: impl Into<Option<u32>>,
                                  offset: impl Into<Option<u32>>)
                                  -> Result<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());

        let url = String::from("me/playlists");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<SimplifiedPlaylist>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-list-users-playlists/
    ///Gets playlists of a user
    ///Parameters:
    ///- user_id - the id of the usr
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn user_playlists(&self,
                          user_id: &str,
                          limit: impl Into<Option<u32>>,
                          offset: impl Into<Option<u32>>)
                          -> Result<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());
        let url = format!("users/{}/playlists", user_id);
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<SimplifiedPlaylist>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-list-users-playlists/
    ///Gets playlist of a user
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- fields - which fields to return
    pub fn user_playlist(&self,
                         user_id: &str,
                         playlist_id: Option<&mut str>,
                         fields: Option<&str>)
                         -> Result<FullPlaylist> {
        let mut params = HashMap::new();
        if let Some(_fields) = fields {
            params.insert("fields", _fields.to_string());
        }
        match playlist_id {
            Some(_playlist_id) => {
                let plid = self.get_id(Type::Playlist, _playlist_id);
                let url = format!("users/{}/playlists/{}", user_id, plid);
                let result = self.get(&url, &mut params);
                self.convert_result::<FullPlaylist>(&result.unwrap_or_default())
            }
            None => {
                let url = format!("users/{}/starred", user_id);
                let result = self.get(&url, &mut params);
                self.convert_result::<FullPlaylist>(&result.unwrap_or_default())
            }
        }
    }

    ///https://developer.spotify.com/web-api/get-playlists-tracks/
    ///Get full details of the tracks of a playlist owned by a user
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- fields - which fields to return
    ///- limit - the maximum number of tracks to return
    ///- offset - the index of the first track to return
    ///- market - an ISO 3166-1 alpha-2 country code.
    pub fn user_playlist_tracks(&self,
                                user_id: &str,
                                playlist_id: &str,
                                fields: Option<&str>,
                                limit: impl Into<Option<u32>>,
                                offset: impl Into<Option<u32>>,
                                market: Option<&str>)
                                -> Result<Page<PlaylistTrack>> {
        let mut params = HashMap::new();
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());
        if let Some(_market) = market {
            params.insert("market", _market.to_owned());
        }
        if let Some(_fields) = fields {
            params.insert("fields", _fields.to_string());
        }
        let plid = self.get_id(Type::Playlist, playlist_id);
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<PlaylistTrack>>(&result.unwrap_or_default())
    }


    ///https://developer.spotify.com/web-api/create-playlist/
    ///Creates a playlist for a user
    ///Parameters:
    ///- user_id - the id of the user
    ///- name - the name of the playlist
    ///- public - is the created playlist public
    ///- description - the description of the playlist
    pub fn user_playlist_create(&self,
                                user_id: &str,
                                name: &str,
                                public: impl Into<Option<bool>>,
                                description: impl Into<Option<String>>)
                                -> Result<FullPlaylist> {
        let public = public.into().unwrap_or(true);
        let description = description.into().unwrap_or("".to_owned());
        let params = json!({
            "name": name,
            "public": public,
            "description": description
        });
        let url = format!("users/{}/playlists", user_id);
        let result = self.post(&url, params);
        self.convert_result::<FullPlaylist>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/change-playlist-details/
    ///Changes a playlist's name and/or public/private state
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- name - optional name of the playlist
    ///- public - optional is the playlist public
    ///- collaborative - optional is the playlist collaborative
    ///- description - optional description of the playlist
    pub fn user_playlist_change_detail(&self,
                                       user_id: &str,
                                       playlist_id: &str,
                                       name: Option<&str>,
                                       public: Option<bool>,
                                       description: Option<String>,
                                       collaborative: Option<bool>)
                                       -> Result<String> {
        let mut params = Map::new();
        if let Some(_name) = name {
            params.insert("name".to_owned(), _name.into());
        }
        if let Some(_public) = public {
            params.insert("public".to_owned(), _public.into());
        }
        if let Some(_collaborative) = collaborative {
            params.insert("collaborative".to_owned(), _collaborative.into());
        }
        if let Some(_description) = description {
            params.insert("description".to_owned(), _description.into());
        }
        let url = format!("users/{}/playlists/{}", user_id,playlist_id);
        self.put(&url, Value::Object(params))
    }

    ///https://developer.spotify.com/web-api/unfollow-playlist/
    ///Unfollows (deletes) a playlist for a user
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    pub fn user_playlist_unfollow(&self, user_id: &str, playlist_id: &str) -> Result<String> {
        let url = format!("users/{}/playlists/{}/followers",user_id,playlist_id);
        self.delete(&url, json!({}))
    }

    ///https://developer.spotify.com/web-api/add-tracks-to-playlist/
    ///Adds tracks to a playlist
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- track_ids - a list of track URIs, URLs or IDs
    ///- position - the position to add the tracks
    pub fn user_playlist_add_tracks(&self,
                                    user_id: &str,
                                    playlist_id: &str,
                                    track_ids: Vec<String>,
                                    position: Option<i32>)
                                    -> Result<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        let mut params = Map::new();
        if let Some(_position) = position {
            params.insert("position".to_owned(), _position.into());
        }
        params.insert("uris".to_owned(), uris.into());
        let url = format!("users/{}/playlists/{}/tracks",user_id,plid);
        let result = self.post(&url, Value::Object(params));
        self.convert_result::<CUDResult>(&result.unwrap_or_default())

    }
    ///https://developer.spotify.com/web-api/replace-playlists-tracks/
    ///Replace all tracks in a playlist
    ///Parameters:
    ///- user - the id of the user
    ///- playlist_id - the id of the playlist
    ///- tracks - the list of track ids to add to the playlist

    pub fn user_playlist_replace_tracks(&self,
                                        user_id: &str,
                                        playlist_id: &str,
                                        track_ids: Vec<String>)
                                        -> Result<()> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        // let mut params = Map::new();
        // params.insert("uris".to_owned(), uris.into());
        let params = json!({
            "uris": uris
        });
        let url = format!("users/{}/playlists/{}/tracks",user_id,plid);
        match self.put(&url, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/reorder-playlists-tracks/
    ///Reorder tracks in a playlist
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- range_start - the position of the first track to be reordered
    ///- range_length - optional the number of tracks to be reordered (default: 1)
    ///- insert_before - the position where the tracks should be inserted
    ///- snapshot_id - optional playlist's snapshot ID
    pub fn user_playlist_recorder_tracks(&self,
                                         user_id: &str,
                                         playlist_id: &str,
                                         range_start: i32,
                                         range_length: impl Into<Option<i32>>,
                                         insert_before: i32,
                                         snapshot_id: Option<String>)
                                         -> Result<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let range_length = range_length.into().unwrap_or(1);
        let mut params = Map::new();
        if let Some(_snapshot_id) = snapshot_id {
            params.insert("snapshot_id".to_owned(), _snapshot_id.into());
        }
        params.insert("range_start".to_owned(), range_start.into());
        params.insert("range_length".to_owned(), range_length.into());
        params.insert("insert_before".to_owned(), insert_before.into());
        let url = format!("users/{}/playlists/{}/tracks",user_id,plid);
        let result = self.put(&url, Value::Object(params));
        self.convert_result::<CUDResult>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/remove-tracks-playlist/
    ///Removes all occurrences of the given tracks from the given playlist
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- track_ids - the list of track ids to add to the playlist
    ///- snapshot_id - optional id of the playlist snapshot
    pub fn user_playlist_remove_all_occurrences_of_tracks(&self,
                                                          user_id: &str,
                                                          playlist_id: &str,
                                                          track_ids: Vec<String>,
                                                          snapshot_id: Option<String>)
                                                          -> Result<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        let mut params = Map::new();
        let mut tracks: Vec<Map<String, Value>> = vec![];
        for uri in uris {
            let mut map = Map::new();
            map.insert("uri".to_owned(), uri.into());
            tracks.push(map);
        }
        params.insert("tracks".to_owned(), tracks.into());
        if let Some(_snapshot_id) = snapshot_id {
            params.insert("snapshot_id".to_owned(), _snapshot_id.into());
        }
        let url = format!("users/{}/playlists/{}/tracks",user_id,plid);
        let result = self.delete(&url, Value::Object(params));
        self.convert_result::<CUDResult>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/remove-tracks-playlist/
    ///Removes all occurrences of the given tracks from the given playlist
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    ///- tracks - an array of map containing Spotify URIs of the tracks
    /// to remove with their current positions in the playlist.  For example:
    ///{ "tracks": [{ "uri": "spotify:track:4iV5W9uYEdYUVa79Axb7Rh", "positions": [0,3] },{
    ///"uri": "spotify:track:1301WleyT98MSxVHPZCA6M", "positions": [7] }] }
    ///- snapshot_id - optional id of the playlist snapshot
    pub fn user_playlist_remove_specific_occurrenes_of_tracks(&self,
                                                              user_id: &str,
                                                              playlist_id: &str,
                                                              tracks: Vec<Map<String, Value>>,
                                                              snapshot_id: Option<String>)
                                                              -> Result<CUDResult> {
        let mut params = Map::new();
        let plid = self.get_id(Type::Playlist, playlist_id);
        let mut ftracks: Vec<Map<String, Value>> = vec![];
        for track in tracks {
            let mut map = Map::new();
            if let Some(_uri) = track.get("uri") {
                let uri = self.get_uri(Type::Track, &mut _uri.as_str().unwrap().to_owned());
                map.insert("uri".to_owned(), uri.into());
            }
            if let Some(_position) = track.get("position") {
                map.insert("position".to_owned(), _position.to_owned());
            }
            ftracks.push(map);
        }
        params.insert("tracks".to_owned(), ftracks.into());
        if let Some(_snapshot_id) = snapshot_id {
            params.insert("snapshot_id".to_owned(), _snapshot_id.into());
        }
        let url = format!("users/{}/playlists/{}/tracks",user_id,plid);
        let result = self.delete(&url, Value::Object(params));
        self.convert_result::<CUDResult>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/follow-playlist/
    ///Add the current authenticated user as a follower of a playlist.
    ///Parameters:
    ///- playlist_owner_id - the user id of the playlist owner
    ///- playlist_id - the id of the playlist
    pub fn user_playlist_follow_playlist(&self,
                                         playlist_owner_id: &str,
                                         playlist_id: &str,
                                         public: impl Into<Option<bool>>)
                                         -> Result<()> {
        let mut map = Map::new();
        let public = public.into().unwrap_or(true);
        map.insert("public".to_owned(), public.into());
        let url = format!("users/{}/playlists/{}/followers",playlist_owner_id,playlist_id);
        match self.put(&url, Value::Object(map)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }

    }

    ///https://developer.spotify.com/web-api/check-user-following-playlist/
    ///Check to see if the given users are following the given playlist
    ///Parameters:
    ///- playlist_owner_id - the user id of the playlist owner
    ///- playlist_id - the id of the playlist
    ///- user_ids - the ids of the users that you want to
    ///check to see if they follow the playlist. Maximum: 5 ids.
    pub fn user_playlist_check_follow(&self,
                                      playlist_owner_id: &str,
                                      playlist_id: &str,
                                      user_ids: Vec<String>)
                                      -> Result<Vec<bool>> {
        if user_ids.len() > 5 {
            eprintln!("The maximum length of user ids is limited to 5 :-)");
        }
        let url =
            format!("users/{}/playlists/{}/followers/contains?ids={}"
                                          ,playlist_owner_id,playlist_id,user_ids.join(","));
        let mut dumb: HashMap<&str, String> = HashMap::new();
        let result = self.get(&url, &mut dumb);
        self.convert_result::<Vec<bool>>(&result.unwrap_or_default())

    }
    ///https://developer.spotify.com/web-api/get-current-users-profile/
    ///Get detailed profile information about the current user.
    ///An alias for the 'current_user' method.
    pub fn me(&self) -> Result<PrivateUser> {
        let mut dumb: HashMap<&str, String> = HashMap::new();
        let url = String::from("me/");
        let result = self.get(&url, &mut dumb);
        self.convert_result::<PrivateUser>(&result.unwrap_or_default())
    }
    ///Get detailed profile information about the current user.
    ///An alias for the 'me' method.
    pub fn current_user(&self) -> Result<PrivateUser> {
        self.me()
    }

    /// https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
    /// Get information about the current users currently playing track.
    pub fn current_user_playing_track(&self) -> Result<Option<Playing>> {
        let mut dumb = HashMap::new();
        let url = String::from("me/player/currently-playing");
        match self.get(&url, &mut dumb) {
            Ok(result) => {
                if result.is_empty() {
                    Ok(None)
                } else {
                    self.convert_result::<Option<Playing>>(&result)
                }
            }
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/get-users-saved-albums/
    ///Gets a list of the albums saved in the current authorized user's
    ///"Your Music" library
    ///Parameters:
    ///- limit - the number of albums to return
    ///- offset - the index of the first album to return
    ///- market - Provide this parameter if you want to apply Track Relinking.
    pub fn current_user_saved_albums(&self,
                                     limit: impl Into<Option<u32>>,
                                     offset: impl Into<Option<u32>>)
                                     -> Result<Page<FullAlbum>> {
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        let url = String::from("me/albums");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<FullAlbum>>(&result.unwrap_or_default())
    }
    ///https://developer.spotify.com/web-api/get-users-saved-tracks/
    ///Parameters:
    ///- limit - the number of tracks to return
    ///- offset - the index of the first track to return
    ///- market - Provide this parameter if you want to apply Track Relinking.
    pub fn current_user_saved_tracks(&self,
                                     limit: impl Into<Option<u32>>,
                                     offset: impl Into<Option<u32>>)
                                     -> Result<Page<SavedTrack>> {
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        let url = String::from("me/tracks");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<SavedTrack>>(&result.unwrap_or_default())

    }
    ///https://developer.spotify.com/web-api/get-followed-artists/
    ///Gets a list of the artists followed by the current authorized user
    ///Parameters:
    ///- limit - the number of tracks to return
    ///- after - ghe last artist ID retrieved from the previous request
    pub fn current_user_followed_artists(&self,
                                         limit: impl Into<Option<u32>>,
                                         after: Option<String>)
                                         -> Result<CursorPageFullArtists> {
        let limit = limit.into().unwrap_or(20);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        if let Some(_after) = after {
            params.insert("after", _after);
        }
        params.insert("type", Type::Artist.as_str().to_owned());
        let url = String::from("me/following");
        let result = self.get(&url, &mut params);
        self.convert_result::<CursorPageFullArtists>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/remove-tracks-user/
    ///Remove one or more tracks from the current user's
    ///"Your Music" library.
    ///Parameters:
    ///- track_ids - a list of track URIs, URLs or IDs
    pub fn current_user_saved_tracks_delete(&self, mut track_ids: Vec<String>) -> Result<()> {
        let uris: Vec<String> = track_ids
            .iter_mut()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/?ids={}",uris.join(","));
        match self.delete(&url, json!({})) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/check-users-saved-tracks/
    ///Check if one or more tracks is already saved in
    ///the current Spotify user’s “Your Music” library.
    ///Parameters:
    ///- track_ids - a list of track URIs, URLs or IDs
    pub fn current_user_saved_tracks_contains(&self, track_ids: Vec<String>) -> Result<Vec<bool>> {
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/contains/?ids={}",uris.join(","));
        let mut dumb = HashMap::new();
        let result = self.get(&url, &mut dumb);
        self.convert_result::<Vec<bool>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/save-tracks-user/
    ///Save one or more tracks to the current user's
    ///"Your Music" library.
    ///Parameters:
    ///- track_ids - a list of track URIs, URLs or IDs
    pub fn current_user_saved_tracks_add(&self, track_ids: Vec<String>) -> Result<()> {
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/?ids={}",uris.join(","));
        match self.put(&url, json!({})) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/get-users-top-artists-and-tracks/
    ///Get the current user's top artists
    ///Parameters:
    ///- limit - the number of entities to return
    ///- offset - the index of the first entity to return
    ///- time_range - Over what time frame are the affinities computed

    pub fn current_user_top_artists(&self,
                                    limit: impl Into<Option<u32>>,
                                    offset: impl Into<Option<u32>>,
                                    time_range: impl Into<Option<TimeRange>>)
                                    -> Result<Page<FullArtist>> {
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        let time_range = time_range.into().unwrap_or(TimeRange::MediumTerm);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        params.insert("time_range", time_range.as_str().to_owned());
        let url = String::from("me/top/artists");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<FullArtist>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-users-top-artists-and-tracks/
    ///Get the current user's top tracks
    ///Parameters:
    ///- limit - the number of entities to return
    ///- offset - the index of the first entity to return
    ///- time_range - Over what time frame are the affinities computed
    pub fn current_user_top_tracks(&self,
                                   limit: impl Into<Option<u32>>,
                                   offset: impl Into<Option<u32>>,
                                   time_range: impl Into<Option<TimeRange>>)
                                   -> Result<Page<FullTrack>> {
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        let time_range = time_range.into().unwrap_or(TimeRange::MediumTerm);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        params.insert("time_range", time_range.as_str().to_owned());
        let url = String::from("me/top/tracks");
        let result = self.get(&url, &mut params);
        self.convert_result::<Page<FullTrack>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/web-api-personalization-endpoints/get-recently-played/
    ///Get the current user's recently played tracks
    ///Parameters:
    ///- limit - the number of entities to return
    pub fn current_user_recently_played(self,
                                        limit: impl Into<Option<u32>>)
                                        -> Result<CursorBasedPage<PlayHistory>> {
        let limit = limit.into().unwrap_or(50);
        let mut params = HashMap::new();
        params.insert("limit", limit.to_string());
        let url = String::from("me/player/recently-played");
        let result = self.get(&url, &mut params);
        self.convert_result::<CursorBasedPage<PlayHistory>>(&result.unwrap_or_default())
    }


    ///https://developer.spotify.com/web-api/save-albums-user/
    ///Add one or more albums to the current user's
    ///"Your Music" library.
    ///Parameters:
    ///- album_ids - a list of album URIs, URLs or IDs
    pub fn current_user_saved_albums_add(&self, album_ids: Vec<String>) -> Result<()> {
        let uris: Vec<String> = album_ids
            .iter()
            .map(|id| self.get_id(Type::Album, id))
            .collect();
        let url = format!("me/albums/?ids={}",uris.join(","));
        match self.put(&url, json!({})) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }


    ///https://developer.spotify.com/web-api/follow-artists-users/
    ///Follow one or more artists
    ///Parameters:
    ///- artist_ids - a list of artist IDs
    pub fn user_follow_artists(&self, artist_ids: Vec<String>) -> Result<()> {
        let url = format!("me/following?type=artist&ids={}",artist_ids.join(","));
        match self.put(&url, json!({})) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/follow-artists-users/
    ///Follow one or more users
    ///Parameters:
    ///- user_ids - a list of artist IDs
    pub fn user_follow_users(&self, user_ids: Vec<String>) -> Result<()> {
        let url = format!("me/following?type=user&ids={}",user_ids.join(","));
        match self.put(&url, json!({})) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/get-list-featured-playlists/
    ///Get a list of Spotify featured playlists
    ///Parameters:
    ///- locale - The desired language, consisting of a lowercase ISO
    ///639 language code and an uppercase ISO 3166-1 alpha-2 country
    ///code, joined by an underscore.
    ///- country - An ISO 3166-1 alpha-2 country code.
    ///- timestamp - A timestamp in ISO 8601 format:
    ///yyyy-MM-ddTHH:mm:ss. Use this parameter to specify the user's
    ///local time to get results tailored for that specific date and
    ///time in the day
    ///- limit - The maximum number of items to return. Default: 20.
    ///Minimum: 1. Maximum: 50
    ///- offset - The index of the first item to return. Default: 0
    ///(the first object). Use with limit to get the next set of
    ///items.
    pub fn featured_playlists(&self,
                              locale: Option<String>,
                              country: Option<Country>,
                              timestamp: Option<DateTime<Utc>>,
                              limit: impl Into<Option<u32>>,
                              offset: impl Into<Option<u32>>)
                              -> Result<FeaturedPlaylists> {
        let mut params = HashMap::new();
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        if let Some(_locale) = locale {
            params.insert("locale", _locale);
        }
        if let Some(_country) = country {
            params.insert("country", _country.as_str().to_owned());
        }
        if let Some(_timestamp) = timestamp {
            params.insert("timestamp", _timestamp.to_rfc3339());
        }
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        let url = String::from("browse/featured-playlists");
        let result = self.get(&url, &mut params);
        self.convert_result::<FeaturedPlaylists>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-list-new-releases/
    ///Get a list of new album releases featured in Spotify
    ///Parameters:
    ///- country - An ISO 3166-1 alpha-2 country code.
    ///- limit - The maximum number of items to return. Default: 20.
    ///Minimum: 1. Maximum: 50
    ///- offset - The index of the first item to return. Default: 0
    ///(the first object). Use with limit to get the next set of
    ///items.
    pub fn new_releases(&self,
                        country: Option<Country>,
                        limit: impl Into<Option<u32>>,
                        offset: impl Into<Option<u32>>)
                        -> Result<PageSimpliedAlbums> {
        let mut params = HashMap::new();
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        if let Some(_country) = country {
            params.insert("country", _country.as_str().to_owned());
        }
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        let url = String::from("browse/new-releases");
        let result = self.get(&url, &mut params);
        self.convert_result::<PageSimpliedAlbums>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-list-categories/
    ///Get a list of new album releases featured in Spotify
    ///Parameters:
    ///- country - An ISO 3166-1 alpha-2 country code.
    ///- locale - The desired language, consisting of an ISO 639
    ///language code and an ISO 3166-1 alpha-2 country code, joined
    ///by an underscore.
    ///- limit - The maximum number of items to return. Default: 20.
    ///Minimum: 1. Maximum: 50
    ///- offset - The index of the first item to return. Default: 0
    ///(the first object). Use with limit to get the next set of
    ///items.
    pub fn categories(&self,
                      locale: Option<String>,
                      country: Option<Country>,
                      limit: impl Into<Option<u32>>,
                      offset: impl Into<Option<u32>>)
                      -> Result<PageCategory> {
        let mut params = HashMap::new();
        let limit = limit.into().unwrap_or(20);
        let offset = offset.into().unwrap_or(0);
        if let Some(_locale) = locale {
            params.insert("locale", _locale);
        }
        if let Some(_country) = country {
            params.insert("country", _country.as_str().to_owned());
        }
        params.insert("limit", limit.to_string());
        params.insert("offset", offset.to_string());
        let url = String::from("browse/categories");
        let result = self.get(&url, &mut params);
        self.convert_result::<PageCategory>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-recommendations/
    ///Get Recommendations Based on Seeds
    ///            Parameters:
    /// - seed_artists - a list of artist IDs, URIs or URLs
    /// - seed_tracks - a list of artist IDs, URIs or URLs
    /// - seed_genres - a list of genre names. Available genres for
    /// - country - An ISO 3166-1 alpha-2 country code. If provided, all
    ///   results will be playable in this country.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 100
    /// - min/max/target_<attribute> - For the tuneable track attributes listed
    ///   in the documentation, these values provide filters and targeting on
    ///   results.
    // pub fn recommendations(&self,
    //                        mut seed_artists: Option<Vec<String>>,
    //                        mut seed_genres: Option<Vec<String>>,
    //                        mut seed_tracks: Option<Vec<String>>,
    //                        limit: impl Into<Option<u32>>,
    //                        country: Option<Country>)
    //                        -> Result<Recommendations> {
    // }
    ///https://developer.spotify.com/web-api/get-audio-features/
    ///Get audio features for a track
    ///- track - track URI, URL or ID
    pub fn audio_features(&self, track: &str) -> Result<AudioFeatures> {
        let track_id = self.get_id(Type::Track, track);
        let url = format!("audio-features/{}",track_id);
        let mut dumb = HashMap::new();
        let result = self.get(&url, &mut dumb);
        self.convert_result::<AudioFeatures>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-audio-features/
    ///Get Audio Features for Several Tracks
    /// -tracks a list of track URIs, URLs or IDs
    pub fn audios_features(&self, tracks: Vec<String>) -> Result<Option<AudioFeaturesPayload>> {
        let ids: Vec<String> = tracks
            .iter()
            .map(|track| self.get_id(Type::Track, track))
            .collect();
        let url = format!("audio-features/?ids={}",ids.join(","));
        let mut dumb = HashMap::new();
        match self.get(&url, &mut dumb) {
            Ok(result) => {
                if result.is_empty() {
                    Ok(None)
                } else {
                    self.convert_result::<Option<AudioFeaturesPayload>>(&result)
                }
            }
            Err(e) => Err(e),
        }

    }

    ///https://developer.spotify.com/web-api/get-a-users-available-devices/
    ///Get a User’s Available Devices
    pub fn device(&self) -> Result<DevicePayload> {
        let url = String::from("me/player/devices");
        let mut dumb = HashMap::new();
        let result = self.get(&url, &mut dumb);
        self.convert_result::<DevicePayload>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-information-about-the-users-current-playback/
    ///Get Information About The User’s Current Playback
    ///        Parameters:
    ///        - market - an ISO 3166-1 alpha-2 country code.
    pub fn current_playback(&self, market: Option<Country>) -> Result<Option<FullPlayingContext>> {
        let url = String::from("me/player");
        let mut params = HashMap::new();
        if let Some(_market) = market {
            params.insert("country", _market.as_str().to_owned());
        }
        match self.get(&url, &mut params) {
            Ok(result) => {
                if result.is_empty() {
                    Ok(None)
                } else {
                    self.convert_result::<Option<FullPlayingContext>>(&result)
                }
            }
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/get-the-users-currently-playing-track/
    /// Get the User’s Currently Playing Track
    ///        Parameters:
    ///        - market - an ISO 3166-1 alpha-2 country code.
    pub fn current_playing(&self,
                           market: Option<Country>)
                           -> Result<Option<SimplifiedPlayingContext>> {
        let url = String::from("me/player/currently-playing");
        let mut params = HashMap::new();
        if let Some(_market) = market {
            params.insert("country", _market.as_str().to_owned());
        }
        match self.get(&url, &mut params) {
            Ok(result) => {
                if result.is_empty() {
                    Ok(None)
                } else {
                    self.convert_result::<Option<SimplifiedPlayingContext>>(&result)
                }
            }
            Err(e) => Err(e),
        }
    }
    ///https://developer.spotify.com/web-api/transfer-a-users-playback/
    ///Transfer a User’s Playback
    ///Note: Although an array is accepted, only a single device_id is currently
    /// supported. Supplying more than one will return 400 Bad Request
    ///            Parameters:
    ///- device_id - transfer playback to this device
    ///- force_play - true: after transfer, play. false:
    ///keep current state.
    pub fn transfer_playback(&self,
                             device_id: &str,
                             force_play: impl Into<Option<bool>>)
                             -> Result<()> {
        let device_ids = vec![device_id.to_owned()];
        let force_play = force_play.into().unwrap_or(true);
        let mut payload = Map::new();
        payload.insert("devie_ids".to_owned(), device_ids.into());
        payload.insert("play".to_owned(), force_play.into());
        let url = String::from("me/player");
        match self.put(&url, Value::Object(payload)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    ///https://developer.spotify.com/web-api/start-a-users-playback/
    ///Start/Resume a User’s Playback
    ///Provide a `context_uri` to start playback or a album,
    ///artist, or playlist.
    ///
    ///Provide a `uris` list to start playback of one or more
    ///tracks.
    ///
    ///Provide `offset` as {"position": <int>} or {"uri": "<track uri>"}
    ///to start playback at a particular offset.
    ///
    ///Parameters:
    ///- device_id - device target for playback
    ///- context_uri - spotify context uri to play
    ///- uris - spotify track uris
    ///- offset - offset into context by index or track

    pub fn start_playback(&self,
                          device_id: Option<String>,
                          context_uri: Option<String>,
                          uris: Option<Vec<String>>,
                          offset: Option<u32>)
                          -> Result<()> {
        if context_uri.is_some() && uris.is_some() {
            eprintln!("specify either contexxt uri or uris, not both");
        }
        let mut params = Map::new();
        if let Some(_context_uri) = context_uri {
            params.insert("context_uri".to_owned(), _context_uri.into());
        }
        if let Some(_uris) = uris {
            params.insert("uris".to_owned(), _uris.into());
        }
        if let Some(_offset) = offset {
            params.insert("offset".to_owned(), _offset.into());
        }
        let url = self.append_device_id("me/player/play".to_owned(), device_id);
        match self.put(&url, Value::Object(params)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }

    }

    pub fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Result<T> {
        let result = serde_json::from_str::<T>(input)
            .chain_err(|| format!("convert result failed, content {:?}",input))?;
        Ok(result)
    }

    ///Append device ID to API path.
    fn append_device_id(&self, path: String, device_id: Option<String>) -> String {
        let mut new_path = path.clone();
        if let Some(_device_id) = device_id {
            if path.contains("?") {
                new_path.push_str(&format!("&device_id={}",_device_id));
            } else {
                new_path.push_str(&format!("?device_id={}",_device_id));
            }
        }
        new_path
    }

    fn get_uri(&self, _type: Type, _id: &str) -> String {
        let mut uri = String::from("spotify:");
        uri.push_str(_type.as_str());
        uri.push(':');
        uri.push_str(&self.get_id(_type, _id));
        uri
    }
    /// get spotify id by type and id
    fn get_id(&self, _type: Type, id: &str) -> String {
        let mut _id = id.to_owned().clone();
        let fields: Vec<&str> = _id.split(":").collect();
        let len = fields.len();
        if len >= 3 {
            if _type.as_str() != fields[len - 2] {
                eprintln!("expected id of type {:?} but found type {:?} {:?}",
                                        _type,
                                        fields[len - 2],
                                        _id);
            } else {
                return fields[len - 1].to_owned();
            }
        }
        let sfields: Vec<&str> = _id.split("/").collect();
        let len: usize = sfields.len();
        if len >= 3 {
            if _type.as_str() != sfields[len - 2] {
                eprintln!(
                                        "expected id of type {:?} but found type {:?} {:?}",
                                        _type,
                                        sfields[len - 2],
                                        _id
                                );
            } else {
                return sfields[len - 1].to_owned();
            }
        }
        return _id.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_id() {
        // assert artist
        let spotify = Spotify::default().access_token("test-access").build();
        let mut artist_id = String::from("spotify:artist:2WX2uTcsvV5OnS0inACecP");
        let id = spotify.get_id(Type::Artist, &mut artist_id);
        assert_eq!("2WX2uTcsvV5OnS0inACecP", &id);
        // assert album
        let mut artist_id_a = String::from("spotify/album/2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
                        "2WX2uTcsvV5OnS0inACecP",
                        &spotify.get_id(Type::Album, &mut artist_id_a)
                );

        // mismatch type
        let mut artist_id_b = String::from("spotify:album:2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
                        "spotify:album:2WX2uTcsvV5OnS0inACecP",
                        &spotify.get_id(Type::Artist, &mut artist_id_b)
                );

        // could not split
        let mut artist_id_c = String::from("spotify-album-2WX2uTcsvV5OnS0inACecP");
        assert_eq!(
                        "spotify-album-2WX2uTcsvV5OnS0inACecP",
                        &spotify.get_id(Type::Artist, &mut artist_id_c)
                );

        let mut playlist_id = String::from("spotify:playlist:59ZbFPES4DQwEjBpWHzrtC");
        assert_eq!(
                        "59ZbFPES4DQwEjBpWHzrtC",
                        &spotify.get_id(Type::Playlist, &mut playlist_id)
                );
    }
    #[test]
    fn test_get_uri() {
        let spotify = Spotify::default().access_token("test-access").build();
        let mut track_id1 = String::from("spotify:track:4iV5W9uYEdYUVa79Axb7Rh");
        let mut track_id2 = String::from("1301WleyT98MSxVHPZCA6M");
        let uri1 = spotify.get_uri(Type::Track, &mut track_id1);
        let uri2 = spotify.get_uri(Type::Track, &mut track_id2);
        assert_eq!(track_id1,uri1);
        assert_eq!("spotify:track:1301WleyT98MSxVHPZCA6M",&uri2);
    }
}

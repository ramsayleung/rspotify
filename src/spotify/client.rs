// 3rd-part library
use serde_json;
use serde_json::Value;
use serde_json::map::Map;
use serde::de::Deserialize;
use reqwest::header::{Authorization, Bearer, ContentType, Headers};
use reqwest::Client;
use reqwest::Method::{self, Delete, Get, Post, Put};

//  built-in battery
use std::collections::HashMap;
use std::io::Read;
use std::borrow::Cow;

use errors::Result;
use super::oauth2::SpotifyClientCredentials;
use super::spotify_enum::{AlbumType, Type};
use super::model::album::{FullAlbum, FullAlbums, SimplifiedAlbum};
use super::model::page::Page;
use super::model::track::{FullTrack, FullTracks, SimplifiedTrack};
use super::model::artist::{FullArtist, FullArtists};
use super::model::user::PublicUser;
use super::model::playlist::{FullPlaylist, PlaylistTrack, SimplifiedPlaylist};
use super::model::cud_result::CUDResult;
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
    fn get(&self, url: &mut str, params: &mut HashMap<&str, String>) -> Result<String> {
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
    fn post(&self, url: &mut str, payload: Value) -> Result<String> {
        self.internal_call(Post, url, payload)
    }
    ///send put request
    fn put(&self, url: &mut str, payload: Value) -> Result<String> {
        self.internal_call(Put, url, payload)
    }

    /// send delete request
    fn delete(&self, url: &mut str, payload: Value) -> Result<String> {
        self.internal_call(Delete, url, payload)
    }

    ///https://developer.spotify.com/web-api/get-track/
    ///returns a single track given the track's ID, URI or URL
    ///Parameters:
    ///- track_id - a spotify URI, URL or ID
    pub fn track(&self, track_id: &mut str) -> Option<FullTrack> {
        let trid = self.get_id(Type::Track, track_id);
        let mut url = String::from("tracks/");
        url.push_str(&trid);
        let result = self.get(&mut url, &mut HashMap::new());
        self.convert_result::<FullTrack>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-tracks/
    ///returns a list of tracks given a list of track IDs, URIs, or URLs
    ///Parameters:
    ///- track_ids - a list of spotify URIs, URLs or IDs
    ///- market - an ISO 3166-1 alpha-2 country code.
    pub fn tracks(&self, track_ids: Vec<String>, market: Option<&str>) -> Option<FullTracks> {
        let mut ids: Vec<String> = vec![];
        for mut track_id in track_ids {
            ids.push(self.get_id(Type::Track, &mut track_id));
        }
        let mut url = String::from("tracks/?ids=");
        url.push_str(&ids.join(","));
        let mut params: HashMap<&str, String> = HashMap::new();
        if let Some(_market) = market {
            params.insert("market", _market.to_owned());
        }
        println!("{:?}", &url);
        let result = self.get(&mut url, &mut params);
        self.convert_result::<FullTracks>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-artist/
    ///returns a single artist given the artist's ID, URI or URL
    ///Parameters:
    ///- artist_id - an artist ID, URI or URL
    pub fn artist(&self, artist_id: &mut str) -> Option<FullArtist> {
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
    pub fn artists(&self, artist_ids: Vec<String>) -> Option<FullArtists> {
        let mut ids: Vec<String> = vec![];
        for mut artist_id in artist_ids {
            ids.push(self.get_id(Type::Artist, &mut artist_id));
        }
        let mut url = String::from("artists/?ids=");
        url.push_str(&ids.join(","));
        let result = self.get(&mut url, &mut HashMap::new());
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
                         artist_id: &mut str,
                         album_type: Option<AlbumType>,
                         country: Option<&str>,
                         limit: Option<u32>,
                         offset: Option<u32>)
                         -> Option<Page<SimplifiedAlbum>> {
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
            params.insert("country", _country.to_string());
        }
        let trid = self.get_id(Type::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/albums");
        let result = self.get(&mut url, &mut params);
        self.convert_result::<Page<SimplifiedAlbum>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-artists-top-tracks/
    /// Get Spotify catalog information about an artist's top 10 tracks by country.
    ///    Parameters:
    ///        - artist_id - the artist ID, URI or URL
    ///        - country - limit the response to one particular country.
    pub fn artist_top_tracks(&self,
                             artist_id: &mut str,
                             country: impl Into<Option<String>>)
                             -> Option<FullTracks> {
        let mut params: HashMap<&str, String> = HashMap::new();
        params.insert("country", country.into().unwrap_or("US".to_owned()));
        let trid = self.get_id(Type::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/top-tracks");
        match self.get(&mut url, &mut params) {
            Ok(result) => {
                // let mut albums: Albums = ;
                match serde_json::from_str::<FullTracks>(&result) {
                    Ok(_tracks) => Some(_tracks),
                    Err(why) => {
                        eprintln!("convert albums from String to Albums failed {:?}", why);
                        None
                    }
                }
            }
            Err(_) => None,
        }
    }

    ///https://developer.spotify.com/web-api/get-related-artists/
    ///Get Spotify catalog information about artists similar to an
    ///identified artist. Similarity is based on analysis of the
    ///Spotify community's listening history.
    ///Parameters:
    ///- artist_id - the artist ID, URI or URL
    pub fn artist_related_artists(&self, artist_id: &mut str) -> Option<FullArtists> {
        let trid = self.get_id(Type::Artist, artist_id);
        let mut url = String::from("artists/");
        url.push_str(&trid);
        url.push_str("/related-artists");
        let result = self.get(&mut url, &mut HashMap::new());
        self.convert_result::<FullArtists>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-album/
    ///returns a single album given the album's ID, URIs or URL
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    pub fn album(&self, album_id: &mut str) -> Option<FullAlbum> {
        let trid = self.get_id(Type::Album, album_id);
        let mut url = String::from("albums/");
        url.push_str(&trid);
        let result = self.get(&mut url, &mut HashMap::new());
        self.convert_result::<FullAlbum>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-several-albums/
    ///returns a list of albums given the album IDs, URIs, or URLs
    ///Parameters:
    ///- albums_ids - a list of  album IDs, URIs or URLs
    pub fn albums(&self, album_ids: Vec<String>) -> Option<FullAlbums> {
        let mut ids: Vec<String> = vec![];
        for mut album_id in album_ids {
            ids.push(self.get_id(Type::Album, &mut album_id));
        }
        let mut url = String::from("albums/?ids=");
        url.push_str(&ids.join(","));
        let result = self.get(&mut url, &mut HashMap::new());
        self.convert_result::<FullAlbums>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-albums-tracks/
    ///Get Spotify catalog information about an album's tracks
    ///Parameters:
    ///- album_id - the album ID, URI or URL
    ///- limit  - the number of items to return
    ///- offset - the index of the first item to return
    pub fn album_track(&self,
                       album_id: &mut str,
                       limit: impl Into<Option<u32>>,
                       offset: impl Into<Option<u32>>)
                       -> Option<Page<SimplifiedTrack>> {
        let mut params = HashMap::new();
        let trid = self.get_id(Type::Album, album_id);
        let mut url = String::from("albums/");
        url.push_str(&trid);
        url.push_str("/tracks");
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());
        let result = self.get(&mut url, &mut params);
        self.convert_result::<Page<SimplifiedTrack>>(&result.unwrap_or_default())
    }

    ///https://developer.spotify.com/web-api/get-users-profile/
    ///Gets basic profile information about a Spotify User
    ///Parameters:
    ///- user - the id of the usr
    pub fn user(&self, user_id: &str) -> Option<PublicUser> {
        let mut url = String::from(format!("users/{}", user_id));
        let result = self.get(&mut url, &mut HashMap::new());
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
                                  -> Option<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());

        let mut url = String::from("me/playlists");
        let result = self.get(&mut url, &mut params);
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
                          -> Option<Page<SimplifiedPlaylist>> {
        let mut params = HashMap::new();
        params.insert("limit", limit.into().unwrap_or(50).to_string());
        params.insert("offset", offset.into().unwrap_or(0).to_string());
        let mut url = String::from(format!("users/{}/playlists", user_id));
        let result = self.get(&mut url, &mut params);
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
                         -> Option<FullPlaylist> {
        let mut params = HashMap::new();
        if let Some(_fields) = fields {
            params.insert("fields", _fields.to_string());
        }
        match playlist_id {
            Some(_playlist_id) => {
                let plid = self.get_id(Type::Playlist, _playlist_id);
                let mut url = String::from(format!("users/{}/playlists/{}", user_id, plid));
                let result = self.get(&mut url, &mut params);
                self.convert_result::<FullPlaylist>(&result.unwrap_or_default())
            }
            None => {
                let mut url = String::from(format!("users/{}/starred", user_id));
                let result = self.get(&mut url, &mut params);
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
                                playlist_id: &mut str,
                                fields: Option<&str>,
                                limit: impl Into<Option<u32>>,
                                offset: impl Into<Option<u32>>,
                                market: Option<&str>)
                                -> Option<Page<PlaylistTrack>> {
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
        let mut url = String::from(format!("users/{}/playlists/{}/tracks", user_id, plid));
        let result = self.get(&mut url, &mut params);
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
                                -> Option<FullPlaylist> {
        let public = public.into().unwrap_or(true);
        let description = description.into().unwrap_or("".to_owned());
        let params = json!({
            "name": name,
            "public": public,
            "description": description
        });
        let mut url = String::from(format!("users/{}/playlists", user_id));
        let result = self.post(&mut url, params);
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
        let mut url = String::from(format!("users/{}/playlists/{}", user_id,playlist_id));
        self.put(&mut url, Value::Object(params))
    }

    ///https://developer.spotify.com/web-api/unfollow-playlist/
    ///Unfollows (deletes) a playlist for a user
    ///Parameters:
    ///- user_id - the id of the user
    ///- playlist_id - the id of the playlist
    pub fn user_playlist_unfollow(&self, user_id: &str, playlist_id: &str) -> Result<String> {
        let mut url = String::from(format!("users/{}/playlists/{}/followers",user_id,playlist_id));
        self.delete(&mut url, json!({}))
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
                                    playlist_id: &mut str,
                                    mut track_ids: Vec<String>,
                                    position: Option<i32>)
                                    -> Option<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter_mut()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        let mut params = Map::new();
        if let Some(_position) = position {
            params.insert("position".to_owned(), _position.into());
        }
        params.insert("uris".to_owned(), uris.into());
        let mut url = String::from(format!("users/{}/playlists/{}/tracks",user_id,plid));
        let result = self.post(&mut url, Value::Object(params));
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
                                        playlist_id: &mut str,
                                        mut track_ids: Vec<String>)
                                        -> Result<()> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter_mut()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        // let mut params = Map::new();
        // params.insert("uris".to_owned(), uris.into());
        let params = json!({
            "uris": uris
        });
        let mut url = String::from(format!("users/{}/playlists/{}/tracks",user_id,plid));
        match self.put(&mut url, params) {
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
                                         playlist_id: &mut str,
                                         range_start: i32,
                                         range_length: impl Into<Option<i32>>,
                                         insert_before: i32,
                                         snapshot_id: Option<String>)
                                         -> Option<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let range_length = range_length.into().unwrap_or(1);
        let mut params = Map::new();
        if let Some(_snapshot_id) = snapshot_id {
            params.insert("snapshot_id".to_owned(), _snapshot_id.into());
        }
        params.insert("range_start".to_owned(), range_start.into());
        params.insert("range_length".to_owned(), range_length.into());
        params.insert("insert_before".to_owned(), insert_before.into());
        let mut url = String::from(format!("users/{}/playlists/{}/tracks",user_id,plid));
        let result = self.put(&mut url, Value::Object(params));
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
                                                          playlist_id: &mut str,
                                                          mut track_ids: Vec<String>,
                                                          snapshot_id: Option<String>)
                                                          -> Option<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter_mut()
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
        let mut url = String::from(format!("users/{}/playlists/{}/tracks",user_id,plid));
        let result = self.delete(&mut url, Value::Object(params));
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
                                                              playlist_id: &mut str,
                                                              tracks: Vec<Map<String, Value>>,
                                                              snapshot_id: Option<String>)
                                                              -> Option<CUDResult> {
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
        let mut url = String::from(format!("users/{}/playlists/{}/tracks",user_id,plid));
        let result = self.delete(&mut url, Value::Object(params));
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
        let mut url =
            String::from(format!("users/{}/playlists/{}/followers",playlist_owner_id,playlist_id));
        self.put(&mut url, Value::Object(map));
        Ok(())
    }



    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> Option<T> {
        match serde_json::from_str::<T>(input) {
            Ok(result) => Some(result),
            Err(why) => {
                eprintln!("convert result failed {:?}", why);
                eprintln!("content: {:?}", &input);
                None
            }
        }
    }

    fn get_uri(&self, _type: Type, _id: &mut str) -> String {
        let mut uri = String::from("spotify:");
        uri.push_str(_type.as_str());
        uri.push(':');
        uri.push_str(&self.get_id(_type, _id));
        uri
    }
    /// get spotify id by type and id
    fn get_id(&self, _type: Type, _id: &mut str) -> String {
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

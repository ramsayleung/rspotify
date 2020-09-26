//! Client to Spotify API endpoint

use chrono::prelude::*;
use derive_builder::Builder;
use log::error;
use maybe_async::maybe_async;
use serde::Deserialize;
use serde_json::map::Map;
use serde_json::{json, Value};
use thiserror::Error;

use std::path::PathBuf;

use super::http::BaseClient;
use super::json_insert;
use super::model::album::{FullAlbum, FullAlbums, PageSimpliedAlbums, SavedAlbum, SimplifiedAlbum};
use super::model::artist::{CursorPageFullArtists, FullArtist, FullArtists};
use super::model::audio::{AudioAnalysis, AudioFeatures, AudioFeaturesPayload};
use super::model::category::PageCategory;
use super::model::context::{CurrentlyPlaybackContext, CurrentlyPlayingContext};
use super::model::cud_result::CUDResult;
use super::model::device::DevicePayload;
use super::model::page::{CursorBasedPage, Page};
use super::model::playing::{PlayHistory, Playing};
use super::model::playlist::{FeaturedPlaylists, FullPlaylist, PlaylistTrack, SimplifiedPlaylist};
use super::model::recommend::Recommendations;
use super::model::search::SearchResult;
use super::model::show::{
    FullEpisode, FullShow, SeveralEpisodes, SeversalSimplifiedShows, Show, SimplifiedEpisode,
};
use super::model::track::{FullTrack, FullTracks, SavedTrack, SimplifiedTrack};
use super::model::user::{PrivateUser, PublicUser};
use super::oauth2::{Credentials, OAuth, Token};
use super::senum::{
    AdditionalType, AlbumType, Country, IncludeExternal, RepeatState, SearchType, TimeRange, Type,
};

/// Possible errors returned from the `rspotify` client.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Raised when the authentication isn't configured properly.
    #[error("invalid client authentication: {0}")]
    InvalidAuth(String),

    // TODO: this could be replaced with `StatusCode`
    #[error("request unauthorized")]
    Unauthorized,

    // TODO: this could be replaced with `StatusCode`
    #[error("exceeded request limit")]
    RateLimited(Option<usize>),

    // TODO: this could be replaced with `StatusCode`
    #[error("request error: {0}")]
    Request(String),

    #[error("status code {0}: {1}")]
    StatusCode(u16, String),

    #[error("spotify error: {0}")]
    Api(#[from] ApiError),

    #[error("json parse error: {0}")]
    ParseJSON(#[from] serde_json::Error),

    #[error("input/output error: {0}")]
    IO(#[from] std::io::Error),

    #[cfg(feature = "cli")]
    #[error("cli error: {0}")]
    CLI(String),

    #[error("cache file error: {0}")]
    CacheFile(String),
}

pub type ClientResult<T> = Result<T, ClientError>;

/// Matches errors that are returned from the Spotfiy
/// API as part of the JSON response object.
#[derive(Debug, Error, Deserialize)]
pub enum ApiError {
    /// See https://developer.spotify.com/documentation/web-api/reference/object-model/#error-object
    #[error("{status}: {message}")]
    #[serde(alias = "error")]
    Regular { status: u16, message: String },

    /// See https://developer.spotify.com/documentation/web-api/reference/object-model/#player-error-object
    #[error("{status} ({reason}): {message}")]
    #[serde(alias = "error")]
    Player {
        status: u16,
        message: String,
        reason: String,
    },
}

/// Spotify API object
#[derive(Builder, Debug, Clone)]
pub struct Spotify {
    /// reqwest needs an instance of its client to perform requests.
    #[cfg(feature = "client-reqwest")]
    #[builder(setter(skip))]
    pub(in crate) client: reqwest::Client,

    /// The access token information required for requests to the Spotify API.
    #[builder(setter(strip_option), default)]
    pub token: Option<Token>,

    /// The credentials needed for obtaining a new access token, for requests
    /// without OAuth authentication.
    #[builder(setter(strip_option), default)]
    pub credentials: Option<Credentials>,

    /// The OAuth information required for obtaining a new access token, for
    /// requests with OAuth authentication. `credentials` also needs to be
    /// set up.
    #[builder(setter(strip_option), default)]
    pub oauth: Option<OAuth>,

    /// The Spotify API prefix, `https://api.spotify.com/v1/` by default.
    #[builder(
        setter(into),
        default = r#"String::from("https://api.spotify.com/v1/")"#
    )]
    pub prefix: String,

    /// The cache file path, in case it's used. By default it's
    /// `.spotify_token_cache.json`.
    #[builder(default = r#"PathBuf::from(".spotify_token_cache.json")"#)]
    pub cache_path: PathBuf,
}

// Endpoint-related methods for the client.
impl Spotify {
    /// Returns the access token, or an error in case it's not configured.
    pub(in crate) fn get_token(&self) -> ClientResult<&Token> {
        self.token
            .as_ref()
            .ok_or_else(|| ClientError::InvalidAuth("no access token configured".to_string()))
    }

    /// Returns the credentials, or an error in case it's not configured.
    pub(in crate) fn get_creds(&self) -> ClientResult<&Credentials> {
        self.credentials
            .as_ref()
            .ok_or_else(|| ClientError::InvalidAuth("no credentials configured".to_string()))
    }

    /// Returns the oauth information, or an error in case it's not configured.
    pub(in crate) fn get_oauth(&self) -> ClientResult<&OAuth> {
        self.oauth
            .as_ref()
            .ok_or_else(|| ClientError::InvalidAuth("no oauth configured".to_string()))
    }

    /// Returns the oauth information as mutable, or an error in case it's not
    /// configured.
    pub(in crate) fn get_oauth_mut(&mut self) -> ClientResult<&mut OAuth> {
        self.oauth
            .as_mut()
            .ok_or_else(|| ClientError::InvalidAuth("no oauth configured".to_string()))
    }

    /// TODO: should be moved into a custom type
    fn get_uri(&self, _type: Type, _id: &str) -> String {
        format!("spotify:{}:{}", _type.as_str(), self.get_id(_type, _id))
    }

    /// Converts a JSON response from Spotify into its model.
    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> ClientResult<T> {
        serde_json::from_str::<T>(input).map_err(Into::into)
    }

    /// Get spotify id by type and id
    /// TODO: should be rewritten and moved into a separate type for IDs
    fn get_id(&self, _type: Type, id: &str) -> String {
        let mut _id = id.to_owned();
        let fields: Vec<&str> = _id.split(':').collect();
        let len = fields.len();
        if len >= 3 {
            if _type.as_str() != fields[len - 2] {
                error!(
                    "expected id of type {:?} but found type {:?} {:?}",
                    _type,
                    fields[len - 2],
                    _id
                );
            } else {
                return fields[len - 1].to_owned();
            }
        }
        let sfields: Vec<&str> = _id.split('/').collect();
        let len: usize = sfields.len();
        if len >= 3 {
            if _type.as_str() != sfields[len - 2] {
                error!(
                    "expected id of type {:?} but found type {:?} {:?}",
                    _type,
                    sfields[len - 2],
                    _id
                );
            } else {
                return sfields[len - 1].to_owned();
            }
        }
        _id.to_owned()
    }

    /// Parse the response code in the given response url
    ///
    /// Step 2 of the [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    ///
    /// TODO: this might be better off with an implementation from a separate
    /// library.
    pub fn parse_response_code(&self, url: &str) -> Option<String> {
        url.split("?code=")
            .nth(1)
	    .and_then(|s| s.split('&').next()).map(|s| s.to_string())
    }

    /// Append device ID to an API path.
    fn append_device_id(&self, path: &str, device_id: Option<String>) -> String {
        let mut new_path = path.to_string();
        if let Some(_device_id) = device_id {
            if path.contains('?') {
                new_path.push_str(&format!("&device_id={}", _device_id));
            } else {
                new_path.push_str(&format!("?device_id={}", _device_id));
            }
        }
        new_path
    }

    /// [get-track](https://developer.spotify.com/web-api/get-track/)
    /// returns a single track given the track's ID, URI or URL
    /// Parameters:
    /// - track_id - a spotify URI, URL or ID
    #[maybe_async]
    pub async fn track(&self, track_id: &str) -> ClientResult<FullTrack> {
        let trid = self.get_id(Type::Track, track_id);
        let url = format!("tracks/{}", trid);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get-several-tracks](https://developer.spotify.com/web-api/get-several-tracks/)
    /// returns a list of tracks given a list of track IDs, URIs, or URLs
    /// Parameters:
    /// - track_ids - a list of spotify URIs, URLs or IDs
    /// - market - an ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn tracks(
        &self,
        track_ids: Vec<&str>,
        market: Option<Country>,
    ) -> ClientResult<FullTracks> {
        let mut ids: Vec<String> = vec![];
        for track_id in track_ids {
            ids.push(self.get_id(Type::Track, track_id));
        }
        let url = format!("tracks/?ids={}", ids.join(","));
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "market", market);
        }
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get-artist](https://developer.spotify.com/web-api/get-artist/)
    /// returns a single artist given the artist's ID, URI or URL
    /// Parameters:
    /// - artist_id - an artist ID, URI or URL
    #[maybe_async]
    pub async fn artist(&self, artist_id: &str) -> ClientResult<FullArtist> {
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}", trid);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get-several-artists](https://developer.spotify.com/web-api/get-several-artists/)
    /// returns a list of artists given the artist IDs, URIs, or URLs
    /// Parameters:
    /// - artist_ids - a list of  artist IDs, URIs or URLs
    #[maybe_async]
    pub async fn artists(&self, artist_ids: Vec<String>) -> ClientResult<FullArtists> {
        let mut ids: Vec<String> = vec![];
        for artist_id in artist_ids {
            ids.push(self.get_id(Type::Artist, &artist_id));
        }
        let url = format!("artists/?ids={}", ids.join(","));
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get-artists-albums](https://developer.spotify.com/web-api/get-artists-albums/)
    /// Get Spotify catalog information about an artist's albums
    /// - artist_id - the artist ID, URI or URL
    /// - album_type - 'album', 'single', 'appears_on', 'compilation'
    /// - country - limit the response to one particular country.
    /// - limit  - the number of albums to return
    /// - offset - the index of the first album to return
    #[maybe_async]
    pub async fn artist_albums(
        &self,
        artist_id: &str,
        album_type: Option<AlbumType>,
        country: Option<Country>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedAlbum>> {
        let mut params = json!({});
        if let Some(limit) = limit {
            json_insert!(params, "limit", limit);
        }
        if let Some(album_type) = album_type {
            json_insert!(params, "album_type", album_type.as_str());
        }
        if let Some(offset) = offset {
            json_insert!(params, "offset", offset);
        }
        if let Some(country) = country {
            json_insert!(params, "country", country.as_str());
        }
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/albums", trid);
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get artists to tracks](https://developer.spotify.com/web-api/get-artists-top-tracks/)
    /// Get Spotify catalog information about an artist's top 10 tracks by country.
    /// Parameters:
    ///        - artist_id - the artist ID, URI or URL
    ///        - country - limit the response to one particular country.
    #[maybe_async]
    pub async fn artist_top_tracks<T: Into<Option<Country>>>(
        &self,
        artist_id: &str,
        country: T,
    ) -> ClientResult<FullTracks> {
        let params = json!({
            "country": country
                .into()
                .unwrap_or(Country::UnitedStates)
                .as_str()
        });

        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/top-tracks", trid);

        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get related artists](https://developer.spotify.com/web-api/get-related-artists/)
    /// Get Spotify catalog information about artists similar to an
    /// identified artist. Similarity is based on analysis of the
    /// Spotify community's listening history.
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    #[maybe_async]
    pub async fn artist_related_artists(&self, artist_id: &str) -> ClientResult<FullArtists> {
        let trid = self.get_id(Type::Artist, artist_id);
        let url = format!("artists/{}/related-artists", trid);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get album](https://developer.spotify.com/web-api/get-album/)
    /// returns a single album given the album's ID, URIs or URL
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    #[maybe_async]
    pub async fn album(&self, album_id: &str) -> ClientResult<FullAlbum> {
        let trid = self.get_id(Type::Album, album_id);
        let url = format!("albums/{}", trid);

        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get several albums](https://developer.spotify.com/web-api/get-several-albums/)
    /// returns a list of albums given the album IDs, URIs, or URLs
    /// Parameters:
    /// - albums_ids - a list of  album IDs, URIs or URLs
    #[maybe_async]
    pub async fn albums(&self, album_ids: Vec<String>) -> ClientResult<FullAlbums> {
        let mut ids: Vec<String> = vec![];
        for album_id in album_ids {
            ids.push(self.get_id(Type::Album, &album_id));
        }
        let url = format!("albums/?ids={}", ids.join(","));
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [search for items](https://developer.spotify.com/web-api/search-item/)
    /// Search for an Item
    /// Get Spotify catalog information about artists, albums, tracks or
    /// playlists that match a keyword string.
    /// Parameters:
    /// - q - the search query
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    /// - type - the type of item to return. One of 'artist', 'album', 'track',
    ///  'playlist', 'show' or 'episode'
    /// - market - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - include_external: Optional.Possible values: audio. If include_external=audio is specified the response will include any relevant audio content that is hosted externally.  
    #[maybe_async]
    pub async fn search<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        q: &str,
        _type: SearchType,
        limit: L,
        offset: O,
        market: Option<Country>,
        include_external: Option<IncludeExternal>,
    ) -> ClientResult<SearchResult> {
        let mut params = json! ({
            "limit": limit.into().unwrap_or(10),
            "offset": offset.into().unwrap_or(0),
            "q": q,
            "type": _type,
        });
        if let Some(market) = market {
            json_insert!(params, "market", market);
        }
        if let Some(include_external) = include_external {
            json_insert!(params, "include_external", include_external);
        }

        let result = self.get("search", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get albums tracks](https://developer.spotify.com/web-api/get-albums-tracks/)
    /// Get Spotify catalog information about an album's tracks
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    #[maybe_async]
    pub async fn album_track<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        album_id: &str,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<SimplifiedTrack>> {
        let params = json!({
            "limit": limit.into().unwrap_or(50).to_string(),
            "offset": offset.into().unwrap_or(0).to_string(),
        });
        let trid = self.get_id(Type::Album, album_id);
        let url = format!("albums/{}/tracks", trid);
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    ///[get users profile](https://developer.spotify.com/web-api/get-users-profile/)
    ///Gets basic profile information about a Spotify User
    ///Parameters:
    ///- user - the id of the usr
    #[maybe_async]
    pub async fn user(&self, user_id: &str) -> ClientResult<PublicUser> {
        let url = format!("users/{}", user_id);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get playlist](https://developer.spotify.com/documentation/web-api/reference/playlists/get-playlist/)
    /// Get full details about Spotify playlist
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - market - an ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn playlist(
        &self,
        playlist_id: &str,
        fields: Option<&str>,
        market: Option<Country>,
    ) -> ClientResult<FullPlaylist> {
        let mut params = json!({});
        if let Some(fields) = fields {
            json_insert!(params, "fields", fields);
        }
        if let Some(market) = market {
            json_insert!(params, "market", market.as_str());
        }

        let plid = self.get_id(Type::Playlist, playlist_id);
        let url = format!("playlists/{}", plid);
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get users playlists](https://developer.spotify.com/web-api/get-a-list-of-current-users-playlists/)
    /// Get current user playlists without required getting his profile
    /// Parameters:
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    #[maybe_async]
    pub async fn current_user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let params = json!({
            "limit": limit.into().unwrap_or(50).to_string(),
            "offset": offset.into().unwrap_or(0).to_string(),
        });

        let result = self.get("me/playlists", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get list users playlists](https://developer.spotify.com/web-api/get-list-users-playlists/)
    /// Gets playlists of a user
    /// Parameters:
    /// - user_id - the id of the usr
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    #[maybe_async]
    pub async fn user_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        user_id: &str,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let params = json!({
            "limit": limit.into().unwrap_or(50).to_string(),
            "offset": offset.into().unwrap_or(0).to_string(),
        });
        let url = format!("users/{}/playlists", user_id);
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get list users playlists](https://developer.spotify.com/web-api/get-list-users-playlists/)
    /// Gets playlist of a user
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    #[maybe_async]
    pub async fn user_playlist(
        &self,
        user_id: &str,
        playlist_id: Option<&mut str>,
        fields: Option<&str>,
        market: Option<Country>,
    ) -> ClientResult<FullPlaylist> {
        let mut params = json!({});
        if let Some(fields) = fields {
            json_insert!(params, "fields", fields);
        }
        if let Some(market) = market {
            json_insert!(params, "market", market.as_str());
        }
        match playlist_id {
            Some(playlist_id) => {
                let plid = self.get_id(Type::Playlist, playlist_id);
                let url = format!("users/{}/playlists/{}", user_id, plid);
                let result = self.get(&url, None, Some(&params)).await?;
                self.convert_result(&result)
            }
            None => {
                let url = format!("users/{}/starred", user_id);
                let result = self.get(&url, None, Some(&params)).await?;
                self.convert_result(&result)
            }
        }
    }

    /// [get playlists tracks](https://developer.spotify.com/web-api/get-playlists-tracks/)
    /// Get full details of the tracks of a playlist owned by a user
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    /// - limit - the maximum number of tracks to return
    /// - offset - the index of the first track to return
    /// - market - an ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn user_playlist_tracks<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        user_id: &str,
        playlist_id: &str,
        fields: Option<&str>,
        limit: L,
        offset: O,
        market: Option<Country>,
    ) -> ClientResult<Page<PlaylistTrack>> {
        let mut params = json!({
            "limit": limit.into().unwrap_or(50).to_string(),
            "offset": offset.into().unwrap_or(0).to_string(),
        });
        if let Some(market) = market {
            json_insert!(params, "market", market.as_str());
        }
        if let Some(fields) = fields {
            json_insert!(params, "fields", fields);
        }
        let plid = self.get_id(Type::Playlist, playlist_id);
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [create playlist](https://developer.spotify.com/web-api/create-playlist/)
    /// Creates a playlist for a user
    /// Parameters:
    /// - user_id - the id of the user
    /// - name - the name of the playlist
    /// - public - is the created playlist public
    /// - description - the description of the playlist
    #[maybe_async]
    pub async fn user_playlist_create<P: Into<Option<bool>>, D: Into<Option<String>>>(
        &self,
        user_id: &str,
        name: &str,
        public: P,
        description: D,
    ) -> ClientResult<FullPlaylist> {
        let public = public.into().unwrap_or(true);
        let description = description.into().unwrap_or_else(|| "".to_owned());
        let params = json!({
            "name": name,
            "public": public,
            "description": description
        });
        let url = format!("users/{}/playlists", user_id);
        let result = self.post(&url, None, &params).await?;
        self.convert_result(&result)
    }

    /// [change playlists details](https://developer.spotify.com/web-api/change-playlist-details/)
    /// Changes a playlist's name and/or public/private state
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - name - optional name of the playlist
    /// - public - optional is the playlist public
    /// - collaborative - optional is the playlist collaborative
    /// - description - optional description of the playlist
    #[maybe_async]
    pub async fn user_playlist_change_detail(
        &self,
        user_id: &str,
        playlist_id: &str,
        name: Option<&str>,
        public: Option<bool>,
        description: Option<String>,
        collaborative: Option<bool>,
    ) -> ClientResult<String> {
        let mut params = json!({});
        if let Some(name) = name {
            json_insert!(params, "name", name);
        }
        if let Some(public) = public {
            json_insert!(params, "public", public);
        }
        if let Some(collaborative) = collaborative {
            json_insert!(params, "collaborative", collaborative);
        }
        if let Some(description) = description {
            json_insert!(params, "description", description);
        }
        let url = format!("users/{}/playlists/{}", user_id, playlist_id);
        self.put(&url, None, &params).await
    }

    /// [unfollow playlist](https://developer.spotify.com/web-api/unfollow-playlist/)
    /// Unfollows (deletes) a playlist for a user
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    #[maybe_async]
    pub async fn user_playlist_unfollow(
        &self,
        user_id: &str,
        playlist_id: &str,
    ) -> ClientResult<String> {
        let url = format!("users/{}/playlists/{}/followers", user_id, playlist_id);
        self.delete(&url, None, &json!({})).await
    }

    /// [add tracks to playlist](https://developer.spotify.com/web-api/add-tracks-to-playlist/)
    /// Adds tracks to a playlist
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - track_ids - a list of track URIs, URLs or IDs
    /// - position - the position to add the tracks
    #[maybe_async]
    pub async fn user_playlist_add_tracks(
        &self,
        user_id: &str,
        playlist_id: &str,
        track_ids: &[String],
        position: Option<i32>,
    ) -> ClientResult<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        let mut params = json!({ "uris": uris });
        if let Some(position) = position {
            json_insert!(params, "position", position);
        }
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.post(&url, None, &params).await?;
        self.convert_result(&result)
    }
    ///[replaced playlists tracks](https://developer.spotify.com/web-api/replace-playlists-tracks/)
    ///Replace all tracks in a playlist
    ///Parameters:
    ///- user - the id of the user
    ///- playlist_id - the id of the playlist
    ///- tracks - the list of track ids to add to the playlist
    #[maybe_async]
    pub async fn user_playlist_replace_tracks(
        &self,
        user_id: &str,
        playlist_id: &str,
        track_ids: &[String],
    ) -> ClientResult<()> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();
        // let mut params = Map::new();
        // params.insert("uris".to_owned(), uris.into());
        let params = json!({ "uris": uris });
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        self.put(&url, None, &params).await?;

        Ok(())
    }

    /// [reorder playlists tracks](https://developer.spotify.com/web-api/reorder-playlists-tracks/)
    /// Reorder tracks in a playlist
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - range_start - the position of the first track to be reordered
    /// - range_length - optional the number of tracks to be reordered (default: 1)
    /// - insert_before - the position where the tracks should be inserted
    /// - snapshot_id - optional playlist's snapshot ID
    #[maybe_async]
    pub async fn user_playlist_recorder_tracks<R: Into<Option<u32>>>(
        &self,
        user_id: &str,
        playlist_id: &str,
        range_start: i32,
        range_length: R,
        insert_before: i32,
        snapshot_id: Option<String>,
    ) -> ClientResult<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let mut params = json! ({
            "range_start": range_start,
            "range_length": range_length.into().unwrap_or(1),
            "insert_before": insert_before
        });
        if let Some(snapshot_id) = snapshot_id {
            json_insert!(params, "snapshot_id", snapshot_id);
        }

        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.put(&url, None, &params).await?;
        self.convert_result(&result)
    }

    /// [remove tracks playlist](https://developer.spotify.com/web-api/remove-tracks-playlist/)
    /// Removes all occurrences of the given tracks from the given playlist
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - track_ids - the list of track ids to add to the playlist
    /// - snapshot_id - optional id of the playlist snapshot
    #[maybe_async]
    pub async fn user_playlist_remove_all_occurrences_of_tracks(
        &self,
        user_id: &str,
        playlist_id: &str,
        track_ids: &[String],
        snapshot_id: Option<String>,
    ) -> ClientResult<CUDResult> {
        let plid = self.get_id(Type::Playlist, playlist_id);
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_uri(Type::Track, id))
            .collect();

        // TODO: this can be improved
        let mut tracks: Vec<Map<String, Value>> = vec![];
        for uri in uris {
            let mut map = Map::new();
            map.insert("uri".to_owned(), uri.into());
            tracks.push(map);
        }
        let mut params = json!({ "tracks": tracks });
        if let Some(snapshot_id) = snapshot_id {
            json_insert!(params, "snapshot_id", snapshot_id);
        }
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.delete(&url, None, &params).await?;
        self.convert_result(&result)
    }

    /// [remove tracks playlist](https://developer.spotify.com/web-api/remove-tracks-playlist/)
    /// Removes specfic occurrences of the given tracks from the given playlist
    /// Parameters:
    /// - user_id: the id of the user
    /// - playlist_id: the id of the playlist
    /// - tracks: an array of map containing Spotify URIs of the tracks to remove
    /// with their current positions in the playlist. For example:
    ///
    /// ```json
    /// {
    ///    "tracks":[
    ///       {
    ///          "uri":"spotify:track:4iV5W9uYEdYUVa79Axb7Rh",
    ///          "positions":[
    ///             0,
    ///             3
    ///          ]
    ///       },
    ///       {
    ///          "uri":"spotify:track:1301WleyT98MSxVHPZCA6M",
    ///          "positions":[
    ///             7
    ///          ]
    ///       }
    ///    ]
    /// }
    /// ```
    /// - snapshot_id: optional id of the playlist snapshot
    #[maybe_async]
    pub async fn user_playlist_remove_specific_occurrences_of_tracks(
        &self,
        user_id: &str,
        playlist_id: &str,
        tracks: Vec<Map<String, Value>>,
        snapshot_id: Option<String>,
    ) -> ClientResult<CUDResult> {
        // TODO: this can be improved
        let plid = self.get_id(Type::Playlist, playlist_id);
        let mut ftracks: Vec<Map<String, Value>> = vec![];
        for track in tracks {
            let mut map = Map::new();
            if let Some(_uri) = track.get("uri") {
                let uri = self.get_uri(Type::Track, &_uri.as_str().unwrap().to_owned());
                map.insert("uri".to_owned(), uri.into());
            }
            if let Some(_position) = track.get("position") {
                map.insert("position".to_owned(), _position.to_owned());
            }
            ftracks.push(map);
        }

        let mut params = json!({ "tracks": ftracks });
        if let Some(snapshot_id) = snapshot_id {
            json_insert!(params, "snapshot_id", snapshot_id);
        }
        let url = format!("users/{}/playlists/{}/tracks", user_id, plid);
        let result = self.delete(&url, None, &params).await?;
        self.convert_result(&result)
    }

    /// [follow playlist](https://developer.spotify.com/web-api/follow-playlist/)
    /// Add the current authenticated user as a follower of a playlist.
    /// Parameters:
    /// - playlist_owner_id - the user id of the playlist owner
    /// - playlist_id - the id of the playlist
    #[maybe_async]
    pub async fn user_playlist_follow_playlist<P: Into<Option<bool>>>(
        &self,
        playlist_owner_id: &str,
        playlist_id: &str,
        public: P,
    ) -> ClientResult<()> {
        let url = format!(
            "users/{}/playlists/{}/followers",
            playlist_owner_id, playlist_id
        );

        self.put(
            &url,
            None,
            &json! ({
                "public": public.into().unwrap_or(true)
            }),
        )
        .await?;

        Ok(())
    }

    /// [check user following playlist](https://developer.spotify.com/web-api/check-user-following-playlist/)
    /// Check to see if the given users are following the given playlist
    /// Parameters:
    /// - playlist_owner_id - the user id of the playlist owner
    /// - playlist_id - the id of the playlist
    /// - user_ids - the ids of the users that you want to
    /// check to see if they follow the playlist. Maximum: 5 ids.
    #[maybe_async]
    pub async fn user_playlist_check_follow(
        &self,
        playlist_owner_id: &str,
        playlist_id: &str,
        user_ids: &[String],
    ) -> ClientResult<Vec<bool>> {
        if user_ids.len() > 5 {
            error!("The maximum length of user ids is limited to 5 :-)");
        }
        let url = format!(
            "users/{}/playlists/{}/followers/contains?ids={}",
            playlist_owner_id,
            playlist_id,
            user_ids.join(",")
        );
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }
    /// [get current users profile](https://developer.spotify.com/web-api/get-current-users-profile/)
    /// Get detailed profile information about the current user.
    /// An alias for the 'current_user' method.
    #[maybe_async]
    pub async fn me(&self) -> ClientResult<PrivateUser> {
        let result = self.get("me/", None, None).await?;
        self.convert_result(&result)
    }
    /// Get detailed profile information about the current user.
    /// An alias for the 'me' method.
    #[maybe_async]
    pub async fn current_user(&self) -> ClientResult<PrivateUser> {
        self.me().await
    }

    ///  [get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
    ///  Get information about the current users currently playing track.
    #[maybe_async]
    pub async fn current_user_playing_track(&self) -> ClientResult<Option<Playing>> {
        let result = self.get("me/player/currently-playing", None, None).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// [get user saved albums](https://developer.spotify.com/web-api/get-users-saved-albums/)
    /// Gets a list of the albums saved in the current authorized user's
    /// "Your Music" library
    /// Parameters:
    /// - limit - the number of albums to return
    /// - offset - the index of the first album to return
    /// - market - Provide this parameter if you want to apply Track Relinking.
    #[maybe_async]
    pub async fn current_user_saved_albums<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<SavedAlbum>> {
        let result = self
            .get(
                "me/albums",
                None,
                Some(&json!({
                    "limit": limit.into().unwrap_or(20),
                    "offset": offset.into().unwrap_or(0),
                })),
            )
            .await?;
        self.convert_result(&result)
    }
    ///[get users saved tracks](https://developer.spotify.com/web-api/get-users-saved-tracks/)
    ///Parameters:
    ///- limit - the number of tracks to return
    ///- offset - the index of the first track to return
    ///- market - Provide this parameter if you want to apply Track Relinking.
    #[maybe_async]
    pub async fn current_user_saved_tracks<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<SavedTrack>> {
        let result = self
            .get(
                "me/tracks",
                None,
                Some(&json!({
                    "limit": limit.into().unwrap_or(20),
                    "offset": offset.into().unwrap_or(0),
                })),
            )
            .await?;
        self.convert_result(&result)
    }
    ///[get followed artists](https://developer.spotify.com/web-api/get-followed-artists/)
    ///Gets a list of the artists followed by the current authorized user
    ///Parameters:
    ///- limit - the number of tracks to return
    ///- after - ghe last artist ID retrieved from the previous request
    #[maybe_async]
    pub async fn current_user_followed_artists<L: Into<Option<u32>>>(
        &self,
        limit: L,
        after: Option<String>,
    ) -> ClientResult<CursorPageFullArtists> {
        let mut params = json!({
            "limit": limit.into().unwrap_or(20),
            "type": Type::Artist.as_str()
        });
        if let Some(after) = after {
            json_insert!(params, "after", after);
        }

        let result = self.get("me/following", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [remove tracks users](https://developer.spotify.com/web-api/remove-tracks-user/)
    /// Remove one or more tracks from the current user's
    /// "Your Music" library.
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_tracks_delete(&self, track_ids: &[String]) -> ClientResult<()> {
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/?ids={}", uris.join(","));
        self.delete(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [check users saved tracks](https://developer.spotify.com/web-api/check-users-saved-tracks/)
    /// Check if one or more tracks is already saved in
    /// the current Spotify user’s “Your Music” library.
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_tracks_contains(
        &self,
        track_ids: &[String],
    ) -> ClientResult<Vec<bool>> {
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/contains/?ids={}", uris.join(","));
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [save tracks user ](https://developer.spotify.com/web-api/save-tracks-user/)
    /// Save one or more tracks to the current user's
    /// "Your Music" library.
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_tracks_add(&self, track_ids: &[String]) -> ClientResult<()> {
        let uris: Vec<String> = track_ids
            .iter()
            .map(|id| self.get_id(Type::Track, id))
            .collect();
        let url = format!("me/tracks/?ids={}", uris.join(","));
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [get users  top artists and tracks](https://developer.spotify.com/web-api/get-users-top-artists-and-tracks/)
    /// Get the current user's top artists
    /// Parameters:
    /// - limit - the number of entities to return
    /// - offset - the index of the first entity to return
    /// - time_range - Over what time frame are the affinities computed
    #[maybe_async]
    pub async fn current_user_top_artists<
        L: Into<Option<u32>>,
        O: Into<Option<u32>>,
        T: Into<Option<TimeRange>>,
    >(
        &self,
        limit: L,
        offset: O,
        time_range: T,
    ) -> ClientResult<Page<FullArtist>> {
        let result = self
            .get(
                &"me/top/artists",
                None,
                Some(&json! ({
                    "limit": limit.into().unwrap_or(20),
                    "offset": offset.into().unwrap_or(0),
                    "time_range": time_range.into().unwrap_or(TimeRange::MediumTerm),
                })),
            )
            .await?;
        self.convert_result(&result)
    }

    /// [get users top artists and tracks](https://developer.spotify.com/web-api/get-users-top-artists-and-tracks/)
    /// Get the current user's top tracks
    /// Parameters:
    /// - limit - the number of entities to return
    /// - offset - the index of the first entity to return
    /// - time_range - Over what time frame are the affinities computed
    #[maybe_async]
    pub async fn current_user_top_tracks<
        L: Into<Option<u32>>,
        O: Into<Option<u32>>,
        T: Into<Option<TimeRange>>,
    >(
        &self,
        limit: L,
        offset: O,
        time_range: T,
    ) -> ClientResult<Page<FullTrack>> {
        let result = self
            .get(
                "me/top/tracks",
                None,
                Some(&json!({
                    "limit": limit.into().unwrap_or(20),
                    "offset": offset.into().unwrap_or(0),
                    "time_range": time_range.into().unwrap_or(TimeRange::MediumTerm),
                })),
            )
            .await?;
        self.convert_result(&result)
    }

    /// [get recently played](https://developer.spotify.com/web-api/web-api-personalization-endpoints/get-recently-played/)
    /// Get the current user's recently played tracks
    /// Parameters:
    /// - limit - the number of entities to return
    #[maybe_async]
    pub async fn current_user_recently_played<L: Into<Option<u32>>>(
        &self,
        limit: L,
    ) -> ClientResult<CursorBasedPage<PlayHistory>> {
        let result = self
            .get(
                "me/player/recently-played",
                None,
                Some(&json!({
                    "limit": limit.into().unwrap_or(50)
                })),
            )
            .await?;
        self.convert_result(&result)
    }

    /// [save albums user](https://developer.spotify.com/web-api/save-albums-user/)
    /// Add one or more albums to the current user's
    /// "Your Music" library.
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_albums_add(&self, album_ids: &[String]) -> ClientResult<()> {
        let uris: Vec<String> = album_ids
            .iter()
            .map(|id| self.get_id(Type::Album, id))
            .collect();
        let url = format!("me/albums/?ids={}", uris.join(","));
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [remove albums user](https://developer.spotify.com/documentation/web-api/reference/library/remove-albums-user/)
    /// Remove one or more albums from the current user's
    /// "Your Music" library.
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_albums_delete(&self, album_ids: &[String]) -> ClientResult<()> {
        let uris: Vec<String> = album_ids
            .iter()
            .map(|id| self.get_id(Type::Album, id))
            .collect();
        let url = format!("me/albums/?ids={}", uris.join(","));
        self.delete(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [check users saved albums](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-albums/)
    /// Check if one or more albums is already saved in
    /// the current Spotify user’s “Your Music” library.
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    #[maybe_async]
    pub async fn current_user_saved_albums_contains(
        &self,
        album_ids: &[String],
    ) -> ClientResult<Vec<bool>> {
        let uris: Vec<String> = album_ids
            .iter()
            .map(|id| self.get_id(Type::Album, id))
            .collect();
        let url = format!("me/albums/contains/?ids={}", uris.join(","));
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [follow artists users](https://developer.spotify.com/web-api/follow-artists-users/)
    /// Follow one or more artists
    /// Parameters:
    /// - artist_ids - a list of artist IDs
    #[maybe_async]
    pub async fn user_follow_artists(&self, artist_ids: &[String]) -> ClientResult<()> {
        let url = format!("me/following?type=artist&ids={}", artist_ids.join(","));
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [unfollow artists users](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/)
    /// Unfollow one or more artists
    /// Parameters:
    /// - artist_ids - a list of artist IDs
    #[maybe_async]
    pub async fn user_unfollow_artists(&self, artist_ids: &[String]) -> ClientResult<()> {
        let url = format!("me/following?type=artist&ids={}", artist_ids.join(","));
        self.delete(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [check user following
    /// artists](https://developer.spotify.com/web-api/checkcurrent-user-follows/)
    /// Check to see if the given users are following the given artists
    /// Parameters:
    /// - artist_ids - the ids of the users that you want to
    #[maybe_async]
    pub async fn user_artist_check_follow(&self, artsit_ids: &[String]) -> ClientResult<Vec<bool>> {
        let url = format!(
            "me/following/contains?type=artist&ids={}",
            artsit_ids.join(",")
        );
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [follow artists users](https://developer.spotify.com/web-api/follow-artists-users/)
    /// Follow one or more users
    /// Parameters:
    /// - user_ids - a list of artist IDs
    #[maybe_async]
    pub async fn user_follow_users(&self, user_ids: &[String]) -> ClientResult<()> {
        let url = format!("me/following?type=user&ids={}", user_ids.join(","));
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [unfollow artists users](https://developer.spotify.com/documentation/web-api/reference/follow/unfollow-artists-users/)
    /// Unfollow one or more users
    /// Parameters:
    /// - user_ids - a list of artist IDs
    #[maybe_async]
    pub async fn user_unfollow_users(&self, user_ids: &[String]) -> ClientResult<()> {
        let url = format!("me/following?type=user&ids={}", user_ids.join(","));
        self.delete(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [get list featured playlists](https://developer.spotify.com/web-api/get-list-featured-playlists/)
    /// Get a list of Spotify featured playlists
    /// Parameters:
    /// - locale - The desired language, consisting of a lowercase ISO
    /// 639 language code and an uppercase ISO 3166-1 alpha-2 country
    /// code, joined by an underscore.
    /// - country - An ISO 3166-1 alpha-2 country code.
    /// - timestamp - A timestamp in ISO 8601 format:
    /// yyyy-MM-ddTHH:mm:ss. Use this parameter to specify the user's
    /// local time to get results tailored for that specific date and
    /// time in the day
    /// - limit - The maximum number of items to return. Default: 20.
    /// Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0
    /// (the first object). Use with limit to get the next set of
    /// items.
    #[maybe_async]
    pub async fn featured_playlists<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        locale: Option<String>,
        country: Option<Country>,
        timestamp: Option<DateTime<Utc>>,
        limit: L,
        offset: O,
    ) -> ClientResult<FeaturedPlaylists> {
        let mut params = json!({
            "limit": limit.into().unwrap_or(20),
            "offset": offset.into().unwrap_or(0),
        });
        if let Some(locale) = locale {
            json_insert!(params, "locale", locale);
        }
        if let Some(country) = country {
            json_insert!(params, "country", country.as_str());
        }
        if let Some(timestamp) = timestamp {
            json_insert!(params, "timestamp", timestamp.to_rfc3339());
        }
        let result = self
            .get("browse/featured-playlists", None, Some(&params))
            .await?;
        self.convert_result(&result)
    }

    /// [get list new releases](https://developer.spotify.com/web-api/get-list-new-releases/)
    /// Get a list of new album releases featured in Spotify
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code.
    /// - limit - The maximum number of items to return. Default: 20.
    /// Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0
    /// (the first object). Use with limit to get the next set of
    /// items.
    #[maybe_async]
    pub async fn new_releases<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        country: Option<Country>,
        limit: L,
        offset: O,
    ) -> ClientResult<PageSimpliedAlbums> {
        let mut params = json! ({
            "limit": limit.into().unwrap_or(20),
            "offset": offset.into().unwrap_or(0),
        });
        if let Some(country) = country {
            json_insert!(params, "country", country);
        }

        let result = self.get("browse/new-releases", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get list categories](https://developer.spotify.com/web-api/get-list-categories/)
    /// Get a list of new album releases featured in Spotify
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code.
    /// - locale - The desired language, consisting of an ISO 639
    /// language code and an ISO 3166-1 alpha-2 country code, joined
    /// by an underscore.
    /// - limit - The maximum number of items to return. Default: 20.
    /// Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0
    /// (the first object). Use with limit to get the next set of
    /// items.
    #[maybe_async]
    pub async fn categories<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        locale: Option<String>,
        country: Option<Country>,
        limit: L,
        offset: O,
    ) -> ClientResult<PageCategory> {
        let mut params = json!({
            "limit": limit.into().unwrap_or(20),
            "offset": offset.into().unwrap_or(0),
        });
        if let Some(locale) = locale {
            json_insert!(params, "locale", locale);
        }
        if let Some(country) = country {
            json_insert!(params, "country", country);
        }
        let result = self.get("browse/categories", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get recommendtions](https://developer.spotify.com/web-api/get-recommendations/)
    /// Get Recommendations Based on Seeds
    /// Parameters:
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
    #[maybe_async]
    pub async fn recommendations<L: Into<Option<u32>>>(
        &self,
        seed_artists: Option<Vec<String>>,
        seed_genres: Option<Vec<String>>,
        seed_tracks: Option<Vec<String>>,
        limit: L,
        country: Option<Country>,
        payload: &Map<String, Value>,
    ) -> ClientResult<Recommendations> {
        let mut params = json! ({
            "limit": limit.into().unwrap_or(20),
        });
        if let Some(seed_artists) = seed_artists {
            let seed_artists_ids = seed_artists
                .iter()
                .map(|id| self.get_id(Type::Artist, id))
                .collect::<Vec<_>>();
            json_insert!(params, "seed_artists", seed_artists_ids.join(","));
        }
        if let Some(seed_genres) = seed_genres {
            json_insert!(params, "seed_genres", seed_genres.join(","));
        }
        if let Some(seed_tracks) = seed_tracks {
            let seed_tracks_ids = seed_tracks
                .iter()
                .map(|id| self.get_id(Type::Track, id))
                .collect::<Vec<_>>();
            json_insert!(params, "seed_tracks", seed_tracks_ids.join(","));
        }
        if let Some(country) = country {
            json_insert!(params, "market", country.as_str());
        }
        let attributes = [
            "acousticness",
            "danceability",
            "duration_ms",
            "energy",
            "instrumentalness",
            "key",
            "liveness",
            "loudness",
            "mode",
            "popularity",
            "speechiness",
            "tempo",
            "time_signature",
            "valence",
        ];
        let prefixes = ["min", "max", "target"];
        for attribute in attributes.iter() {
            for prefix in prefixes.iter() {
                let param = format!("{}_{}", prefix, attribute);
                if let Some(value) = payload.get(&param) {
                    json_insert!(params, param, value);
                }
            }
        }
        let result = self.get("recommendations", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// [get audio features](https://developer.spotify.com/web-api/get-audio-features/)
    /// Get audio features for a track
    /// - track - track URI, URL or ID
    #[maybe_async]
    pub async fn audio_features(&self, track: &str) -> ClientResult<AudioFeatures> {
        let track_id = self.get_id(Type::Track, track);
        let url = format!("audio-features/{}", track_id);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get several audio features](https://developer.spotify.com/web-api/get-several-audio-features/)
    /// Get Audio Features for Several Tracks
    /// - tracks a list of track URIs, URLs or IDs
    #[maybe_async]
    pub async fn audios_features(
        &self,
        tracks: &[String],
    ) -> ClientResult<Option<AudioFeaturesPayload>> {
        let ids: Vec<String> = tracks
            .iter()
            .map(|track| self.get_id(Type::Track, track))
            .collect();
        let url = format!("audio-features/?ids={}", ids.join(","));

        let result = self.get(&url, None, None).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// [get audio analysis](https://developer.spotify.com/web-api/get-audio-analysis/)
    /// Get Audio Analysis for a Track
    /// Parameters:
    /// - track_id - a track URI, URL or ID
    #[maybe_async]
    pub async fn audio_analysis(&self, track: &str) -> ClientResult<AudioAnalysis> {
        let trid = self.get_id(Type::Track, track);
        let url = format!("audio-analysis/{}", trid);
        let result = self.get(&url, None, None).await?;
        self.convert_result(&result)
    }

    /// [get a users available devices](https://developer.spotify.com/web-api/get-a-users-available-devices/)
    /// Get a User’s Available Devices
    #[maybe_async]
    pub async fn device(&self) -> ClientResult<DevicePayload> {
        let result = self.get("me/player/devices", None, None).await?;
        self.convert_result(&result)
    }

    /// [get informatation about the users  current playback](https://developer.spotify.com/web-api/get-information-about-the-users-current-playback/)
    /// Get Information About The User’s Current Playback
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code.
    /// - additional_types: Optional. A comma-separated list of item types that your client supports besides the default track type. Valid types are: `track` and `episode`.
    #[maybe_async]
    pub async fn current_playback(
        &self,
        market: Option<Country>,
        additional_types: Option<Vec<AdditionalType>>,
    ) -> ClientResult<Option<CurrentlyPlaybackContext>> {
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }
        if let Some(additional_types) = additional_types {
            json_insert!(
                params,
                "additional_types",
                additional_types
                    .iter()
                    .map(|&x| x.as_str().to_owned())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }

        let result = self.get("me/player", None, Some(&params)).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// [get the users currently playing track](https://developer.spotify.com/web-api/get-the-users-currently-playing-track/)
    /// Get the User’s Currently Playing Track
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code.
    /// - additional_types: Optional. A comma-separated list of item types that your client supports besides the default track type. Valid types are: `track` and `episode`.
    #[maybe_async]
    pub async fn current_playing(
        &self,
        market: Option<Country>,
        additional_types: Option<Vec<AdditionalType>>,
    ) -> ClientResult<Option<CurrentlyPlayingContext>> {
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }
        if let Some(additional_types) = additional_types {
            json_insert!(
                params,
                "additional_types",
                additional_types
                    .iter()
                    .map(|&x| x.as_str().to_owned())
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }

        let result = self
            .get("me/player/currently-playing", None, Some(&params))
            .await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// [transfer a users playback](https://developer.spotify.com/web-api/transfer-a-users-playback/)
    /// Transfer a User’s Playback
    /// Note: Although an array is accepted, only a single device_id is currently
    /// supported. Supplying more than one will return 400 Bad Request
    /// Parameters:
    /// - device_id - transfer playback to this device
    /// - force_play - true: after transfer, play. false:
    /// keep current state.
    #[maybe_async]
    pub async fn transfer_playback<T: Into<Option<bool>>>(
        &self,
        device_id: &str,
        force_play: T,
    ) -> ClientResult<()> {
        self.put(
            "me/player",
            None,
            &json! ({
                "device_ids": vec![device_id.to_owned()],
                "play": force_play.into().unwrap_or(true)
            }),
        )
        .await?;

        Ok(())
    }

    /// [start a users playback](https://developer.spotify.com/web-api/start-a-users-playback/)
    /// Start/Resume a User’s Playback
    /// Provide a `context_uri` to start playback or a album,
    /// artist, or playlist.
    ///
    /// Provide a `uris` list to start playback of one or more
    /// tracks.
    ///
    /// Provide `offset` as {"position": <int>} or {"uri": "<track uri>"}
    /// to start playback at a particular offset.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    /// - context_uri - spotify context uri to play
    /// - uris - spotify track uris
    /// - offset - offset into context by index or track
    /// - position_ms - Indicates from what position to start playback.
    #[maybe_async]
    pub async fn start_playback(
        &self,
        device_id: Option<String>,
        context_uri: Option<String>,
        uris: Option<Vec<String>>,
        offset: Option<super::model::offset::Offset>,
        position_ms: Option<u32>,
    ) -> ClientResult<()> {
        if context_uri.is_some() && uris.is_some() {
            error!("specify either contexxt uri or uris, not both");
        }
        let mut params = json!({});
        if let Some(context_uri) = context_uri {
            json_insert!(params, "context_uri", context_uri);
        }
        if let Some(uris) = uris {
            json_insert!(params, "uris", uris);
        }
        if let Some(offset) = offset {
            if let Some(position) = offset.position {
                json_insert!(params, "offset", json!({ "position": position }));
            } else if let Some(uri) = offset.uri {
                json_insert!(params, "offset", json!({ "uri": uri }));
            }
        }
        if let Some(position_ms) = position_ms {
            json_insert!(params, "position_ms", position_ms);
        };
        let url = self.append_device_id("me/player/play", device_id);
        self.put(&url, None, &params).await?;

        Ok(())
    }

    /// [pause a users playback](https://developer.spotify.com/web-api/pause-a-users-playback/)
    /// Pause a User’s Playback
    /// Parameters:
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn pause_playback(&self, device_id: Option<String>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/pause", device_id);
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [skip users playback to the next track](https://developer.spotify.com/web-api/skip-users-playback-to-next-track/)
    /// Skip User’s Playback To Next Track
    /// Parameters:
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn next_track(&self, device_id: Option<String>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/next", device_id);
        self.post(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [skip users playback to previous track](https://developer.spotify.com/web-api/skip-users-playback-to-previous-track/)
    /// Skip User’s Playback To Previous Track
    /// Parameters:
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn previous_track(&self, device_id: Option<String>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/previous", device_id);
        self.post(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [seek-to-position-in-currently-playing-track/](https://developer.spotify.com/web-api/seek-to-position-in-currently-playing-track/)
    /// Seek To Position In Currently Playing Track
    /// Parameters:
    /// - position_ms - position in milliseconds to seek to
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn seek_track(
        &self,
        position_ms: u32,
        device_id: Option<String>,
    ) -> ClientResult<()> {
        let url = self.append_device_id(
            &format!("me/player/seek?position_ms={}", position_ms),
            device_id,
        );
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [set repeat mode on users playback](https://developer.spotify.com/web-api/set-repeat-mode-on-users-playback/)
    /// Set Repeat Mode On User’s Playback
    /// Parameters:
    ///  - state - `track`, `context`, or `off`
    ///  - device_id - device target for playback
    #[maybe_async]
    pub async fn repeat(&self, state: RepeatState, device_id: Option<String>) -> ClientResult<()> {
        let url = self.append_device_id(
            &format!("me/player/repeat?state={}", state.as_str()),
            device_id,
        );
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [set-volume-for-users-playback](https://developer.spotify.com/web-api/set-volume-for-users-playback/)
    /// Set Volume For User’s Playback
    /// Parameters:
    /// - volume_percent - volume between 0 and 100
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn volume(&self, volume_percent: u8, device_id: Option<String>) -> ClientResult<()> {
        if volume_percent > 100u8 {
            error!("volume must be between 0 and 100, inclusive");
        }
        let url = self.append_device_id(
            &format!("me/player/volume?volume_percent={}", volume_percent),
            device_id,
        );
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [toggle shuffle for user playback](https://developer.spotify.com/web-api/toggle-shuffle-for-users-playback/)
    /// Toggle Shuffle For User’s Playback
    /// Parameters:
    /// - state - true or false
    /// - device_id - device target for playback
    #[maybe_async]
    pub async fn shuffle(&self, state: bool, device_id: Option<String>) -> ClientResult<()> {
        let url = self.append_device_id(&format!("me/player/shuffle?state={}", state), device_id);
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [Add an item to the end fo the user's current playback queue](https://developer.spotify.com/console/post-queue/)
    /// Add an item to the end of the user's playback queue
    /// Parameters:
    /// - uri - THe uri of the item to add, Track or Episode
    /// - device id - The id of the device targeting
    /// - If no device ID provided the user's currently active device is targeted
    #[maybe_async]
    pub async fn add_item_to_queue(
        &self,
        item: String,
        device_id: Option<String>,
    ) -> ClientResult<()> {
        let url = self.append_device_id(&format!("me/player/queue?uri={}", &item), device_id);
        self.post(&url, None, &json!({})).await?;

        Ok(())
    }

    /// [Save Shows for Current User](https://developer.spotify.com/console/put-current-user-saved-shows)
    /// Add a show or a list of shows to a user’s library
    /// Parameters:
    /// - ids(Required) A comma-separated list of Spotify IDs for the shows to be added to the user’s library.
    #[maybe_async]
    pub async fn save_shows(&self, ids: Vec<String>) -> ClientResult<()> {
        let joined_ids = ids.join(",");
        let url = format!("me/shows/?ids={}", joined_ids);
        self.put(&url, None, &json!({})).await?;

        Ok(())
    }

    /// Get a list of shows saved in the current Spotify user’s library. Optional parameters can be used to limit the number of shows returned.
    /// [Get user's saved shows](https://developer.spotify.com/documentation/web-api/reference/library/get-users-saved-shows/)
    /// - limit(Optional). The maximum number of shows to return. Default: 20. Minimum: 1. Maximum: 50
    /// - offset(Optional). The index of the first show to return. Default: 0 (the first object). Use with limit to get the next set of shows.
    #[maybe_async]
    pub async fn get_saved_show<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        limit: L,
        offset: O,
    ) -> ClientResult<Page<Show>> {
        let result = self
            .get(
                "me/shows",
                None,
                Some(&json! ({
                    "limit": limit.into().unwrap_or(20),
                    "offset": offset.into().unwrap_or(0)
                })),
            )
            .await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for a single show identified by its unique Spotify ID.
    /// [Get a show](https://developer.spotify.com/documentation/web-api/reference/shows/get-a-show/)
    /// Path Parameters:
    /// - id: The Spotify ID for the show.
    /// Query Parameters
    /// - market(Optional): An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn get_a_show(&self, id: String, market: Option<Country>) -> ClientResult<FullShow> {
        let url = format!("shows/{}", id);
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for multiple shows based on their Spotify IDs.
    /// [Get seversal shows](https://developer.spotify.com/documentation/web-api/reference/shows/get-several-shows/)
    /// Query Parameters
    /// - ids(Required) A comma-separated list of the Spotify IDs for the shows. Maximum: 50 IDs.
    /// - market(Optional) An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn get_several_shows(
        &self,
        ids: Vec<String>,
        market: Option<Country>,
    ) -> ClientResult<SeversalSimplifiedShows> {
        let mut params = json! ({
            "ids": ids.join(","),
        });
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }
        let result = self.get("shows", None, Some(&params)).await?;
        self.convert_result(&result)
    }
    /// Get Spotify catalog information about an show’s episodes. Optional parameters can be used to limit the number of episodes returned.
    /// [Get a show's episodes](https://developer.spotify.com/documentation/web-api/reference/shows/get-shows-episodes/)
    /// Path Parameters
    /// - id: The Spotify ID for the show.
    /// Query Parameters
    /// - limit: Optional. The maximum number of episodes to return. Default: 20. Minimum: 1. Maximum: 50.
    /// - offset: Optional. The index of the first episode to return. Default: 0 (the first object). Use with limit to get the next set of episodes.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn get_shows_episodes<L: Into<Option<u32>>, O: Into<Option<u32>>>(
        &self,
        id: String,
        limit: L,
        offset: O,
        market: Option<Country>,
    ) -> ClientResult<Page<SimplifiedEpisode>> {
        let url = format!("shows/{}/episodes", id);
        let mut params = json! ({
            "limit": limit.into().unwrap_or(20),
            "offset": offset.into().unwrap_or(0),
        });
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }
        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for a single episode identified by its unique Spotify ID.
    /// [Get an Episode](https://developer.spotify.com/documentation/web-api/reference/episodes/get-an-episode/)
    /// Path Parameters
    /// - id: The Spotify ID for the episode.
    ///  Query Parameters
    /// - market: Optional. An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn get_an_episode(
        &self,
        id: String,
        market: Option<Country>,
    ) -> ClientResult<FullEpisode> {
        let url = format!("episodes/{}", id);
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "country", market);
        }

        let result = self.get(&url, None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for multiple episodes based on their Spotify IDs.
    /// [Get seversal episodes](https://developer.spotify.com/documentation/web-api/reference/episodes/get-several-episodes/)
    /// Query Parameters
    /// - ids: Required. A comma-separated list of the Spotify IDs for the episodes. Maximum: 50 IDs.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn get_several_episodes(
        &self,
        ids: Vec<String>,
        market: Option<Country>,
    ) -> ClientResult<SeveralEpisodes> {
        let mut params = json!({
            "ids": ids.join(","),
        });
        if let Some(market) = market {
            json_insert!(params, "country", market.as_str());
        }
        let result = self.get("episodes", None, Some(&params)).await?;
        self.convert_result(&result)
    }

    /// Check if one or more shows is already saved in the current Spotify user’s library.
    /// [Check users saved shows](https://developer.spotify.com/documentation/web-api/reference/library/check-users-saved-shows/)
    /// Query Parameters
    /// - ids: Required. A comma-separated list of the Spotify IDs for the shows. Maximum: 50 IDs.
    #[maybe_async]
    pub async fn check_users_saved_shows(&self, ids: Vec<String>) -> ClientResult<Vec<bool>> {
        let result = self
            .get(
                "me/shows/contains",
                None,
                Some(&json! ({
                    "ids": ids.join(","),
                })),
            )
            .await?;
        self.convert_result(&result)
    }

    /// Delete one or more shows from current Spotify user's library.
    /// Changes to a user's saved shows may not be visible in other Spotify applications immediately.
    /// [Remove user's saved shows](https://developer.spotify.com/documentation/web-api/reference/library/remove-shows-user/)
    /// Query Parameters
    /// - ids: Required. A comma-separated list of Spotify IDs for the shows to be deleted from the user’s library.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code.
    #[maybe_async]
    pub async fn remove_users_saved_shows(
        &self,
        ids: Vec<String>,
        market: Option<Country>,
    ) -> ClientResult<()> {
        let joined_ids = ids.join(",");
        let url = format!("me/shows?ids={}", joined_ids);
        let mut params = json!({});
        if let Some(market) = market {
            json_insert!(params, "country", market.as_str());
        }
        self.delete(&url, None, &params).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response_code() {
        let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN#_=_";
        let spotify = SpotifyBuilder::default().build().unwrap();
        let code = spotify.parse_response_code(url).unwrap();
        assert_eq!(code, "AQD0yXvFEOvw");
    }

    #[test]
    fn test_get_id() {
        // Assert artist
        let spotify = SpotifyBuilder::default().build().unwrap();
        let artist_id = "spotify:artist:2WX2uTcsvV5OnS0inACecP";
        let id = spotify.get_id(Type::Artist, artist_id);
        assert_eq!("2WX2uTcsvV5OnS0inACecP", &id);

        // Assert album
        let artist_id_a = "spotify/album/2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            "2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Album, artist_id_a)
        );

        // Mismatch type
        let artist_id_b = "spotify:album:2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            "spotify:album:2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Artist, artist_id_b)
        );

        // Could not split
        let artist_id_c = "spotify-album-2WX2uTcsvV5OnS0inACecP";
        assert_eq!(
            "spotify-album-2WX2uTcsvV5OnS0inACecP",
            &spotify.get_id(Type::Artist, artist_id_c)
        );

        let playlist_id = "spotify:playlist:59ZbFPES4DQwEjBpWHzrtC";
        assert_eq!(
            "59ZbFPES4DQwEjBpWHzrtC",
            &spotify.get_id(Type::Playlist, playlist_id)
        );
    }

    #[test]
    fn test_get_uri() {
        let spotify = SpotifyBuilder::default().build().unwrap();
        let track_id1 = "spotify:track:4iV5W9uYEdYUVa79Axb7Rh";
        let track_id2 = "1301WleyT98MSxVHPZCA6M";
        let uri1 = spotify.get_uri(Type::Track, track_id1);
        let uri2 = spotify.get_uri(Type::Track, track_id2);
        assert_eq!(track_id1, uri1);
        assert_eq!("spotify:track:1301WleyT98MSxVHPZCA6M", &uri2);
    }
}

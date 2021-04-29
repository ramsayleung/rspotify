//! Rspotify is a wrapper for the [Spotify Web API
//! ](https://developer.spotify.com/web-api/), inspired by [spotipy
//! ](https://github.com/plamere/spotipy). It includes support for all the
//! [authorization flows](https://developer.spotify.com/documentation/general/guides/authorization-guide/),
//! and helper methods for [all available endpoints
//! ](https://developer.spotify.com/documentation/web-api/reference/).
//!
//! ## Configuration
//!
//! ### HTTP Client
//!
//! By default, Rspotify uses the [`reqwest`] asynchronous HTTP client with its
//! default TLS, but you can customize both the HTTP client and the TLS with the
//! following features:
//!
//! - [`reqwest`](https://github.com/seanmonstar/reqwest): enabling
//!   `client-reqwest`, TLS available:
//!     + `reqwest-default-tls` (reqwest's default)
//!     + `reqwest-rustls-tls`
//!     + `reqwest-native-tls`
//!     + `reqwest-native-tls-vendored`
//! - [`ureq`](https://github.com/algesten/ureq): enabling `client-ureq`, TLS
//!   available:
//!     + `ureq-rustls-tls` (ureq's default)
//!
//! If you want to use a different client or TLS than the default ones, you'll
//! have to disable the default features and enable whichever you want. For
//! example, this would compile Rspotify with `reqwest` and the native TLS:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-reqwest", "reqwest-native-tls"]
//! }
//! ```
//!
//! [`maybe_async`] internally enables Rspotify to  use both synchronous and
//! asynchronous HTTP clients. You can also use `ureq`, a synchronous client,
//! like so:
//!
//! ```toml
//! [dependencies]
//! rspotify = {
//!     version = "...",
//!     default-features = false,
//!     features = ["client-ureq", "ureq-rustls-tls"]
//! }
//! ```
//!
//! ### Proxies
//!
//! [`reqwest`](reqwest#proxies) supports system proxies by default. It reads
//! the environment variables `HTTP_PROXY` and `HTTPS_PROXY` environmental
//! variables to set HTTP and HTTPS proxies, respectively.
//!
//! ### Environmental variables
//!
//! Rspotify supports the [`dotenv`] crate, which allows you to save credentials
//! in a `.env` file. These will then be automatically available as
//! environmental values when using methods like
//! [`CredentialsBuilder::from_env`](crate::oauth2::CredentialsBuilder::from_env):
//!
//! ```toml
//! [dependencies]
//! rspotify = { version = "...", features = ["env-file"] }
//! ```
//!
//! ### Cli utilities
//!
//! Rspotify includes basic support for Cli apps to obtain access tokens by
//! prompting the user, after enabling the `cli` feature. See the [Authorization
//! ](#authorization) section for more information.
//!
//! ## Getting Started
//!
//! ### Authorization
//!
//! All endpoints require authorization. You will need to generate a token
//! that indicates that the client has been granted permission to perform
//! requests. You will need to [register your app to get the necessary client
//! credentials](https://developer.spotify.com/dashboard/applications). Read
//! the [official guide for a detailed explanation of the different
//! authorization flows available
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/).
//!
//! The most basic authentication flow, named the [Client Credentials flow
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow),
//! consists on requesting a token to Spotify given some client credentials.
//! This can be done with [`Spotify::request_client_token`
//! ](crate::client::Spotify::request_client_token), as seen in
//! [this example
//! ](https://github.com/ramsayleung/rspotify/blob/master/examples/album.rs).
//!
//! Some of the available endpoints also require access to the user's personal
//! information, meaning that you have to follow the [Authorization Flow
//! ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
//! instead. In a nutshell, these are the steps you need to make for this:
//!
//! 0. Generate a request URL with [`Spotify::get_authorize_url`
//!    ](crate::client::Spotify::get_authorize_url).
//! 1. The user logs in with the request URL, which redirects to the redirect
//!    URI and provides a code in the parameters. This happens on your side.
//! 2. The code obtained in the previous step is parsed with
//!    [`Spotify::parse_response_code`
//!    ](crate::client::Spotify::parse_response_code).
//! 3. The code is sent to Spotify in order to obtain an access token with
//!    [`Spotify::request_user_token`
//!    ](crate::client::Spotify::request_user_token) or
//!    [`Spotify::request_user_token_without_cache`
//!    ](crate::client::Spotify::prompt_for_user_token_without_cache).
//! 4. Finally, this access token can be used internally for the requests.
//!    This access token may expire relatively soon, so it can be refreshed
//!    with the refresh token (obtained in the third step as well) using
//!    [`Spotify::refresh_user_token`
//!    ](crate::client::Spotify::refresh_user_token) or
//!    [`Spotify::refresh_user_token_without_cache`
//!    ](crate::client::Spotify::refresh_user_token_without_cache).
//!    Otherwise, a new access token may be generated from scratch by repeating
//!    these steps, but the advantage of refreshing it is that this doesn't
//!    require the user to log in, and that it's a simpler procedure.
//!
//! See the [`webapp`
//! ](https://github.com/ramsayleung/rspotify/tree/master/examples/webapp)
//! example for more details on how you can implement it for something like a
//! web server.
//!
//! If you're developing a Cli application, you might be interested in the
//! `cli` feature, which brings the [`Spotify::prompt_for_user_token`
//! ](crate::client::Spotify::prompt_for_user_token) and
//! [`Spotify::prompt_for_user_token_without_cache`
//! ](crate::client::Spotify::prompt_for_user_token_without_cache)
//! methods. These will run all the authentication steps. The user wil log in
//! by opening the request URL in its default browser, and the requests will be
//! performed automatically.
//!
//! An example of the Cli authentication:
//!
//! ![demo](https://raw.githubusercontent.com/ramsayleung/rspotify/master/doc/images/rspotify.gif)
//!
//! Note: even if your script does not have an accessible URL, you will have to
//! specify a redirect URI. It doesn't need to work or be accessible, you can
//! use `http://localhost:8888/callback` for example, which will also have the
//! code appended like so: `http://localhost/?code=...`.
//!
//! In order to help other developers to get used to `rspotify`, there are
//! public credentials available for a dummy account. You can test `rspotify`
//! with this account's `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`
//! inside the [`.env` file
//! ](https://github.com/ramsayleung/rspotify/blob/master/.env) for more
//! details.
//!
//! ### Examples
//!
//! There are some [available examples
//! ](https://github.com/ramsayleung/rspotify/tree/master/examples)
//! which can serve as a learning tool.

// Disable all modules when both client features are enabled or when none are.
// This way only the compile error below gets shown instead of a whole list of
// confusing errors..

pub mod endpoints;
pub mod client_creds;
pub mod code_auth;
pub mod code_auth_pkce;

// Subcrate re-exports
pub use rspotify_macros as macros;
pub use rspotify_model as model;
pub use rspotify_http as http;

// Top-level re-exports
pub use macros::scopes;

/// Possible errors returned from the `rspotify` client.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Raised when the authentication isn't configured properly.
    #[error("invalid client authentication: {0}")]
    InvalidAuth(String),

    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("url parse error: {0}")]
    ParseUrl(#[from] url::ParseError),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(feature = "cli")]
    #[error("cli error: {0}")]
    Cli(String),

    #[error("cache file error: {0}")]
    CacheFile(String),
}

pub type ClientResult<T> = Result<T, ClientError>;

pub const DEFAULT_API_PREFIX: &str = "https://api.spotify.com/v1/";
pub const DEFAULT_CACHE_PATH: &str = ".spotify_token_cache.json";
pub const DEFAULT_PAGINATION_CHUNKS: u32 = 50;


/// Spotify API object
#[derive(Builder, Debug, Clone)]
pub struct Spotify {
    /// Internal member to perform requests to the Spotify API.
    #[builder(setter(skip))]
    pub(in crate) http: HTTPClient,

    /// The access token information required for requests to the Spotify API.
    #[builder(setter(strip_option), default)]
    pub token: Option<Token>,

    /// The credentials needed for obtaining a new access token, for requests.
    /// without OAuth authentication.
    #[builder(setter(strip_option), default)]
    pub credentials: Option<Credentials>,

    /// The OAuth information required for obtaining a new access token, for
    /// requests with OAuth authentication. `credentials` also needs to be
    /// set up.
    #[builder(setter(strip_option), default)]
    pub oauth: Option<OAuth>,

    /// The Spotify API prefix, [`DEFAULT_API_PREFIX`] by default.
    #[builder(setter(into), default = "String::from(DEFAULT_API_PREFIX)")]
    pub prefix: String,

    /// The cache file path, in case it's used. By default it's
    /// [`DEFAULT_CACHE_PATH`]
    #[builder(default = "PathBuf::from(DEFAULT_CACHE_PATH)")]
    pub cache_path: PathBuf,

    /// The pagination chunk size used when performing automatically paginated
    /// requests, like [`Spotify::artist_albums`]. This means that a request
    /// will be performed every `pagination_chunks` items. By default this is
    /// [`DEFAULT_PAGINATION_CHUNKS`].
    ///
    /// Note that most endpoints set a maximum to the number of items per
    /// request, which most times is 50.
    #[builder(default = "DEFAULT_PAGINATION_CHUNKS")]
    pub pagination_chunks: u32,
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

    /// Converts a JSON response from Spotify into its model.
    fn convert_result<'a, T: Deserialize<'a>>(&self, input: &'a str) -> ClientResult<T> {
        serde_json::from_str::<T>(input).map_err(Into::into)
    }

    /// Append device ID to an API path.
    fn append_device_id(&self, path: &str, device_id: Option<&str>) -> String {
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

    /// Returns a single track given the track's ID, URI or URL.
    ///
    /// Parameters:
    /// - track_id - a spotify URI, URL or ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-track)
    #[maybe_async]
    pub async fn track(&self, track_id: &TrackId) -> ClientResult<FullTrack> {
        let url = format!("tracks/{}", track_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Returns a list of tracks given a list of track IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - track_ids - a list of spotify URIs, URLs or IDs
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks)
    #[maybe_async]
    pub async fn tracks<'a>(
        &self,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
        market: Option<&Market>,
    ) -> ClientResult<Vec<FullTrack>> {
        let ids = join_ids(track_ids);
        let params = build_map! {
            optional "market": market.map(|x| x.as_ref()),
        };

        let url = format!("tracks/?ids={}", ids);
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result::<FullTracks>(&result).map(|x| x.tracks)
    }

    /// Returns a single artist given the artist's ID, URI or URL.
    ///
    /// Parameters:
    /// - artist_id - an artist ID, URI or URL
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-artist)
    #[maybe_async]
    pub async fn artist(&self, artist_id: &ArtistId) -> ClientResult<FullArtist> {
        let url = format!("artists/{}", artist_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Returns a list of artists given the artist IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - artist_ids - a list of artist IDs, URIs or URLs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-artists)
    #[maybe_async]
    pub async fn artists<'a>(
        &self,
        artist_ids: impl IntoIterator<Item = &'a ArtistId>,
    ) -> ClientResult<Vec<FullArtist>> {
        let ids = join_ids(artist_ids);
        let url = format!("artists/?ids={}", ids);
        let result = self.endpoint_get(&url, &Query::new()).await?;

        self.convert_result::<FullArtists>(&result)
            .map(|x| x.artists)
    }

    /// Get Spotify catalog information about an artist's albums.
    ///
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    /// - album_type - 'album', 'single', 'appears_on', 'compilation'
    /// - market - limit the response to one particular country.
    /// - limit  - the number of albums to return
    /// - offset - the index of the first album to return
    ///
    /// See [`Spotify::artist_albums_manual`] for a manually paginated version
    /// of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-artists-albums)
    pub fn artist_albums<'a>(
        &'a self,
        artist_id: &'a ArtistId,
        album_type: Option<&'a AlbumType>,
        market: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<SimplifiedAlbum>> + 'a {
        paginate(
            move |limit, offset| {
                self.artist_albums_manual(artist_id, album_type, market, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::artist_albums`].
    #[maybe_async]
    pub async fn artist_albums_manual(
        &self,
        artist_id: &ArtistId,
        album_type: Option<&AlbumType>,
        market: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedAlbum>> {
        let limit: Option<String> = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "album_type": album_type.map(|x| x.as_ref()),
            optional "market": market.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let url = format!("artists/{}/albums", artist_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information about an artist's top 10 tracks by
    /// country.
    ///
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    /// - market - limit the response to one particular country.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-artists-top-tracks)
    #[maybe_async]
    pub async fn artist_top_tracks(
        &self,
        artist_id: &ArtistId,
        market: Market,
    ) -> ClientResult<Vec<FullTrack>> {
        let params = build_map! {
            "market": market.as_ref()
        };

        let url = format!("artists/{}/top-tracks", artist_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result::<FullTracks>(&result).map(|x| x.tracks)
    }

    /// Get Spotify catalog information about artists similar to an identified
    /// artist. Similarity is based on analysis of the Spotify community's
    /// listening history.
    ///
    /// Parameters:
    /// - artist_id - the artist ID, URI or URL
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-artists-related-artists)
    #[maybe_async]
    pub async fn artist_related_artists(
        &self,
        artist_id: &ArtistId,
    ) -> ClientResult<Vec<FullArtist>> {
        let url = format!("artists/{}/related-artists", artist_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result::<FullArtists>(&result)
            .map(|x| x.artists)
    }

    /// Returns a single album given the album's ID, URIs or URL.
    ///
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-album)
    #[maybe_async]
    pub async fn album(&self, album_id: &AlbumId) -> ClientResult<FullAlbum> {
        let url = format!("albums/{}", album_id.id());

        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Returns a list of albums given the album IDs, URIs, or URLs.
    ///
    /// Parameters:
    /// - albums_ids - a list of album IDs, URIs or URLs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-albums)
    #[maybe_async]
    pub async fn albums<'a>(
        &self,
        album_ids: impl IntoIterator<Item = &'a AlbumId>,
    ) -> ClientResult<Vec<FullAlbum>> {
        let ids = join_ids(album_ids);
        let url = format!("albums/?ids={}", ids);
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result::<FullAlbums>(&result).map(|x| x.albums)
    }

    /// Search for an Item. Get Spotify catalog information about artists,
    /// albums, tracks or playlists that match a keyword string.
    ///
    /// Parameters:
    /// - q - the search query
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    /// - type - the type of item to return. One of 'artist', 'album', 'track',
    ///  'playlist', 'show' or 'episode'
    /// - market - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - include_external: Optional.Possible values: audio. If
    ///   include_external=audio is specified the response will include any
    ///   relevant audio content that is hosted externally.  
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#category-search)
    #[maybe_async]
    pub async fn search(
        &self,
        q: &str,
        _type: SearchType,
        market: Option<&Market>,
        include_external: Option<&IncludeExternal>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<SearchResult> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            "q": q,
            "type": _type.as_ref(),
            optional "market": market.map(|x| x.as_ref()),
            optional "include_external": include_external.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("search", &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information about an album's tracks.
    ///
    /// Parameters:
    /// - album_id - the album ID, URI or URL
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
    /// See [`Spotify::album_track_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-albums-tracks)
    pub fn album_track<'a>(
        &'a self,
        album_id: &'a AlbumId,
    ) -> impl Paginator<ClientResult<SimplifiedTrack>> + 'a {
        paginate(
            move |limit, offset| self.album_track_manual(album_id, Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::album_track`].
    #[maybe_async]
    pub async fn album_track_manual(
        &self,
        album_id: &AlbumId,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedTrack>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let url = format!("albums/{}/tracks", album_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Gets basic profile information about a Spotify User.
    ///
    /// Parameters:
    /// - user - the id of the usr
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-profile)
    #[maybe_async]
    pub async fn user(&self, user_id: &UserId) -> ClientResult<PublicUser> {
        let url = format!("users/{}", user_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Get full details about Spotify playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-playlist)
    #[maybe_async]
    pub async fn playlist(
        &self,
        playlist_id: &PlaylistId,
        fields: Option<&str>,
        market: Option<&Market>,
    ) -> ClientResult<FullPlaylist> {
        let params = build_map! {
            optional "fields": fields,
            optional "market": market.map(|x| x.as_ref()),
        };

        let url = format!("playlists/{}", playlist_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get current user playlists without required getting his profile.
    ///
    /// Parameters:
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
    /// See [`Spotify::current_user_playlists_manual`] for a manually paginated
    /// version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-a-list-of-current-users-playlists)
    pub fn current_user_playlists(&self) -> impl Paginator<ClientResult<SimplifiedPlaylist>> + '_ {
        paginate(
            move |limit, offset| self.current_user_playlists_manual(Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::current_user_playlists`].
    #[maybe_async]
    pub async fn current_user_playlists_manual(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("me/playlists", &params).await?;
        self.convert_result(&result)
    }

    /// Gets playlists of a user.
    ///
    /// Parameters:
    /// - user_id - the id of the usr
    /// - limit  - the number of items to return
    /// - offset - the index of the first item to return
    ///
    /// See [`Spotify::user_playlists_manual`] for a manually paginated version
    /// of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-list-users-playlists)
    pub fn user_playlists<'a>(
        &'a self,
        user_id: &'a UserId,
    ) -> impl Paginator<ClientResult<SimplifiedPlaylist>> + 'a {
        paginate(
            move |limit, offset| self.user_playlists_manual(user_id, Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::user_playlists`].
    #[maybe_async]
    pub async fn user_playlists_manual(
        &self,
        user_id: &UserId,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let url = format!("users/{}/playlists", user_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Gets playlist of a user.
    ///
    /// Parameters:
    /// - user_id - the id of the user
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-list-users-playlists)
    #[maybe_async]
    pub async fn user_playlist(
        &self,
        user_id: &UserId,
        playlist_id: Option<&PlaylistId>,
        fields: Option<&str>,
    ) -> ClientResult<FullPlaylist> {
        let params = build_map! {
            optional "fields": fields,
        };

        let url = match playlist_id {
            Some(playlist_id) => format!("users/{}/playlists/{}", user_id.id(), playlist_id.id()),
            None => format!("users/{}/starred", user_id.id()),
        };
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get full details of the tracks of a playlist owned by a user.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - fields - which fields to return
    /// - limit - the maximum number of tracks to return
    /// - offset - the index of the first track to return
    /// - market - an ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// See [`Spotify::playlist_tracks`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-playlists-tracks)
    pub fn playlist_tracks<'a>(
        &'a self,
        playlist_id: &'a PlaylistId,
        fields: Option<&'a str>,
        market: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<PlaylistItem>> + 'a {
        paginate(
            move |limit, offset| {
                self.playlist_tracks_manual(playlist_id, fields, market, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::playlist_tracks`].
    #[maybe_async]
    pub async fn playlist_tracks_manual(
        &self,
        playlist_id: &PlaylistId,
        fields: Option<&str>,
        market: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<PlaylistItem>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "fields": fields,
            optional "market": market.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Creates a playlist for a user.
    ///
    /// Parameters:
    /// - user_id - the id of the user
    /// - name - the name of the playlist
    /// - public - is the created playlist public
    /// - description - the description of the playlist
    /// - collaborative - if the playlist will be collaborative
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-create-playlist)
    #[maybe_async]
    pub async fn user_playlist_create(
        &self,
        user_id: &UserId,
        name: &str,
        public: Option<bool>,
        collaborative: Option<bool>,
        description: Option<&str>,
    ) -> ClientResult<FullPlaylist> {
        let params = build_json! {
            "name": name,
            optional "public": public,
            optional "collaborative": collaborative,
            optional "description": description,
        };

        let url = format!("users/{}/playlists", user_id.id());
        let result = self.endpoint_post(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Changes a playlist's name and/or public/private state.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - name - optional name of the playlist
    /// - public - optional is the playlist public
    /// - collaborative - optional is the playlist collaborative
    /// - description - optional description of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-change-playlist-details)
    #[maybe_async]
    pub async fn playlist_change_detail(
        &self,
        playlist_id: &str,
        name: Option<&str>,
        public: Option<bool>,
        description: Option<&str>,
        collaborative: Option<bool>,
    ) -> ClientResult<String> {
        let params = build_json! {
            optional "name": name,
            optional "public": public,
            optional "collaborative": collaborative,
            optional "description": description,
        };

        let url = format!("playlists/{}", playlist_id);
        self.endpoint_put(&url, &params).await
    }

    /// Unfollows (deletes) a playlist for a user.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-unfollow-playlist)
    #[maybe_async]
    pub async fn playlist_unfollow(&self, playlist_id: &str) -> ClientResult<String> {
        let url = format!("playlists/{}/followers", playlist_id);
        self.endpoint_delete(&url, &json!({})).await
    }

    /// Adds tracks to a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - a list of track URIs, URLs or IDs
    /// - position - the position to add the tracks
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-add-tracks-to-playlist)
    #[maybe_async]
    pub async fn playlist_add_tracks<'a>(
        &self,
        playlist_id: &PlaylistId,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
        position: Option<i32>,
    ) -> ClientResult<PlaylistResult> {
        let uris = track_ids.into_iter().map(|id| id.uri()).collect::<Vec<_>>();
        let params = build_json! {
            "uris": uris,
            "position": position,
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.endpoint_post(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Replace all tracks in a playlist
    ///
    /// Parameters:
    /// - user - the id of the user
    /// - playlist_id - the id of the playlist
    /// - tracks - the list of track ids to add to the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-reorder-or-replace-playlists-tracks)
    #[maybe_async]
    pub async fn playlist_replace_tracks<'a>(
        &self,
        playlist_id: &PlaylistId,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
    ) -> ClientResult<()> {
        let uris = track_ids.into_iter().map(|id| id.uri()).collect::<Vec<_>>();
        let params = build_json! {
            "uris": uris
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        self.endpoint_put(&url, &params).await?;

        Ok(())
    }

    /// Reorder tracks in a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - uris - a list of Spotify URIs to replace or clear
    /// - range_start - the position of the first track to be reordered
    /// - insert_before - the position where the tracks should be inserted
    /// - range_length - optional the number of tracks to be reordered (default:
    ///   1)
    /// - snapshot_id - optional playlist's snapshot ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-reorder-or-replace-playlists-tracks)
    #[maybe_async]
    pub async fn playlist_reorder_tracks(
        &self,
        playlist_id: &PlaylistId,
        uris: Option<&[&Id<impl PlayableIdType>]>,
        range_start: Option<i32>,
        insert_before: Option<i32>,
        range_length: Option<u32>,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let uris = uris.map(|u| u.iter().map(|id| id.uri()).collect::<Vec<_>>());
        let params = build_json! {
            "playlist_id": playlist_id,
            optional "uris": uris,
            optional "range_start": range_start,
            optional "insert_before": insert_before,
            optional "range_length": range_length,
            optional "snapshot_id": snapshot_id,
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.endpoint_put(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Removes all occurrences of the given tracks from the given playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - track_ids - the list of track ids to add to the playlist
    /// - snapshot_id - optional id of the playlist snapshot
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-remove-tracks-playlist)
    #[maybe_async]
    pub async fn playlist_remove_all_occurrences_of_tracks<'a>(
        &self,
        playlist_id: &PlaylistId,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let tracks = track_ids
            .into_iter()
            .map(|id| {
                let mut map = Map::with_capacity(1);
                map.insert("uri".to_owned(), id.uri().into());
                map
            })
            .collect::<Vec<_>>();

        let params = build_json! {
            "tracks": tracks,
            optional "snapshot_id": snapshot_id,
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.endpoint_delete(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Removes specfic occurrences of the given tracks from the given playlist.
    ///
    /// Parameters:
    /// - playlist_id: the id of the playlist
    /// - tracks: an array of map containing Spotify URIs of the tracks to
    ///   remove with their current positions in the playlist. For example:
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
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-remove-tracks-playlist)
    #[maybe_async]
    pub async fn playlist_remove_specific_occurrences_of_tracks(
        &self,
        playlist_id: &PlaylistId,
        tracks: Vec<TrackPositions<'_>>,
        snapshot_id: Option<&str>,
    ) -> ClientResult<PlaylistResult> {
        let tracks = tracks
            .into_iter()
            .map(|track| {
                let mut map = Map::new();
                map.insert("uri".to_owned(), track.id.uri().into());
                map.insert("positions".to_owned(), track.positions.into());
                map
            })
            .collect::<Vec<_>>();

        let params = build_json! {
            "tracks": tracks,
            optional "snapshot_id": snapshot_id,
        };

        let url = format!("playlists/{}/tracks", playlist_id.id());
        let result = self.endpoint_delete(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Add the current authenticated user as a follower of a playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-follow-playlist)
    #[maybe_async]
    pub async fn playlist_follow(
        &self,
        playlist_id: &PlaylistId,
        public: Option<bool>,
    ) -> ClientResult<()> {
        let url = format!("playlists/{}/followers", playlist_id.id());

        let params = build_json! {
            optional "public": public,
        };

        self.endpoint_put(&url, &params).await?;

        Ok(())
    }

    /// Check to see if the given users are following the given playlist.
    ///
    /// Parameters:
    /// - playlist_id - the id of the playlist
    /// - user_ids - the ids of the users that you want to
    /// check to see if they follow the playlist. Maximum: 5 ids.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-check-if-user-follows-playlist)
    #[maybe_async]
    pub async fn playlist_check_follow(
        &self,
        playlist_id: &PlaylistId,
        user_ids: &[&UserId],
    ) -> ClientResult<Vec<bool>> {
        if user_ids.len() > 5 {
            error!("The maximum length of user ids is limited to 5 :-)");
        }
        let url = format!(
            "playlists/{}/followers/contains?ids={}",
            playlist_id.id(),
            user_ids
                .iter()
                .map(|id| id.id())
                .collect::<Vec<_>>()
                .join(","),
        );
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Get detailed profile information about the current user.
    /// An alias for the 'current_user' method.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-current-users-profile)
    #[maybe_async]
    pub async fn me(&self) -> ClientResult<PrivateUser> {
        let result = self.endpoint_get("me/", &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Get detailed profile information about the current user.
    /// An alias for the 'me' method.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-current-users-profile)
    #[maybe_async]
    pub async fn current_user(&self) -> ClientResult<PrivateUser> {
        self.me().await
    }

    /// Get information about the current users currently playing track.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-recently-played)
    #[maybe_async]
    pub async fn current_user_playing_track(
        &self,
    ) -> ClientResult<Option<CurrentlyPlayingContext>> {
        let result = self
            .get("me/player/currently-playing", None, &Query::new())
            .await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// Gets a list of the albums saved in the current authorized user's
    /// "Your Music" library
    ///
    /// Parameters:
    /// - limit - the number of albums to return
    /// - offset - the index of the first album to return
    /// - market - Provide this parameter if you want to apply Track Relinking.
    ///
    /// See [`Spotify::current_user_saved_albums`] for a manually paginated
    /// version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-saved-albums)
    pub fn current_user_saved_albums(&self) -> impl Paginator<ClientResult<SavedAlbum>> + '_ {
        paginate(
            move |limit, offset| self.current_user_saved_albums_manual(Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of
    /// [`Spotify::current_user_saved_albums`].
    #[maybe_async]
    pub async fn current_user_saved_albums_manual(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SavedAlbum>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("me/albums", &params).await?;
        self.convert_result(&result)
    }

    /// Get a list of the songs saved in the current Spotify user's "Your Music"
    /// library.
    ///
    /// Parameters:
    /// - limit - the number of tracks to return
    /// - offset - the index of the first track to return
    /// - market - Provide this parameter if you want to apply Track Relinking.
    ///
    /// See [`Spotify::current_user_saved_tracks_manual`] for a manually
    /// paginated version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-saved-tracks)
    pub fn current_user_saved_tracks(&self) -> impl Paginator<ClientResult<SavedTrack>> + '_ {
        paginate(
            move |limit, offset| self.current_user_saved_tracks_manual(Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of
    /// [`Spotify::current_user_saved_tracks`].
    #[maybe_async]
    pub async fn current_user_saved_tracks_manual(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SavedTrack>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("me/tracks", &params).await?;
        self.convert_result(&result)
    }

    /// Gets a list of the artists followed by the current authorized user.
    ///
    /// Parameters:
    /// - after - the last artist ID retrieved from the previous request
    /// - limit - the number of tracks to return
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-followed)
    #[maybe_async]
    pub async fn current_user_followed_artists(
        &self,
        after: Option<&str>,
        limit: Option<u32>,
    ) -> ClientResult<CursorBasedPage<FullArtist>> {
        let limit = limit.map(|s| s.to_string());
        let params = build_map! {
            "r#type": Type::Artist.as_ref(),
            optional "after": after,
            optional "limit": limit.as_deref(),
        };

        let result = self.endpoint_get("me/following", &params).await?;
        self.convert_result::<CursorPageFullArtists>(&result)
            .map(|x| x.artists)
    }

    /// Remove one or more tracks from the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-remove-tracks-user)
    #[maybe_async]
    pub async fn current_user_saved_tracks_delete<'a>(
        &self,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
    ) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", join_ids(track_ids));
        self.endpoint_delete(&url, &json!({})).await?;

        Ok(())
    }

    /// Check if one or more tracks is already saved in the current Spotify
    /// user’s "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-check-users-saved-tracks)
    #[maybe_async]
    pub async fn current_user_saved_tracks_contains<'a>(
        &self,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
    ) -> ClientResult<Vec<bool>> {
        let url = format!("me/tracks/contains/?ids={}", join_ids(track_ids));
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Save one or more tracks to the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - track_ids - a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-save-tracks-user)
    #[maybe_async]
    pub async fn current_user_saved_tracks_add<'a>(
        &self,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
    ) -> ClientResult<()> {
        let url = format!("me/tracks/?ids={}", join_ids(track_ids));
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Get the current user's top artists.
    ///
    /// Parameters:
    /// - limit - the number of entities to return
    /// - offset - the index of the first entity to return
    /// - time_range - Over what time frame are the affinities computed
    ///
    /// See [`Spotify::current_user_top_artists_manual`] for a manually
    /// paginated version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-top-artists-and-tracks)
    pub fn current_user_top_artists<'a>(
        &'a self,
        time_range: Option<&'a TimeRange>,
    ) -> impl Paginator<ClientResult<FullArtist>> + 'a {
        paginate(
            move |limit, offset| {
                self.current_user_top_artists_manual(time_range, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::current_user_top_artists`].
    #[maybe_async]
    pub async fn current_user_top_artists_manual(
        &self,
        time_range: Option<&TimeRange>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<FullArtist>> {
        let limit = limit.map(|s| s.to_string());
        let offset = offset.map(|s| s.to_string());
        let params = build_map! {
            optional "time_range": time_range.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get(&"me/top/artists", &params).await?;
        self.convert_result(&result)
    }

    /// Get the current user's top tracks.
    ///
    /// Parameters:
    /// - limit - the number of entities to return
    /// - offset - the index of the first entity to return
    /// - time_range - Over what time frame are the affinities computed
    ///
    /// See [`Spotify::current_user_top_tracks_manual`] for a manually paginated
    /// version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-top-artists-and-tracks)
    pub fn current_user_top_tracks<'a>(
        &'a self,
        time_range: Option<&'a TimeRange>,
    ) -> impl Paginator<ClientResult<FullTrack>> + 'a {
        paginate(
            move |limit, offset| {
                self.current_user_top_tracks_manual(time_range, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::current_user_top_tracks`].
    #[maybe_async]
    pub async fn current_user_top_tracks_manual(
        &self,
        time_range: Option<&TimeRange>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<FullTrack>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "time_range": time_range.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("me/top/tracks", &params).await?;
        self.convert_result(&result)
    }

    /// Get the current user's recently played tracks.
    ///
    /// Parameters:
    /// - limit - the number of entities to return
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-the-users-currently-playing-track)
    #[maybe_async]
    pub async fn current_user_recently_played(
        &self,
        limit: Option<u32>,
    ) -> ClientResult<CursorBasedPage<PlayHistory>> {
        let limit = limit.map(|x| x.to_string());
        let params = build_map! {
            optional "limit": limit.as_deref(),
        };

        let result = self
            .endpoint_get("me/player/recently-played", &params)
            .await?;
        self.convert_result(&result)
    }

    /// Add one or more albums to the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-save-albums-user)
    #[maybe_async]
    pub async fn current_user_saved_albums_add<'a>(
        &self,
        album_ids: impl IntoIterator<Item = &'a AlbumId>,
    ) -> ClientResult<()> {
        let url = format!("me/albums/?ids={}", join_ids(album_ids));
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Remove one or more albums from the current user's "Your Music" library.
    ///
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-remove-albums-user)
    #[maybe_async]
    pub async fn current_user_saved_albums_delete<'a>(
        &self,
        album_ids: impl IntoIterator<Item = &'a AlbumId>,
    ) -> ClientResult<()> {
        let url = format!("me/albums/?ids={}", join_ids(album_ids));
        self.endpoint_delete(&url, &json!({})).await?;

        Ok(())
    }

    /// Check if one or more albums is already saved in the current Spotify
    /// user’s "Your Music” library.
    ///
    /// Parameters:
    /// - album_ids - a list of album URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-check-users-saved-albums)
    #[maybe_async]
    pub async fn current_user_saved_albums_contains<'a>(
        &self,
        album_ids: impl IntoIterator<Item = &'a AlbumId>,
    ) -> ClientResult<Vec<bool>> {
        let url = format!("me/albums/contains/?ids={}", join_ids(album_ids));
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Follow one or more artists.
    ///
    /// Parameters:
    /// - artist_ids - a list of artist IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-follow-artists-users)
    #[maybe_async]
    pub async fn user_follow_artists<'a>(
        &self,
        artist_ids: impl IntoIterator<Item = &'a ArtistId>,
    ) -> ClientResult<()> {
        let url = format!("me/following?type=artist&ids={}", join_ids(artist_ids));
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Unfollow one or more artists.
    ///
    /// Parameters:
    /// - artist_ids - a list of artist IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-unfollow-artists-users)
    #[maybe_async]
    pub async fn user_unfollow_artists<'a>(
        &self,
        artist_ids: impl IntoIterator<Item = &'a ArtistId>,
    ) -> ClientResult<()> {
        let url = format!("me/following?type=artist&ids={}", join_ids(artist_ids));
        self.endpoint_delete(&url, &json!({})).await?;

        Ok(())
    }

    /// Check to see if the current user is following one or more artists or
    /// other Spotify users.
    ///
    /// Parameters:
    /// - artist_ids - the ids of the users that you want to
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-check-current-user-follows)
    #[maybe_async]
    pub async fn user_artist_check_follow<'a>(
        &self,
        artist_ids: impl IntoIterator<Item = &'a ArtistId>,
    ) -> ClientResult<Vec<bool>> {
        let url = format!(
            "me/following/contains?type=artist&ids={}",
            join_ids(artist_ids)
        );
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Follow one or more users.
    ///
    /// Parameters:
    /// - user_ids - a list of artist IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-follow-artists-users)
    #[maybe_async]
    pub async fn user_follow_users<'a>(
        &self,
        user_ids: impl IntoIterator<Item = &'a UserId>,
    ) -> ClientResult<()> {
        let url = format!("me/following?type=user&ids={}", join_ids(user_ids));
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Unfollow one or more users.
    ///
    /// Parameters:
    /// - user_ids - a list of artist IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-unfollow-artists-users)
    #[maybe_async]
    pub async fn user_unfollow_users<'a>(
        &self,
        user_ids: impl IntoIterator<Item = &'a UserId>,
    ) -> ClientResult<()> {
        let url = format!("me/following?type=user&ids={}", join_ids(user_ids));
        self.endpoint_delete(&url, &json!({})).await?;

        Ok(())
    }

    /// Get a list of Spotify featured playlists.
    ///
    /// Parameters:
    /// - locale - The desired language, consisting of a lowercase ISO 639
    ///   language code and an uppercase ISO 3166-1 alpha-2 country code,
    ///   joined by an underscore.
    /// - country - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - timestamp - A timestamp in ISO 8601 format: yyyy-MM-ddTHH:mm:ss. Use
    ///   this parameter to specify the user's local time to get results
    ///   tailored for that specific date and time in the day
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0
    ///   (the first object). Use with limit to get the next set of
    ///   items.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-featured-playlists)
    #[maybe_async]
    pub async fn featured_playlists(
        &self,
        locale: Option<&str>,
        country: Option<&Market>,
        timestamp: Option<chrono::DateTime<chrono::Utc>>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<FeaturedPlaylists> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let timestamp = timestamp.map(|x| x.to_rfc3339());
        let params = build_map! {
            optional "locale": locale,
            optional "country": country.map(|x| x.as_ref()),
            optional "timestamp": timestamp.as_deref(),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self
            .endpoint_get("browse/featured-playlists", &params)
            .await?;
        self.convert_result(&result)
    }

    /// Get a list of new album releases featured in Spotify.
    ///
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code or string from_token.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Spotify::new_releases_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-new-releases)
    pub fn new_releases<'a>(
        &'a self,
        country: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<SimplifiedAlbum>> + 'a {
        paginate(
            move |limit, offset| self.new_releases_manual(country, Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::new_releases`].
    #[maybe_async]
    pub async fn new_releases_manual(
        &self,
        country: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedAlbum>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "country": country.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let result = self.endpoint_get("browse/new-releases", &params).await?;
        self.convert_result::<PageSimpliedAlbums>(&result)
            .map(|x| x.albums)
    }

    /// Get a list of new album releases featured in Spotify
    ///
    /// Parameters:
    /// - country - An ISO 3166-1 alpha-2 country code or string from_token.
    /// - locale - The desired language, consisting of an ISO 639 language code
    ///   and an ISO 3166-1 alpha-2 country code, joined by an underscore.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Spotify::categories_manual`] for a manually paginated version of
    /// this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-categories)
    pub fn categories<'a>(
        &'a self,
        locale: Option<&'a str>,
        country: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<Category>> + 'a {
        paginate(
            move |limit, offset| self.categories_manual(locale, country, Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::categories`].
    #[maybe_async]
    pub async fn categories_manual(
        &self,
        locale: Option<&str>,
        country: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<Category>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "locale": locale,
            optional "country": country.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };
        let result = self.endpoint_get("browse/categories", &params).await?;
        self.convert_result::<PageCategory>(&result)
            .map(|x| x.categories)
    }

    /// Get a list of playlists in a category in Spotify
    ///
    /// Parameters:
    /// - category_id - The category id to get playlists from.
    /// - country - An ISO 3166-1 alpha-2 country code or the string from_token.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 50
    /// - offset - The index of the first item to return. Default: 0 (the first
    ///   object). Use with limit to get the next set of items.
    ///
    /// See [`Spotify::category_playlists_manual`] for a manually paginated
    /// version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-a-categories-playlists)
    pub fn category_playlists<'a>(
        &'a self,
        category_id: &'a str,
        country: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<SimplifiedPlaylist>> + 'a {
        paginate(
            move |limit, offset| {
                self.category_playlists_manual(category_id, country, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::category_playlists`].
    #[maybe_async]
    pub async fn category_playlists_manual(
        &self,
        category_id: &str,
        country: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedPlaylist>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "country": country.map(|x| x.as_ref()),
            optional "limit": limit.as_deref(),
            optional "offset": offset.as_deref(),
        };

        let url = format!("browse/categories/{}/playlists", category_id);
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result::<CategoryPlaylists>(&result)
            .map(|x| x.playlists)
    }

    /// Get Recommendations Based on Seeds
    ///
    /// Parameters:
    /// - seed_artists - a list of artist IDs, URIs or URLs
    /// - seed_tracks - a list of artist IDs, URIs or URLs
    /// - seed_genres - a list of genre names. Available genres for
    /// - market - An ISO 3166-1 alpha-2 country code or the string from_token. If provided, all
    ///   results will be playable in this country.
    /// - limit - The maximum number of items to return. Default: 20.
    ///   Minimum: 1. Maximum: 100
    /// - min/max/target_<attribute> - For the tuneable track attributes listed
    ///   in the documentation, these values provide filters and targeting on
    ///   results.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-recommendations)
    #[maybe_async]
    pub async fn recommendations(
        &self,
        payload: &Map<String, Value>,
        seed_artists: Option<Vec<&ArtistId>>,
        seed_genres: Option<Vec<&str>>,
        seed_tracks: Option<Vec<&TrackId>>,
        limit: Option<u32>,
        market: Option<&Market>,
    ) -> ClientResult<Recommendations> {
        let seed_artists = seed_artists.map(join_ids);
        let seed_genres = seed_genres.map(|x| x.join(","));
        let seed_tracks = seed_tracks.map(join_ids);
        let limit = limit.map(|x| x.to_string());
        let mut params = build_map! {
            optional "seed_artists": seed_artists.as_ref(),
            optional "seed_genres": seed_genres.as_ref(),
            optional "seed_tracks": seed_tracks.as_ref(),
            optional "market": market.map(|x| x.as_ref()),
            optional "limit": limit.as_ref(),
        };

        // TODO: this probably can be improved.
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
        let mut map_to_hold_owned_value = HashMap::new();
        let prefixes = ["min", "max", "target"];
        for attribute in attributes.iter() {
            for prefix in prefixes.iter() {
                let param = format!("{}_{}", prefix, attribute);
                if let Some(value) = payload.get(&param) {
                    // TODO: not sure if this `to_string` is what we want. It
                    // might add quotes to the strings.
                    map_to_hold_owned_value.insert(param, value.to_string());
                }
            }
        }

        for (ref key, ref value) in &map_to_hold_owned_value {
            params.insert(key, value);
        }

        let result = self.endpoint_get("recommendations", &params).await?;
        self.convert_result(&result)
    }

    /// Get audio features for a track
    ///
    /// Parameters:
    /// - track - track URI, URL or ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-audio-features)
    #[maybe_async]
    pub async fn track_features(&self, track_id: &TrackId) -> ClientResult<AudioFeatures> {
        let url = format!("audio-features/{}", track_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Get Audio Features for Several Tracks
    ///
    /// Parameters:
    /// - tracks a list of track URIs, URLs or IDs
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-audio-features)
    #[maybe_async]
    pub async fn tracks_features<'a>(
        &self,
        track_ids: impl IntoIterator<Item = &'a TrackId>,
    ) -> ClientResult<Option<Vec<AudioFeatures>>> {
        let url = format!("audio-features/?ids={}", join_ids(track_ids));

        let result = self.endpoint_get(&url, &Query::new()).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result::<Option<AudioFeaturesPayload>>(&result)
                .map(|option_payload| option_payload.map(|x| x.audio_features))
        }
    }

    /// Get Audio Analysis for a Track
    ///
    /// Parameters:
    /// - track_id - a track URI, URL or ID
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-audio-analysis)
    #[maybe_async]
    pub async fn track_analysis(&self, track_id: &TrackId) -> ClientResult<AudioAnalysis> {
        let url = format!("audio-analysis/{}", track_id.id());
        let result = self.endpoint_get(&url, &Query::new()).await?;
        self.convert_result(&result)
    }

    /// Get a User’s Available Devices
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-a-users-available-devices)
    #[maybe_async]
    pub async fn device(&self) -> ClientResult<Vec<Device>> {
        let result = self
            .endpoint_get("me/player/devices", &Query::new())
            .await?;
        self.convert_result::<DevicePayload>(&result)
            .map(|x| x.devices)
    }

    /// Get Information About The User’s Current Playback
    ///
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code or the string from_token.
    /// - additional_types: Optional. A comma-separated list of item types that
    ///   your client supports besides the default track type. Valid types are:
    ///   `track` and `episode`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-information-about-the-users-current-playback)
    #[maybe_async]
    pub async fn current_playback(
        &self,
        country: Option<&Market>,
        additional_types: Option<Vec<AdditionalType>>,
    ) -> ClientResult<Option<CurrentPlaybackContext>> {
        let additional_types =
            additional_types.map(|x| x.iter().map(|x| x.as_ref()).collect::<Vec<_>>().join(","));
        let params = build_map! {
            optional "country": country.map(|x| x.as_ref()),
            optional "additional_types": additional_types.as_ref(),
        };

        let result = self.endpoint_get("me/player", &params).await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// Get the User’s Currently Playing Track
    ///
    /// Parameters:
    /// - market: Optional. an ISO 3166-1 alpha-2 country code or the string from_token.
    /// - additional_types: Optional. A comma-separated list of item types that
    ///   your client supports besides the default track type. Valid types are:
    ///   `track` and `episode`.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-recently-played)
    #[maybe_async]
    pub async fn current_playing(
        &self,
        market: Option<&Market>,
        additional_types: Option<Vec<AdditionalType>>,
    ) -> ClientResult<Option<CurrentlyPlayingContext>> {
        let additional_types =
            additional_types.map(|x| x.iter().map(|x| x.as_ref()).collect::<Vec<_>>().join(","));
        let params = build_map! {
            optional "market": market.map(|x| x.as_ref()),
            optional "additional_types": additional_types.as_ref(),
        };

        let result = self
            .get("me/player/currently-playing", None, &params)
            .await?;
        if result.is_empty() {
            Ok(None)
        } else {
            self.convert_result(&result)
        }
    }

    /// Transfer a User’s Playback.
    ///
    /// Note: Although an array is accepted, only a single device_id is
    /// currently supported. Supplying more than one will return 400 Bad Request
    ///
    /// Parameters:
    /// - device_id - transfer playback to this device
    /// - force_play - true: after transfer, play. false:
    ///   keep current state.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-transfer-a-users-playback)
    #[maybe_async]
    pub async fn transfer_playback(
        &self,
        device_id: &str,
        force_play: Option<bool>,
    ) -> ClientResult<()> {
        let params = build_json! {
            "device_ids": [device_id],
            optional "force_play": force_play,
        };

        self.endpoint_put("me/player", &params).await?;
        Ok(())
    }

    /// Start/Resume a User’s Playback.
    ///
    /// Provide a `context_uri` to start playback or a album, artist, or
    /// playlist. Provide a `uris` list to start playback of one or more tracks.
    /// Provide `offset` as {"position": <int>} or {"uri": "<track uri>"} to
    /// start playback at a particular offset.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    /// - context_uri - spotify context uri to play
    /// - uris - spotify track uris
    /// - offset - offset into context by index or track
    /// - position_ms - Indicates from what position to start playback.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-start-a-users-playback)
    #[maybe_async]
    pub async fn start_context_playback(
        &self,
        context_uri: &Id<impl PlayContextIdType>,
        device_id: Option<&str>,
        offset: Option<Offset<impl PlayableIdType>>,
        position_ms: Option<time::Duration>,
    ) -> ClientResult<()> {
        let params = build_json! {
            "context_uri": context_uri.uri(),
            optional "offset": offset.map(|x| match x {
                Offset::Position(position) => json!({ "position": position }),
                Offset::Uri(uri) => json!({ "uri": uri.uri() }),
            }),
            optional "position_ms": position_ms,

        };

        let url = self.append_device_id("me/player/play", device_id);
        self.put(&url, None, &params).await?;

        Ok(())
    }

    #[maybe_async]
    pub async fn start_uris_playback<T: PlayableIdType>(
        &self,
        uris: &[&Id<T>],
        device_id: Option<&str>,
        offset: Option<crate::model::Offset<T>>,
        position_ms: Option<u32>,
    ) -> ClientResult<()> {
        let params = build_json! {
            "uris": uris.iter().map(|id| id.uri()).collect::<Vec<_>>(),
            optional "position_ms": position_ms,
            optional "offset": offset.map(|x| match x {
                Offset::Position(position) => json!({ "position": position }),
                Offset::Uri(uri) => json!({ "uri": uri.uri() }),
            }),
        };

        let url = self.append_device_id("me/player/play", device_id);
        self.endpoint_put(&url, &params).await?;

        Ok(())
    }

    /// Pause a User’s Playback.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-pause-a-users-playback)
    #[maybe_async]
    pub async fn pause_playback(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/pause", device_id);
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Skip User’s Playback To Next Track.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-skip-users-playback-to-next-track)
    #[maybe_async]
    pub async fn next_track(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/next", device_id);
        self.endpoint_post(&url, &json!({})).await?;

        Ok(())
    }

    /// Skip User’s Playback To Previous Track.
    ///
    /// Parameters:
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-skip-users-playback-to-previous-track)
    #[maybe_async]
    pub async fn previous_track(&self, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id("me/player/previous", device_id);
        self.endpoint_post(&url, &json!({})).await?;

        Ok(())
    }

    /// Seek To Position In Currently Playing Track.
    ///
    /// Parameters:
    /// - position_ms - position in milliseconds to seek to
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-seek-to-position-in-currently-playing-track)
    #[maybe_async]
    pub async fn seek_track(&self, position_ms: u32, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id(
            &format!("me/player/seek?position_ms={}", position_ms),
            device_id,
        );
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Set Repeat Mode On User’s Playback.
    ///
    /// Parameters:
    /// - state - `track`, `context`, or `off`
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-set-repeat-mode-on-users-playback)
    #[maybe_async]
    pub async fn repeat(&self, state: RepeatState, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id(
            &format!("me/player/repeat?state={}", state.as_ref()),
            device_id,
        );
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Set Volume For User’s Playback.
    ///
    /// Parameters:
    /// - volume_percent - volume between 0 and 100
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-set-volume-for-users-playback)
    #[maybe_async]
    pub async fn volume(&self, volume_percent: u8, device_id: Option<&str>) -> ClientResult<()> {
        if volume_percent > 100u8 {
            error!("volume must be between 0 and 100, inclusive");
        }
        let url = self.append_device_id(
            &format!("me/player/volume?volume_percent={}", volume_percent),
            device_id,
        );
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Toggle Shuffle For User’s Playback.
    ///
    /// Parameters:
    /// - state - true or false
    /// - device_id - device target for playback
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-toggle-shuffle-for-users-playback)
    #[maybe_async]
    pub async fn shuffle(&self, state: bool, device_id: Option<&str>) -> ClientResult<()> {
        let url = self.append_device_id(&format!("me/player/shuffle?state={}", state), device_id);
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Add an item to the end of the user's playback queue.
    ///
    /// Parameters:
    /// - uri - The uri of the item to add, Track or Episode
    /// - device id - The id of the device targeting
    /// - If no device ID provided the user's currently active device is
    ///   targeted
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-add-to-queue)
    #[maybe_async]
    pub async fn add_item_to_queue(
        &self,
        item: &Id<impl PlayableIdType>,
        device_id: Option<&str>,
    ) -> ClientResult<()> {
        let url = self.append_device_id(&format!("me/player/queue?uri={}", item), device_id);
        self.endpoint_post(&url, &json!({})).await?;

        Ok(())
    }

    /// Add a show or a list of shows to a user’s library.
    ///
    /// Parameters:
    /// - ids(Required) A comma-separated list of Spotify IDs for the shows to
    ///   be added to the user’s library.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-save-shows-user)
    #[maybe_async]
    pub async fn save_shows<'a>(
        &self,
        show_ids: impl IntoIterator<Item = &'a ShowId>,
    ) -> ClientResult<()> {
        let url = format!("me/shows/?ids={}", join_ids(show_ids));
        self.endpoint_put(&url, &json!({})).await?;

        Ok(())
    }

    /// Get a list of shows saved in the current Spotify user’s library.
    /// Optional parameters can be used to limit the number of shows returned.
    ///
    /// Parameters:
    /// - limit(Optional). The maximum number of shows to return. Default: 20.
    ///   Minimum: 1. Maximum: 50.
    /// - offset(Optional). The index of the first show to return. Default: 0
    ///   (the first object). Use with limit to get the next set of shows.
    ///
    /// See [`Spotify::get_saved_show_manual`] for a manually paginated version
    /// of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-users-saved-shows)
    pub fn get_saved_show(&self) -> impl Paginator<ClientResult<Show>> + '_ {
        paginate(
            move |limit, offset| self.get_saved_show_manual(Some(limit), Some(offset)),
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::get_saved_show`].
    #[maybe_async]
    pub async fn get_saved_show_manual(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<Show>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "limit": limit.as_ref(),
            optional "offset": offset.as_ref(),
        };

        let result = self.endpoint_get("me/shows", &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for a single show identified by its unique Spotify ID.
    ///
    /// Path Parameters:
    /// - id: The Spotify ID for the show.
    ///
    /// Query Parameters
    /// - market(Optional): An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-a-show)
    #[maybe_async]
    pub async fn get_a_show(&self, id: &ShowId, market: Option<&Market>) -> ClientResult<FullShow> {
        let params = build_map! {
            optional "market": market.map(|x| x.as_ref()),
        };

        let url = format!("shows/{}", id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for multiple shows based on their
    /// Spotify IDs.
    ///
    /// Query Parameters
    /// - ids(Required) A comma-separated list of the Spotify IDs for the shows. Maximum: 50 IDs.
    /// - market(Optional) An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-shows)
    #[maybe_async]
    pub async fn get_several_shows<'a>(
        &self,
        ids: impl IntoIterator<Item = &'a ShowId>,
        market: Option<&Market>,
    ) -> ClientResult<Vec<SimplifiedShow>> {
        let ids = join_ids(ids);
        let params = build_map! {
            "ids": &ids,
            optional "market": market.map(|x| x.as_ref()),
        };

        let result = self.endpoint_get("shows", &params).await?;
        self.convert_result::<SeversalSimplifiedShows>(&result)
            .map(|x| x.shows)
    }

    /// Get Spotify catalog information about an show’s episodes. Optional
    /// parameters can be used to limit the number of episodes returned.
    ///
    /// Path Parameters
    /// - id: The Spotify ID for the show.
    ///
    /// Query Parameters
    /// - limit: Optional. The maximum number of episodes to return. Default: 20. Minimum: 1. Maximum: 50.
    /// - offset: Optional. The index of the first episode to return. Default: 0 (the first object). Use with limit to get the next set of episodes.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// See [`Spotify::get_shows_episodes_manual`] for a manually paginated
    /// version of this.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-a-shows-episodes)
    pub fn get_shows_episodes<'a>(
        &'a self,
        id: &'a ShowId,
        market: Option<&'a Market>,
    ) -> impl Paginator<ClientResult<SimplifiedEpisode>> + 'a {
        paginate(
            move |limit, offset| {
                self.get_shows_episodes_manual(id, market, Some(limit), Some(offset))
            },
            self.pagination_chunks,
        )
    }

    /// The manually paginated version of [`Spotify::get_shows_episodes`].
    #[maybe_async]
    pub async fn get_shows_episodes_manual(
        &self,
        id: &ShowId,
        market: Option<&Market>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> ClientResult<Page<SimplifiedEpisode>> {
        let limit = limit.map(|x| x.to_string());
        let offset = offset.map(|x| x.to_string());
        let params = build_map! {
            optional "market": market.map(|x| x.as_ref()),
            optional "limit": limit.as_ref(),
            optional "offset": offset.as_ref(),
        };

        let url = format!("shows/{}/episodes", id.id());
        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for a single episode identified by its unique Spotify ID.
    ///
    /// Path Parameters
    /// - id: The Spotify ID for the episode.
    ///
    /// Query Parameters
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-an-episode)
    #[maybe_async]
    pub async fn get_an_episode(
        &self,
        id: &EpisodeId,
        market: Option<&Market>,
    ) -> ClientResult<FullEpisode> {
        let url = format!("episodes/{}", id.id());
        let params = build_map! {
            optional "market": market.map(|x| x.as_ref()),
        };

        let result = self.endpoint_get(&url, &params).await?;
        self.convert_result(&result)
    }

    /// Get Spotify catalog information for multiple episodes based on their Spotify IDs.
    ///
    /// Query Parameters
    /// - ids: Required. A comma-separated list of the Spotify IDs for the episodes. Maximum: 50 IDs.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-multiple-episodes)
    #[maybe_async]
    pub async fn get_several_episodes<'a>(
        &self,
        ids: impl IntoIterator<Item = &'a EpisodeId>,
        market: Option<&Market>,
    ) -> ClientResult<Vec<FullEpisode>> {
        let ids = join_ids(ids);
        let params = build_map! {
            "ids": &ids,
            optional "market": market.map(|x| x.as_ref()),
        };

        let result = self.endpoint_get("episodes", &params).await?;
        self.convert_result::<EpisodesPayload>(&result)
            .map(|x| x.episodes)
    }

    /// Check if one or more shows is already saved in the current Spotify user’s library.
    ///
    /// Query Parameters
    /// - ids: Required. A comma-separated list of the Spotify IDs for the shows. Maximum: 50 IDs.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-check-users-saved-shows)
    #[maybe_async]
    pub async fn check_users_saved_shows<'a>(
        &self,
        ids: impl IntoIterator<Item = &'a ShowId>,
    ) -> ClientResult<Vec<bool>> {
        let ids = join_ids(ids);
        let params = build_map! {
            "ids": &ids,
        };
        let result = self.endpoint_get("me/shows/contains", &params).await?;
        self.convert_result(&result)
    }

    /// Delete one or more shows from current Spotify user's library.
    /// Changes to a user's saved shows may not be visible in other Spotify applications immediately.
    ///
    /// Query Parameters
    /// - ids: Required. A comma-separated list of Spotify IDs for the shows to be deleted from the user’s library.
    /// - market: Optional. An ISO 3166-1 alpha-2 country code or the string from_token.
    ///
    /// [Reference](https://developer.spotify.com/documentation/web-api/reference/#endpoint-remove-shows-user)
    #[maybe_async]
    pub async fn remove_users_saved_shows<'a>(
        &self,
        show_ids: impl IntoIterator<Item = &'a ShowId>,
        country: Option<&Market>,
    ) -> ClientResult<()> {
        let url = format!("me/shows?ids={}", join_ids(show_ids));
        let params = build_json! {
            optional "country": country.map(|x| x.as_ref())
        };
        self.endpoint_delete(&url, &params).await?;

        Ok(())
    }
}

#[inline]
fn join_ids<'a, T: 'a + IdType>(ids: impl IntoIterator<Item = &'a Id<T>>) -> String {
    ids.into_iter().collect::<Vec<_>>().join(",")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_response_code() {
        let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN#_=_";
        let spotify = SpotifyBuilder::default().build().unwrap();
        let code = spotify.parse_response_code(url).unwrap();
        assert_eq!(code, "AQD0yXvFEOvw");
    }

    #[test]
    fn test_append_device_id_without_question_mark() {
        let path = "me/player/play";
        let device_id = Some("fdafdsadfa");
        let spotify = SpotifyBuilder::default().build().unwrap();
        let new_path = spotify.append_device_id(path, device_id);
        assert_eq!(new_path, "me/player/play?device_id=fdafdsadfa");
    }

    #[test]
    fn test_append_device_id_with_question_mark() {
        let path = "me/player/shuffle?state=true";
        let device_id = Some("fdafdsadfa");
        let spotify = SpotifyBuilder::default().build().unwrap();
        let new_path = spotify.append_device_id(path, device_id);
        assert_eq!(
            new_path,
            "me/player/shuffle?state=true&device_id=fdafdsadfa"
        );
    }
}

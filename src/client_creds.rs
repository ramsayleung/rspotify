use crate::{
    clients::BaseClient,
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, Token,
};

use maybe_async::maybe_async;

/// The [Client Credentials Flow][reference] client for the Spotify API.
///
/// This is the most basic flow. It requests a token to Spotify given some
/// client credentials, without user authorization. The only step to take is to
/// call [`Self::request_token`]. See [this example][example-main].
///
/// Note: This flow does not include authorization and therefore cannot be used
/// to access or to manage the endpoints related to user private data in
/// [`OAuthClient`](crate::clients::OAuthClient).
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/client_creds.rs
#[derive(Clone, Debug, Default)]
pub struct ClientCredsSpotify {
    pub config: Config,
    pub creds: Credentials,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
}

/// This client has access to the base methods.
impl BaseClient for ClientCredsSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn get_token_mut(&mut self) -> Option<&mut Token> {
        self.token.as_mut()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }
}

impl ClientCredsSpotify {
    /// Builds a new [`ClientCredsSpotify`] given a pair of client credentials
    /// and OAuth information.
    pub fn new(creds: Credentials) -> Self {
        ClientCredsSpotify {
            creds,
            ..Default::default()
        }
    }

    /// Build a new [`ClientCredsSpotify`] from an already generated token. Note
    /// that once the token expires this will fail to make requests,
    /// as the client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        ClientCredsSpotify {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the
    /// client.
    pub fn with_config(creds: Credentials, config: Config) -> Self {
        ClientCredsSpotify {
            config,
            creds,
            ..Default::default()
        }
    }

    /// Tries to read the cache file's token.
    ///
    /// This will return an error if the token couldn't be read (e.g. it's not
    /// available or the JSON is malformed). It may return `Ok(None)` if:
    ///
    /// * The read token is expired
    /// * Its scopes don't match with the current client
    /// * The cached token is disabled in the config
    #[maybe_async]
    pub async fn read_token_cache(&self) -> ClientResult<Option<Token>> {
        if !self.get_config().token_cached {
            return Ok(None);
        }

        let token = Token::from_cache(&self.get_config().cache_path)?;
        if token.is_expired() {
            // Invalid token, since it doesn't have at least the currently
            // required scopes or it's expired.
            Ok(None)
        } else {
            Ok(Some(token))
        }
    }

    /// Obtains the client access token for the app. The resulting token will be
    /// saved internally.
    #[maybe_async]
    pub async fn request_token(&mut self) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::GRANT_TYPE, headers::GRANT_CLIENT_CREDS);

        self.token = Some(self.fetch_access_token(&data).await?);

        self.write_token_cache()
    }
}

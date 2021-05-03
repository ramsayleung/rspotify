use crate::{
    endpoints::BaseClient,
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, Token,
};

use maybe_async::maybe_async;

/// The [Client Credentials Flow](reference) client for the Spotify API.
///
/// This is the most basic flow. It requests a token to Spotify given some
/// client credentials, without user authorization. The only step to take is to
/// call [`Self::request_token`]. See [this example](example-main).
///
/// Note: This flow does not include authorization and therefore cannot be used
/// to access or to manage the endpoints related to user private data in
/// [`OAuthClient`](crate::endpoints::OAuthClient).
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/client_creds.rs
#[derive(Clone, Debug, Default)]
pub struct ClientCredentialsSpotify {
    pub config: Config,
    pub creds: Credentials,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
}

impl BaseClient for ClientCredentialsSpotify {
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

impl ClientCredentialsSpotify {
    pub fn new(creds: Credentials) -> Self {
        ClientCredentialsSpotify {
            creds,
            ..Default::default()
        }
    }

    pub fn with_config(creds: Credentials, config: Config) -> Self {
        ClientCredentialsSpotify {
            config,
            creds,
            ..Default::default()
        }
    }

    /// Build a new `ClientCredentialsSpotify` from an already generated token.
    /// Note that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        ClientCredentialsSpotify {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Obtains the client access token for the app without saving it into the
    /// cache file. The resulting token is saved internally.
    // TODO: handle with and without cache
    #[maybe_async]
    pub async fn request_token(&mut self) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::GRANT_TYPE, headers::GRANT_CLIENT_CREDS);

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
    }
}

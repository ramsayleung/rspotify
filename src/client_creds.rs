use crate::{
    endpoints::BaseClient,
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, Token,
};

use maybe_async::maybe_async;

/// The [Client Credentials
/// Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#client-credentials-flow)
/// client for the Spotify API.
///
/// Note: This flow does not include authorization and therefore cannot be used
/// to access or to manage endpoints related to user private data.
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

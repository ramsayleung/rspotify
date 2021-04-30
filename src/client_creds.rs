use crate::{
    endpoints::BaseClient,
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, Token,
};

use maybe_async::maybe_async;

#[derive(Clone, Debug, Default)]
pub struct ClientCredentialsSpotify {
    pub config: Config,
    pub creds: Credentials,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
}

// This could even use a macro
impl BaseClient for ClientCredentialsSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.token.as_ref()
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
            creds,
            config,
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

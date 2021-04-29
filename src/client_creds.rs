use crate::{endpoints::BaseClient, Config, Credentials, http::HttpClient, Token};

#[derive(Clone, Debug, Default)]
pub struct ClientCredentialsSpotify {
    pub config: Config,
    pub creds: Credentials,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
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

    pub fn request_token(&mut self) {
        todo!()
    }
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

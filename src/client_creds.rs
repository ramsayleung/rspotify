use crate::{endpoints::BaseClient, prelude::*, Config, Credentials, HTTPClient, Token};

#[derive(Clone, Debug, Default)]
pub struct ClientCredentialsSpotify {
    pub config: Config,
    pub creds: Credentials,
    pub token: Option<Token>,
    pub(in crate) http: HTTPClient,
}

impl ClientCredentialsSpotify {
    pub fn new(creds: Credentials) -> Self {
        ClientCredentialsSpotify {
            creds,
            token: None,
            http: HTTPClient {},
            ..Default::default()
        }
    }

    pub fn with_config(creds: Credentials, config: Config) {
        ClientCredentialsSpotify {
            creds,
            config,
            token: None,
            http: HTTPClient {},
        }
    }

    pub fn request_token(&mut self) {
        self.token = Some(Token("client credentials token".to_string()))
    }
}

// This could even use a macro
impl BaseClient for ClientCredentialsSpotify {
    fn get_http(&self) -> &HTTPClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }
}

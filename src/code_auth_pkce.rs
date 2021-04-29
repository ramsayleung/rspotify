use crate::{
    endpoints::{BaseClient, OAuthClient},
    http::HttpClient,
    Config, Credentials, OAuth, Token,
};

#[derive(Clone, Debug, Default)]
pub struct CodeAuthPKCESpotify {
    creds: Credentials,
    oauth: OAuth,
    config: Config,
    tok: Option<Token>,
    http: HttpClient,
}

impl BaseClient for CodeAuthPKCESpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.tok.as_ref()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }
}

impl OAuthClient for CodeAuthPKCESpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

impl CodeAuthPKCESpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthPKCESpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        CodeAuthPKCESpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }
}

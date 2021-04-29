use crate::{prelude::*, Credentials, http::HttpClient, OAuth, Token, Config};

#[derive(Clone, Debug, Default)]
pub struct CodeAuthSpotify {
    creds: Credentials,
    oauth: OAuth,
    tok: Option<Token>,
    http: HttpClient,
}

impl CodeAuthSpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    pub fn prompt_for_user_token(&mut self) {
        todo!()
    }
}

impl BaseClient for CodeAuthSpotify {
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
        todo!()
    }
}

// This could also be a macro (less important)
impl OAuthClient for CodeAuthSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

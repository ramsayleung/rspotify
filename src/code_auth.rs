use crate::{prelude::*, Credentials, HTTPClient, OAuth, Token};

#[derive(Clone, Debug)]
pub struct CodeAuthSpotify {
    creds: Credentials,
    oauth: OAuth,
    tok: Option<Token>,
    http: HTTPClient,
}

impl CodeAuthSpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthSpotify {
            creds,
            oauth,
            tok: None,
            http: HTTPClient {},
        }
    }

    pub fn prompt_for_user_token(&mut self) {
        self.tok = Some(Token("code auth token".to_string()))
    }
}

impl BaseClient for CodeAuthSpotify {
    fn get_http(&self) -> &HTTPClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.tok.as_ref()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }
}

// This could also be a macro (less important)
impl OAuthClient for CodeAuthSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

use crate::{prelude::*, Credentials, HTTPClient, Token};

#[derive(Clone, Debug)]
pub struct ClientCredentialsSpotify {
    creds: Credentials,
    tok: Option<Token>,
    http: HTTPClient,
}

impl ClientCredentialsSpotify {
    pub fn new(creds: Credentials) -> Self {
        ClientCredentialsSpotify {
            creds,
            tok: None,
            http: HTTPClient {},
        }
    }

    pub fn request_token(&mut self) {
        self.tok = Some(Token("client credentials token".to_string()))
    }
}

// This could even use a macro
impl BaseClient for ClientCredentialsSpotify {
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

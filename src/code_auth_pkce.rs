use url::Url;

use crate::{
    auth_urls,
    endpoints::{BaseClient, OAuthClient},
    headers,
    http::HttpClient,
    ClientResult, Config, Credentials, OAuth, Token,
};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct CodeAuthPkceSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub tok: Option<Token>,
    pub(in crate) http: HttpClient,
}

impl BaseClient for CodeAuthPkceSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.tok.as_ref()
    }

    fn get_token_mut(&mut self) -> Option<&mut Token> {
        self.tok.as_mut()
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }
}

impl OAuthClient for CodeAuthPkceSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

impl CodeAuthPkceSpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthPkceSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        CodeAuthPkceSpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Gets the required URL to authorize the current client to begin the
    /// authorization flow.
    pub fn get_authorize_url(&self) -> ClientResult<String> {
        let mut payload: HashMap<&str, &str> = HashMap::new();
        let oauth = self.get_oauth();
        let scope = oauth
            .scope
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        payload.insert(headers::CLIENT_ID, &self.get_creds().id);
        payload.insert(headers::RESPONSE_TYPE, headers::RESPONSE_CODE);
        payload.insert(headers::REDIRECT_URI, &oauth.redirect_uri);
        payload.insert(headers::SCOPE, &scope);
        payload.insert(headers::STATE, &oauth.state);
        // TODO
        // payload.insert(headers::CODE_CHALLENGE, todo!());
        // payload.insert(headers::CODE_CHALLENGE_METHOD, "S256");

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into_string())
    }
}
use url::Url;

use crate::{ClientResult, Config, Credentials, OAuth, Token, auth_urls, endpoints::{BaseClient, OAuthClient}, headers, http::HttpClient};

use std::collections::HashMap;

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

    /// Gets the required URL to authorize the current client to start the
    /// [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
        let mut payload: HashMap<&str, &str> = HashMap::new();
        let oauth = self.get_oauth();
        let scope = oauth
            .scope
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        payload.insert(headers::CLIENT_ID, &self.get_creds().id);
        payload.insert(headers::RESPONSE_TYPE, headers::RESPONSE_CODE);
        payload.insert(headers::REDIRECT_URI, &oauth.redirect_uri);
        payload.insert(headers::SCOPE, &scope);
        payload.insert(headers::STATE, &oauth.state);
        payload.insert(headers::CODE_CHALLENGE, todo!());
        payload.insert(headers::CODE_CHALLENGE_METHOD, "S256");

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into_string())
    }
}

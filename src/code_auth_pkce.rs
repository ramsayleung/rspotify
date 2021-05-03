use url::Url;

use crate::{
    auth_urls,
    endpoints::{BaseClient, OAuthClient},
    headers,
    http::HttpClient,
    ClientResult, Config, Credentials, OAuth, Token,
};

use std::collections::HashMap;

/// The [Authorization Code Flow with Proof Key for Code Exchange
/// (PKCE)](reference) client for the Spotify API.
///
/// This flow is very similar to the regular Authorization Code Flow, so please
/// read [`CodeAuthSpotify`](crate::CodeAuthSpotify) for more information about
/// it. The main difference in this case is that you can avoid storing your
/// client secret by generating a *code verifier* and a *code challenge*.
///
/// There's an [example](example-main) available to learn how to use this
/// client.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow-with-proof-key-for-code-exchange-pkce
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/code_auth.rs
#[derive(Clone, Debug, Default)]
pub struct CodeAuthPkceSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
}

impl BaseClient for CodeAuthPkceSpotify {
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

    /// Build a new `CodeAuthPkceSpotify` from an already generated token. Note
    /// that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        CodeAuthPkceSpotify {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Gets the required URL to authorize the current client to begin the
    /// authorization flow.
    // TODO
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
        // payload.insert(headers::CODE_CHALLENGE, todo!());
        // payload.insert(headers::CODE_CHALLENGE_METHOD, "S256");

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into_string())
    }
}

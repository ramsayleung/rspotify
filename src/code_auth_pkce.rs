use crate::{
    auth_urls,
    endpoints::{BaseClient, OAuthClient},
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, OAuth, Token,
};

use std::collections::HashMap;

use maybe_async::maybe_async;
use url::Url;

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

/// This client has access to the base methods.
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

/// This client includes user authorization, so it has access to the user
/// private endpoints in [`OAuthClient`].
impl OAuthClient for CodeAuthPkceSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

/// Some client-specific implementations specific to the authorization flow.
impl CodeAuthPkceSpotify {
    /// Builds a new [`CodeAuthPkceSpotify`] given a pair of client credentials
    /// and OAuth information.
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthPkceSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`CodeAuthPkceSpotify`] from an already generated token.
    /// Note that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        CodeAuthPkceSpotify {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the
    /// client.
    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        CodeAuthPkceSpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    pub fn get_authorize_url(&self) -> ClientResult<String> {
        // TODO
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

    /// Obtains a user access token given a code, as part of the OAuth
    /// authentication. The access token will be saved internally.
    #[maybe_async]
    pub async fn request_token(&mut self, code: &str) -> ClientResult<()> {
        // TODO
        let mut data = Form::new();
        let oauth = self.get_oauth();
        let scopes = oauth
            .scope
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        data.insert(headers::GRANT_TYPE, headers::GRANT_AUTH_CODE);
        data.insert(headers::REDIRECT_URI, oauth.redirect_uri.as_ref());
        data.insert(headers::CODE, code);
        data.insert(headers::SCOPE, scopes.as_ref());
        data.insert(headers::STATE, oauth.state.as_ref());

        let token = self.fetch_access_token(&data).await?;
        self.token = Some(token);

        self.write_token_cache()
    }

    /// Refreshes the current access token given a refresh token.
    ///
    /// The obtained token will be saved internally.
    #[maybe_async]
    pub async fn refresh_token(&mut self, refresh_token: &str) -> ClientResult<()> {
        // TODO
        let mut data = Form::new();
        data.insert(headers::REFRESH_TOKEN, refresh_token);
        data.insert(headers::GRANT_TYPE, headers::GRANT_REFRESH_TOKEN);

        let mut token = self.fetch_access_token(&data).await?;
        token.refresh_token = Some(refresh_token.to_string());
        self.token = Some(token);

        self.write_token_cache()
    }

    /// Opens up the authorization URL in the user's browser so that it can
    /// authenticate. It also reads from the standard input the redirect URI
    /// in order to obtain the access token information. The resulting access
    /// token will be saved internally once the operation is successful.
    ///
    /// The authorizaton URL can be obtained with [`Self::get_authorize_url`].
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_token(&mut self, url: &str) -> ClientResult<()> {
        // TODO: this should go in OAuthClient
        match self.read_token_cache().await {
            // TODO: shouldn't this also refresh the obtained token?
            Some(new_token) => self.token.replace(new_token),
            // Otherwise following the usual procedure to get the token.
            None => {
                let code = self.get_code_from_user(url)?;
                // Will write to the cache file if successful
                self.request_token(&code).await?;
            }
        }

        self.write_token_cache()
    }
}

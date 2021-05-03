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

/// The [Authorization Code
/// Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow)
/// client for the Spotify API.
///
/// This includes user authorization, and thus has access to endpoints related
/// to user private data, unlike the [Client Credentials
/// Flow](crate::ClientCredentialsSpotify) client.
#[derive(Clone, Debug, Default)]
pub struct CodeAuthSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Option<Token>,
    pub(in crate) http: HttpClient,
}

impl BaseClient for CodeAuthSpotify {
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

// This could also be a macro (less important)
impl OAuthClient for CodeAuthSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }
}

impl CodeAuthSpotify {
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        CodeAuthSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        CodeAuthSpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Gets the required URL to authorize the current client to begin the
    /// authorization flow.
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
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

        if show_dialog {
            payload.insert(headers::SHOW_DIALOG, "true");
        }

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into_string())
    }

    /// Obtains the user access token for the app with the given code without
    /// saving it into the cache file, as part of the OAuth authentication.
    /// The access token will be saved inside the Spotify instance.
    // TODO: implement with and without cache.
    #[maybe_async]
    pub async fn request_token(&mut self, code: &str) -> ClientResult<()> {
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

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
    }

    /// Refreshes the access token with the refresh token provided by the
    /// without saving it into the cache file.
    ///
    /// The obtained token will be saved internally.
    // TODO: implement with and without cache
    #[maybe_async]
    pub async fn refresh_token(&mut self, refresh_token: &str) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::REFRESH_TOKEN, refresh_token);
        data.insert(headers::GRANT_TYPE, headers::GRANT_REFRESH_TOKEN);

        let mut tok = self.fetch_access_token(&data).await?;
        tok.refresh_token = Some(refresh_token.to_string());
        self.token = Some(tok);

        Ok(())
    }

    /// The same as the `prompt_for_user_token_without_cache` method, but it
    /// will try to use the user token into the cache file, and save it in
    /// case it didn't exist/was invalid.
    ///
    /// Note: this method requires the `cli` feature.
    // TODO: handle with and without cache
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_token(&mut self, url: &str) -> ClientResult<()> {
        match self.read_oauth_token_cache().await {
            // TODO: shouldn't this also refresh the obtained token?
            Some(mut new_token) => {
                let mut cur_token = self.get_token_mut();
                cur_token.replace(&mut new_token);
            }
            // Otherwise following the usual procedure to get the token.
            None => {
                let code = self.get_code_from_user(url)?;
                // Will write to the cache file if successful
                self.request_token(&code).await?;
            }
        }

        Ok(())
    }
}

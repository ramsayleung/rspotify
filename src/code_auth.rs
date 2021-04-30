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

#[derive(Clone, Debug, Default)]
pub struct CodeAuthSpotify {
    creds: Credentials,
    oauth: OAuth,
    config: Config,
    token: Option<Token>,
    http: HttpClient,
}

impl BaseClient for CodeAuthSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Option<&Token> {
        self.token.as_ref()
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

    /// Tries to open the authorization URL in the user's browser, and returns
    /// the obtained code.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    fn get_code_from_user(&self) -> ClientResult<String> {
        use crate::ClientError;

        let url = self.get_authorize_url(false)?;

        match webbrowser::open(&url) {
            Ok(_) => println!("Opened {} in your browser.", url),
            Err(why) => eprintln!(
                "Error when trying to open an URL in your browser: {:?}. \
                 Please navigate here manually: {}",
                why, url
            ),
        }

        println!("Please enter the URL you were redirected to: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let code = self
            .parse_response_code(&input)
            .ok_or_else(|| ClientError::Cli("unable to parse the response code".to_string()))?;

        Ok(code)
    }
}

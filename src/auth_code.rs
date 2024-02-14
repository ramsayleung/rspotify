use crate::{
    auth_urls,
    clients::{BaseClient, OAuthClient},
    http::{Form, HttpClient},
    join_scopes, params,
    sync::Mutex,
    ClientError, ClientResult, Config, Credentials, OAuth, Token,
};

use std::collections::HashMap;
use std::sync::Arc;

use maybe_async::maybe_async;
use url::Url;

/// The [Authorization Code Flow][reference] client for the Spotify API.
///
/// This includes user authorization, and thus has access to endpoints related
/// to user private data, unlike the [Client Credentials
/// Flow](crate::ClientCredsSpotify) client. See [`BaseClient`] and
/// [`OAuthClient`] for the available endpoints.
///
/// If you're developing a CLI application, you might be interested in the `cli`
/// feature. This brings the `prompt_for_token` method to automatically follow
/// the flow steps via user interaction.
///
/// Otherwise, these are the steps to be followed to authenticate your app:
///
/// 0. Generate a request URL with [`Self::get_authorize_url`].
/// 1. The user logs in with the request URL. They will be redirected to the
///    given redirect URI, including a code in the URL parameters. This happens
///    on your side.
/// 2. The code obtained in the previous step is parsed with
///    [`Self::parse_response_code`].
/// 3. The code is sent to Spotify in order to obtain an access token with
///    [`Self::request_token`].
/// 4. Finally, this access token can be used internally for the requests.
///    It may expire relatively soon, so it can be refreshed with the refresh
///    token (obtained in the previous step as well) using
///    [`Self::refresh_token`]. Otherwise, a new access token may be generated
///    from scratch by repeating these steps, but the advantage of refreshing it
///    is that this doesn't require the user to log in, and that it's a simpler
///    procedure.
///
///    See [this related example][example-refresh-token] to learn more about
///    refreshing tokens.
///
/// There's a [webapp example][example-webapp] for more details on how you can
/// implement it for something like a web server, or [this one][example-main]
/// for a CLI use case.
///
/// An example of the CLI authentication:
///
/// ![demo](https://raw.githubusercontent.com/ramsayleung/rspotify/master/doc/images/rspotify.gif)
///
/// Note: even if your script does not have an accessible URL, you will have to
/// specify a redirect URI. It doesn't need to work, you can use
/// `http://localhost:8888/callback` for example, which will also have the code
/// appended like so: `http://localhost/?code=...`.
///
/// [reference]: https://developer.spotify.com/documentation/web-api/tutorials/code-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code.rs
/// [example-webapp]: https://github.com/ramsayleung/rspotify/tree/master/examples/webapp
/// [example-refresh-token]: https://github.com/ramsayleung/rspotify/blob/master/examples/with_refresh_token.rs
#[derive(Clone, Debug, Default)]
pub struct AuthCodeSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<Mutex<Option<Token>>>,
    pub(crate) http: HttpClient,
}

/// This client has access to the base methods.
#[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
impl BaseClient for AuthCodeSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_token(&self) -> Arc<Mutex<Option<Token>>> {
        Arc::clone(&self.token)
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    /// Refetch the current access token given a refresh token. May return
    /// `None` if there's no access/refresh token.
    async fn refetch_token(&self) -> ClientResult<Option<Token>> {
        match self.token.lock().await.unwrap().as_ref() {
            Some(Token {
                refresh_token: Some(refresh_token),
                ..
            }) => {
                let mut data = Form::new();
                data.insert(params::REFRESH_TOKEN, refresh_token);
                data.insert(params::GRANT_TYPE, params::REFRESH_TOKEN);

                let headers = self
                    .creds
                    .auth_headers()
                    .expect("No client secret set in the credentials.");
                let mut token = self.fetch_access_token(&data, Some(&headers)).await?;

                token.refresh_token = Some(refresh_token.to_string());

                if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
                    callback_fn.0(token.clone())?;
                }

                Ok(Some(token))
            }
            _ => {
                log::warn!("Can not refresh token! Token missing!");
                Err(ClientError::InvalidToken)
            }
        }
    }
}

/// This client includes user authorization, so it has access to the user
/// private endpoints in [`OAuthClient`].
#[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
impl OAuthClient for AuthCodeSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }

    /// Obtains a user access token given a code, as part of the OAuth
    /// authentication. The access token will be saved internally.
    async fn request_token(&self, code: &str) -> ClientResult<()> {
        log::info!("Requesting Auth Code token");

        let scopes = join_scopes(&self.oauth.scopes);

        let mut data = Form::new();
        data.insert(params::GRANT_TYPE, params::GRANT_TYPE_AUTH_CODE);
        data.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        data.insert(params::CODE, code);
        data.insert(params::SCOPE, &scopes);
        data.insert(params::STATE, &self.oauth.state);

        let headers = self
            .creds
            .auth_headers()
            .expect("No client secret set in the credentials.");

        let token = self.fetch_access_token(&data, Some(&headers)).await?;

        if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
            callback_fn.0(token.clone())?;
        }

        *self.token.lock().await.unwrap() = Some(token);

        self.write_token_cache().await
    }
}

impl AuthCodeSpotify {
    /// Builds a new [`AuthCodeSpotify`] given a pair of client credentials and
    /// OAuth information.
    #[must_use]
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        Self {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodeSpotify`] from an already generated token. Note
    /// that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    #[must_use]
    pub fn from_token(token: Token) -> Self {
        Self {
            token: Arc::new(Mutex::new(Some(token))),
            ..Default::default()
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the
    /// client.
    #[must_use]
    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        Self {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodeSpotify`] from an already generated token and
    /// config. Use this to be able to refresh a token.
    #[must_use]
    pub fn from_token_with_config(
        token: Token,
        creds: Credentials,
        oauth: OAuth,
        config: Config,
    ) -> Self {
        Self {
            token: Arc::new(Mutex::new(Some(token))),
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
        log::info!("Building auth URL");

        let scopes = join_scopes(&self.oauth.scopes);

        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert(params::CLIENT_ID, &self.creds.id);
        payload.insert(params::RESPONSE_TYPE, params::RESPONSE_TYPE_CODE);
        payload.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        payload.insert(params::SCOPE, &scopes);
        payload.insert(params::STATE, &self.oauth.state);

        if show_dialog {
            payload.insert(params::SHOW_DIALOG, "true");
        }

        let request_url = self.auth_url(auth_urls::AUTHORIZE);
        let parsed = Url::parse_with_params(&request_url, payload)?;
        Ok(parsed.into())
    }
}

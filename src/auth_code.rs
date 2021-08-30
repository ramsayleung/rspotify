use crate::{
    auth_urls,
    clients::{BaseClient, OAuthClient},
    headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, OAuth, Token,
};

use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "__sync")]
use std::sync::Mutex;

#[cfg(feature = "__async")]
use futures::lock::Mutex;
use maybe_async::maybe_async;
use url::Url;

/// The [Authorization Code Flow](reference) client for the Spotify API.
///
/// This includes user authorization, and thus has access to endpoints related
/// to user private data, unlike the [Client Credentials
/// Flow](crate::ClientCredsSpotify) client. See [`BaseClient`] and
/// [`OAuthClient`] for the available endpoints.
///
/// If you're developing a CLI application, you might be interested in the `cli`
/// feature. This brings the [`Self::prompt_for_token`] utility to automatically
/// follow the flow steps via user interaction.
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
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code.rs
/// [example-webapp]: https://github.com/ramsayleung/rspotify/tree/master/examples/webapp
/// [example-refresh-token]: https://github.com/ramsayleung/rspotify/blob/master/examples/with_refresh_token.rs
#[derive(Debug, Default, Clone)]
pub struct AuthCodeSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<Mutex<Option<Token>>>,
    pub(in crate) http: HttpClient,
}
/// This client has access to the base methods.
#[maybe_async]
impl BaseClient for AuthCodeSpotify {
    fn get_http(&self) -> &HttpClient {
        &self.http
    }

    fn get_creds(&self) -> &Credentials {
        &self.creds
    }

    fn get_config(&self) -> &Config {
        &self.config
    }

    async fn get_token(&self) -> Arc<Mutex<Option<Token>>> {
        self.auto_reauth()
            .await
            .expect("Failed to re-authenticate automatically, please authenticate");
        Arc::clone(&self.token)
    }

    /// Refetch the current access token given a refresh token. May return
    /// `None` if there's no access/refresh token.
    async fn refetch_token(&self) -> ClientResult<Option<Token>> {
        // NOTE: this can't use `get_token` because `get_token` itself might
        // call this function when automatic reauthentication is enabled.

        // The sync and async versions of Mutex have different function signatures
        let tok = self.get_token().await;
        let locked_token = tok.lock().await;
        let mut tmp_locked_lock = Option::None;
        #[cfg(feature = "__async")]
        {
            tmp_locked_lock = locked_token.as_ref();
        }

        #[cfg(feature = "__sync")]
        {
            tmp_locked_lock = locked_token.unwrap().as_ref();
        }

        match tmp_locked_lock {
            Some(Token {
                refresh_token: Some(refresh_token),
                ..
            }) => {
                let mut data = Form::new();
                data.insert(headers::REFRESH_TOKEN, &refresh_token);
                data.insert(headers::GRANT_TYPE, headers::GRANT_REFRESH_TOKEN);

                let mut token = self.fetch_access_token(&data).await?;
                token.refresh_token.replace(refresh_token.to_string());
                Ok(Some(token))
            }
            _ => Ok(None),
        }
    }
}

/// This client includes user authorization, so it has access to the user
/// private endpoints in [`OAuthClient`].
#[maybe_async]
impl OAuthClient for AuthCodeSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }

    /// Obtains a user access token given a code, as part of the OAuth
    /// authentication. The access token will be saved internally.
    async fn request_token(&self, code: &str) -> ClientResult<()> {
        let mut data = Form::new();
        let oauth = self.get_oauth();
        let scopes = oauth
            .scopes
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

        #[cfg(feature = "__async")]
        {
            *self.token.lock().await = Some(token);
        }
        #[cfg(feature = "__sync")]
        {
            *self.token.lock().unwrap() = Some(token);
        }

        self.write_token_cache().await
    }
}

impl AuthCodeSpotify {
    /// Builds a new [`AuthCodeSpotify`] given a pair of client credentials and
    /// OAuth information.
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        AuthCodeSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodeSpotify`] from an already generated token. Note
    /// that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        AuthCodeSpotify {
            token: Arc::new(Mutex::new(Some(token))),
            ..Default::default()
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the
    /// client.
    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        AuthCodeSpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
        let mut payload: HashMap<&str, &str> = HashMap::new();
        let oauth = self.get_oauth();
        let scopes = oauth
            .scopes
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .join(" ");
        payload.insert(headers::CLIENT_ID, &self.get_creds().id);
        payload.insert(headers::RESPONSE_TYPE, headers::RESPONSE_CODE);
        payload.insert(headers::REDIRECT_URI, &oauth.redirect_uri);
        payload.insert(headers::SCOPE, &scopes);
        payload.insert(headers::STATE, &oauth.state);

        if show_dialog {
            payload.insert(headers::SHOW_DIALOG, "true");
        }

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into())
    }
}

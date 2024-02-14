use crate::{
    alphabets, auth_urls,
    clients::{BaseClient, OAuthClient},
    generate_random_string,
    http::{Form, HttpClient},
    join_scopes, params,
    sync::Mutex,
    ClientResult, Config, Credentials, OAuth, Token,
};

use base64::{engine::general_purpose, Engine as _};

use std::collections::HashMap;
use std::sync::Arc;

use maybe_async::maybe_async;
use sha2::{Digest, Sha256};
use url::Url;

/// The [Authorization Code Flow with Proof Key for Code Exchange
/// (PKCE)][reference] client for the Spotify API.
///
/// This flow is very similar to the regular Authorization Code Flow, so please
/// read [`AuthCodeSpotify`](crate::AuthCodeSpotify) for more information about
/// it. The main difference in this case is that you can avoid storing your
/// client secret by generating a *code verifier* and a *code challenge*.
/// However, note that the refresh token obtained with PKCE will only work to
/// request the next one, after which it'll become invalid.
///
/// There's an [example][example-main] available to learn how to use this
/// client.
///
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code_pkce.rs
#[derive(Clone, Debug, Default)]
pub struct AuthCodePkceSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Arc<Mutex<Option<Token>>>,
    /// The code verifier for the authentication process
    pub verifier: Option<String>,
    pub(crate) http: HttpClient,
}

/// This client has access to the base methods.
#[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
impl BaseClient for AuthCodePkceSpotify {
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

    async fn refetch_token(&self) -> ClientResult<Option<Token>> {
        match self.token.lock().await.unwrap().as_ref() {
            Some(Token {
                refresh_token: Some(refresh_token),
                ..
            }) => {
                let mut data = Form::new();
                data.insert(params::GRANT_TYPE, params::GRANT_TYPE_REFRESH_TOKEN);
                data.insert(params::REFRESH_TOKEN, refresh_token);
                data.insert(params::CLIENT_ID, &self.creds.id);

                let token = self.fetch_access_token(&data, None).await?;

                if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
                    callback_fn.0(token.clone())?;
                }

                Ok(Some(token))
            }
            _ => Ok(None),
        }
    }
}

/// This client includes user authorization, so it has access to the user
/// private endpoints in [`OAuthClient`].
#[cfg_attr(target_arch = "wasm32", maybe_async(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async)]
impl OAuthClient for AuthCodePkceSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }

    /// Note that the code verifier must be set at this point, either manually
    /// or with [`Self::get_authorize_url`]. Otherwise, this function will
    /// panic.
    async fn request_token(&self, code: &str) -> ClientResult<()> {
        log::info!("Requesting PKCE Auth Code token");

        let verifier = self.verifier.as_ref().expect(
            "Unknown code verifier. Try calling \
            `AuthCodePkceSpotify::get_authorize_url` first or setting it \
            yourself.",
        );

        let mut data = Form::new();
        data.insert(params::CLIENT_ID, &self.creds.id);
        data.insert(params::GRANT_TYPE, params::GRANT_TYPE_AUTH_CODE);
        data.insert(params::CODE, code);
        data.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        data.insert(params::CODE_VERIFIER, verifier);

        let token = self.fetch_access_token(&data, None).await?;

        if let Some(callback_fn) = &*self.get_config().token_callback_fn.clone() {
            callback_fn.0(token.clone())?;
        }

        *self.token.lock().await.unwrap() = Some(token);

        self.write_token_cache().await
    }
}

impl AuthCodePkceSpotify {
    /// Builds a new [`AuthCodePkceSpotify`] given a pair of client credentials
    /// and OAuth information.
    #[must_use]
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        Self {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodePkceSpotify`] from an already generated token.
    /// Note that once the token expires this will fail to make requests, as the
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

    /// Generate the verifier code and the challenge code.
    fn generate_codes(verifier_bytes: usize) -> (String, String) {
        log::info!("Generating PKCE codes");

        debug_assert!(verifier_bytes >= 43);
        debug_assert!(verifier_bytes <= 128);
        // The code verifier is just the randomly generated string.
        let verifier = generate_random_string(verifier_bytes, alphabets::PKCE_CODE_VERIFIER);
        // The code challenge is the code verifier hashed with SHA256 and then
        // encoded with base64url.
        //
        // NOTE: base64url != base64; it uses a different set of characters. See
        // https://datatracker.ietf.org/doc/html/rfc4648#section-5 for more
        // information.
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge = hasher.finalize();

        let challenge = general_purpose::URL_SAFE_NO_PAD.encode(challenge);

        (verifier, challenge)
    }

    /// Returns the URL needed to authorize the current client as the first step
    /// in the authorization flow.
    ///
    /// The parameter `verifier_bytes` is the length of the randomly generated
    /// code verifier. Note that it must be between 43 and 128. If `None` is
    /// given, a length of 43 will be used by default. See [the official
    /// docs][reference] or [PKCE's RFC][rfce] for more information about the
    /// code verifier.
    ///
    /// [reference]: https://developer.spotify.com/documentation/general/guides/authorization/code-flow
    /// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
    pub fn get_authorize_url(&mut self, verifier_bytes: Option<usize>) -> ClientResult<String> {
        log::info!("Building auth URL");

        let scopes = join_scopes(&self.oauth.scopes);
        let verifier_bytes = verifier_bytes.unwrap_or(43);
        let (verifier, challenge) = Self::generate_codes(verifier_bytes);
        // The verifier will be needed later when requesting the token
        self.verifier = Some(verifier);

        let mut payload: HashMap<&str, &str> = HashMap::new();
        payload.insert(params::CLIENT_ID, &self.creds.id);
        payload.insert(params::RESPONSE_TYPE, params::RESPONSE_TYPE_CODE);
        payload.insert(params::REDIRECT_URI, &self.oauth.redirect_uri);
        payload.insert(
            params::CODE_CHALLENGE_METHOD,
            params::CODE_CHALLENGE_METHOD_S256,
        );
        payload.insert(params::CODE_CHALLENGE, &challenge);
        payload.insert(params::STATE, &self.oauth.state);
        payload.insert(params::SCOPE, &scopes);

        let request_url = self.auth_url(auth_urls::AUTHORIZE);
        let parsed = Url::parse_with_params(&request_url, payload)?;
        Ok(parsed.into())
    }
}

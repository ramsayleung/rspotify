use crate::{
    alphabets, auth_urls,
    clients::{BaseClient, OAuthClient},
    generate_random_string, headers,
    http::{Form, HttpClient},
    ClientResult, Config, Credentials, OAuth, Token,
};

use std::collections::HashMap;

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
/// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow-with-proof-key-for-code-exchange-pkce
/// [example-main]: https://github.com/ramsayleung/rspotify/blob/master/examples/auth_code_pkce.rs
#[derive(Clone, Debug, Default)]
pub struct AuthCodePkceSpotify {
    pub creds: Credentials,
    pub oauth: OAuth,
    pub config: Config,
    pub token: Option<Token>,
    /// The code verifier for the authentication process
    pub verifier: Option<String>,
    pub(in crate) http: HttpClient,
}

/// This client has access to the base methods.
impl BaseClient for AuthCodePkceSpotify {
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
#[maybe_async]
impl OAuthClient for AuthCodePkceSpotify {
    fn get_oauth(&self) -> &OAuth {
        &self.oauth
    }

    async fn request_token(&mut self, code: &str) -> ClientResult<()> {
        // TODO
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
        data.insert(headers::CODE_VERIFIER, todo!());

        let token = self.fetch_access_token(&data).await?;
        self.token = Some(token);

        self.write_token_cache()
    }

    async fn refresh_token(&mut self, refresh_token: &str) -> ClientResult<()> {
        // TODO
        let mut data = Form::new();
        data.insert(headers::REFRESH_TOKEN, refresh_token);
        data.insert(headers::GRANT_TYPE, headers::GRANT_REFRESH_TOKEN);
        data.insert(headers::CLIENT_ID, headers::CLIENT_ID);

        let mut token = self.fetch_access_token(&data).await?;
        token.refresh_token = Some(refresh_token.to_string());
        self.token = Some(token);

        self.write_token_cache()
    }
}

impl AuthCodePkceSpotify {
    /// Builds a new [`AuthCodePkceSpotify`] given a pair of client credentials
    /// and OAuth information.
    pub fn new(creds: Credentials, oauth: OAuth) -> Self {
        AuthCodePkceSpotify {
            creds,
            oauth,
            ..Default::default()
        }
    }

    /// Build a new [`AuthCodePkceSpotify`] from an already generated token.
    /// Note that once the token expires this will fail to make requests, as the
    /// client credentials aren't known.
    pub fn from_token(token: Token) -> Self {
        AuthCodePkceSpotify {
            token: Some(token),
            ..Default::default()
        }
    }

    /// Same as [`Self::new`] but with an extra parameter to configure the
    /// client.
    pub fn with_config(creds: Credentials, oauth: OAuth, config: Config) -> Self {
        AuthCodePkceSpotify {
            creds,
            oauth,
            config,
            ..Default::default()
        }
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
    /// [reference]: https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow-with-proof-key-for-code-exchange-pkce
    /// [rfce]: https://datatracker.ietf.org/doc/html/rfc7636#section-4.1
    pub fn get_authorize_url(&mut self, verifier_bytes: Option<usize>) -> ClientResult<String> {
        let verifier_bytes = verifier_bytes.unwrap_or(43);
        debug_assert!(43 <= verifier_bytes);
        debug_assert!(verifier_bytes <= 128);
        // The code verifier is just the randomly generated string.
        let verifier = generate_random_string(verifier_bytes, alphabets::PKCE_CODE_VERIFIER);
        // The code challenge is the code verifier hashed with SHA256 and then
        // encoded with base64.
        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let challenge = hasher.finalize();
        let challenge = base64::encode(challenge);
        self.verifier = Some(verifier);

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
        payload.insert(headers::CODE_CHALLENGE, &challenge);
        payload.insert(
            headers::CODE_CHALLENGE_METHOD,
            headers::CODE_CHALLENGE_METHOD_S256,
        );

        let parsed = Url::parse_with_params(auth_urls::AUTHORIZE, payload)?;
        Ok(parsed.into())
    }
}

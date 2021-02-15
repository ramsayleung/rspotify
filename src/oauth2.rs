//! User authorization and client credentials management.

use chrono::prelude::*;
use derive_builder::Builder;
use maybe_async::maybe_async;
use serde::{Deserialize, Serialize};
use url::Url;

use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{
    env, fs,
    io::{Read, Write},
    path::Path,
};

use super::client::{ClientResult, Spotify};
use super::http::{headers, BaseClient, Form, Headers};
use super::util::generate_random_string;

mod auth_urls {
    pub const AUTHORIZE: &str = "https://accounts.spotify.com/authorize";
    pub const TOKEN: &str = "https://accounts.spotify.com/api/token";
}

mod duration_second {
    use serde::{de, Deserialize, Serializer};
    use std::time::Duration;

    /// Deserialize `std::time::Duration` from milliseconds (represented as u64)
    pub(in crate) fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let duration: u64 = Deserialize::deserialize(d)?;
        Ok(Duration::from_secs(duration))
    }

    /// Serialize `std::time::Duration` to milliseconds (represented as u64)
    pub(in crate) fn serialize<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_u64(x.as_secs())
    }
}

mod space_separated_scope {
    use serde::{de, Deserialize, Serializer};
    use std::collections::HashSet;
    use std::iter::FromIterator;
    pub(crate) fn deserialize<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let scope: &str = Deserialize::deserialize(d)?;
        Ok(HashSet::from_iter(
            scope
                .split_whitespace()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>(),
        ))
    }

    pub(crate) fn serialize<S>(scope: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_str(
            scope
                .iter()
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .join(" ")
                .as_ref(),
        )
    }
}
/// Spotify access token information
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization-guide/)
#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    /// An access token that can be provided in subsequent calls
    #[builder(setter(into))]
    pub access_token: String,
    /// The time period (in seconds) for which the access token is valid.
    #[builder(default = "Duration::from_secs(0)")]
    #[serde(with = "duration_second")]
    pub expires_in: Duration,
    /// The valid time for which the access token is available represented
    /// in ISO 8601 combined date and time.
    #[builder(setter(strip_option), default = "Some(Utc::now())")]
    pub expires_at: Option<DateTime<Utc>>,
    /// A token that can be sent to the Spotify Accounts service
    /// in place of an authorization code
    #[builder(setter(into, strip_option), default)]
    pub refresh_token: Option<String>,
    /// A list of scopes which have been granted for this `access_token`
    #[builder(default = "HashSet::new()")]
    #[serde(with = "space_separated_scope")]
    pub scope: HashSet<String>,
}

impl TokenBuilder {
    /// Tries to initialize the token from a cache file.
    pub fn from_cache<T: AsRef<Path>>(path: T) -> Self {
        if let Ok(mut file) = fs::File::open(path) {
            let mut tok_str = String::new();
            if file.read_to_string(&mut tok_str).is_ok() {
                if let Ok(tok) = serde_json::from_str::<Token>(&tok_str) {
                    return TokenBuilder {
                        access_token: Some(tok.access_token),
                        expires_in: Some(tok.expires_in),
                        expires_at: Some(tok.expires_at),
                        refresh_token: Some(tok.refresh_token),
                        scope: Some(tok.scope),
                    };
                }
            }
        }

        TokenBuilder::default()
    }
}

impl Token {
    /// Saves the token information into its cache file.
    pub fn write_cache<T: AsRef<Path>>(&self, path: T) -> ClientResult<()> {
        let token_info = serde_json::to_string(&self)?;

        let mut file = fs::OpenOptions::new().write(true).create(true).open(path)?;
        file.set_len(0)?;
        file.write_all(token_info.as_bytes())?;

        Ok(())
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map_or(true, |x| Utc::now().timestamp() > x.timestamp())
    }
}

/// Simple client credentials object for Spotify.
#[derive(Builder, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Credentials {
    #[builder(setter(into))]
    pub id: String,
    #[builder(setter(into))]
    pub secret: String,
}

impl CredentialsBuilder {
    /// Parses the credentials from the environment variables
    /// `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`. You can optionally
    /// activate the `env-file` feature in order to read these variables from
    /// a `.env` file.
    pub fn from_env() -> Self {
        #[cfg(feature = "env-file")]
        {
            dotenv::dotenv().ok();
        }

        CredentialsBuilder {
            id: env::var("RSPOTIFY_CLIENT_ID").ok(),
            secret: env::var("RSPOTIFY_CLIENT_SECRET").ok(),
        }
    }
}

/// Structure that holds the required information for requests with OAuth.
#[derive(Builder, Debug, Default, Clone, Serialize, Deserialize)]
pub struct OAuth {
    #[builder(setter(into))]
    pub redirect_uri: String,
    /// The state is generated by default, as suggested by the OAuth2 spec:
    /// https://tools.ietf.org/html/rfc6749#section-10.12
    #[builder(setter(into), default = "generate_random_string(16)")]
    pub state: String,
    #[builder(default = "HashSet::new()")]
    pub scope: HashSet<String>,
    #[builder(setter(into, strip_option), default)]
    pub proxies: Option<String>,
}

impl OAuthBuilder {
    /// Parses the credentials from the environment variable
    /// `RSPOTIFY_REDIRECT_URI`. You can optionally activate the `env-file`
    /// feature in order to read these variables from a `.env` file.
    pub fn from_env() -> Self {
        #[cfg(feature = "env-file")]
        {
            dotenv::dotenv().ok();
        }

        OAuthBuilder {
            redirect_uri: env::var("RSPOTIFY_REDIRECT_URI").ok(),
            ..Default::default()
        }
    }
}

/// Authorization-related methods for the client.
impl Spotify {
    /// Updates the cache file at the internal cache path.
    pub fn write_token_cache(&self) -> ClientResult<()> {
        if let Some(tok) = self.token.as_ref() {
            tok.write_cache(&self.cache_path)?;
        }

        Ok(())
    }

    /// Gets the required URL to authorize the current client to start the
    /// [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    pub fn get_authorize_url(&self, show_dialog: bool) -> ClientResult<String> {
        let oauth = self.get_oauth()?;
        let mut payload: HashMap<&str, &str> = HashMap::new();
        let scope = oauth
            .scope
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(" ");
        payload.insert(headers::CLIENT_ID, &self.get_creds()?.id);
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

    /// Tries to read the cache file's token, which may not exist.
    #[maybe_async]
    pub async fn read_token_cache(&mut self) -> Option<Token> {
        let tok = TokenBuilder::from_cache(&self.cache_path).build().ok()?;

        if !self.get_oauth().ok()?.scope.is_subset(&tok.scope) || tok.is_expired() {
            // Invalid token, since it doesn't have at least the currently
            // required scopes or it's expired.
            None
        } else {
            Some(tok)
        }
    }

    /// Sends a request to Spotify for an access token.
    #[maybe_async]
    async fn fetch_access_token(&self, payload: &Form) -> ClientResult<Token> {
        // This request uses a specific content type, and the client ID/secret
        // as the authentication, since the access token isn't available yet.
        let mut head = Headers::new();
        let (key, val) = headers::basic_auth(&self.get_creds()?.id, &self.get_creds()?.secret);
        head.insert(key, val);

        let response = self
            .post_form(auth_urls::TOKEN, Some(&head), payload)
            .await?;
        let mut tok = serde_json::from_str::<Token>(&response)?;
        tok.expires_at = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(tok.expires_in.as_secs() as i64));
        Ok(tok)
    }

    /// Refreshes the access token with the refresh token provided by the
    /// [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow),
    /// without saving it into the cache file.
    ///
    /// The obtained token will be saved internally.
    #[maybe_async]
    pub async fn refresh_user_token_without_cache(
        &mut self,
        refresh_token: &str,
    ) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(headers::REFRESH_TOKEN.to_owned(), refresh_token.to_owned());
        data.insert(
            headers::GRANT_TYPE.to_owned(),
            headers::GRANT_REFRESH_TOKEN.to_owned(),
        );

        let mut tok = self.fetch_access_token(&data).await?;
        tok.refresh_token = Some(refresh_token.to_string());
        self.token = Some(tok);

        Ok(())
    }

    /// The same as `refresh_user_token_without_cache`, but saves the token
    /// into the cache file if possible.
    #[maybe_async]
    pub async fn refresh_user_token(&mut self, refresh_token: &str) -> ClientResult<()> {
        self.refresh_user_token_without_cache(refresh_token).await?;

        Ok(())
    }

    /// Obtains the client access token for the app without saving it into the
    /// cache file. The resulting token is saved internally.
    #[maybe_async]
    pub async fn request_client_token_without_cache(&mut self) -> ClientResult<()> {
        let mut data = Form::new();
        data.insert(
            headers::GRANT_TYPE.to_owned(),
            headers::GRANT_CLIENT_CREDS.to_owned(),
        );

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
    }

    /// The same as `request_client_token_without_cache`, but saves the token
    /// into the cache file if possible.
    #[maybe_async]
    pub async fn request_client_token(&mut self) -> ClientResult<()> {
        self.request_client_token_without_cache().await?;
        self.write_token_cache()
    }

    /// Parse the response code in the given response url. If the URL cannot be
    /// parsed or the `code` parameter is not present, this will return `None`.
    ///
    /// Step 2 of the [Authorization Code Flow
    /// ](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    pub fn parse_response_code(&self, url: &str) -> Option<String> {
        let url = Url::parse(url).ok()?;
        let mut params = url.query_pairs();
        let (_, url) = params.find(|(key, _)| key == "code")?;
        Some(url.to_string())
    }

    /// Obtains the user access token for the app with the given code without
    /// saving it into the cache file, as part of the OAuth authentication.
    /// The access token will be saved inside the Spotify instance.
    ///
    /// Step 3 of the [Authorization Code Flow](https://developer.spotify.com/documentation/general/guides/authorization-guide/#authorization-code-flow).
    #[maybe_async]
    pub async fn request_user_token_without_cache(&mut self, code: &str) -> ClientResult<()> {
        let oauth = self.get_oauth()?;
        let mut data = Form::new();
        data.insert(
            headers::GRANT_TYPE.to_owned(),
            headers::GRANT_AUTH_CODE.to_owned(),
        );
        data.insert(headers::REDIRECT_URI.to_owned(), oauth.redirect_uri.clone());
        data.insert(headers::CODE.to_owned(), code.to_owned());
        data.insert(
            headers::SCOPE.to_owned(),
            oauth
                .scope
                .clone()
                .into_iter()
                .collect::<Vec<String>>()
                .join(" "),
        );
        data.insert(headers::STATE.to_owned(), oauth.state.clone());

        self.token = Some(self.fetch_access_token(&data).await?);

        Ok(())
    }

    /// The same as `request_user_token_without_cache`, but saves the token into
    /// the cache file if possible.
    #[maybe_async]
    pub async fn request_user_token(&mut self, code: &str) -> ClientResult<()> {
        self.request_user_token_without_cache(code).await?;
        self.write_token_cache()
    }

    /// Opens up the authorization URL in the user's browser so that it can
    /// authenticate. It also reads from the standard input the redirect URI
    /// in order to obtain the access token information. The resulting access
    /// token will be saved internally once the operation is successful.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_user_token_without_cache(&mut self) -> ClientResult<()> {
        let code = self.get_code_from_user()?;
        self.request_user_token_without_cache(&code).await?;

        Ok(())
    }

    /// The same as the `prompt_for_user_token_without_cache` method, but it
    /// will try to use the user token into the cache file, and save it in
    /// case it didn't exist/was invalid.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    #[maybe_async]
    pub async fn prompt_for_user_token(&mut self) -> ClientResult<()> {
        // TODO: shouldn't this also refresh the obtained token?
        self.token = self.read_token_cache().await;

        // Otherwise following the usual procedure to get the token.
        if self.token.is_none() {
            let code = self.get_code_from_user()?;
            // Will write to the cache file if successful
            self.request_user_token(&code).await?;
        }

        Ok(())
    }

    /// Tries to open the authorization URL in the user's browser, and returns
    /// the obtained code.
    ///
    /// Note: this method requires the `cli` feature.
    #[cfg(feature = "cli")]
    fn get_code_from_user(&self) -> ClientResult<String> {
        use crate::client::ClientError;

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
            .ok_or_else(|| ClientError::CLI("unable to parse the response code".to_string()))?;

        Ok(code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::SpotifyBuilder;

    use std::collections::HashSet;
    use std::fs;
    use std::io::Read;
    use std::iter::FromIterator;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_write_token() {
        let now: DateTime<Utc> = Utc::now();
        let scope: HashSet<String>  = HashSet::from_iter("playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private streaming ugc-image-upload user-follow-modify user-follow-read user-library-read user-library-modify user-read-private user-read-birthdate user-read-email user-top-read user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played".split_whitespace().map(|x|x.to_owned()).collect::<Vec<String>>());
        let tok = TokenBuilder::default()
            .access_token("test-access_token")
            .expires_in(Duration::from_secs(3600))
            .expires_at(now)
            .scope(scope.clone())
            .refresh_token("...")
            .build()
            .unwrap();

        let spotify = SpotifyBuilder::default()
            .token(tok.clone())
            .build()
            .unwrap();

        let tok_str = serde_json::to_string(&tok).unwrap();
        spotify.write_token_cache().unwrap();

        let mut file = fs::File::open(&spotify.cache_path).unwrap();
        let mut tok_str_file = String::new();
        file.read_to_string(&mut tok_str_file).unwrap();

        assert_eq!(tok_str, tok_str_file);
        let tok_from_file: Token = serde_json::from_str(&tok_str_file).unwrap();
        assert_eq!(tok_from_file.scope, scope);
        assert_eq!(tok_from_file.expires_in, Duration::from_secs(3600));
        assert_eq!(tok_from_file.expires_at.unwrap(), now);
    }

    #[test]
    fn test_token_is_expired() {
        let scope: HashSet<String>  = HashSet::from_iter("playlist-read-private playlist-read-collaborative playlist-modify-public playlist-modify-private streaming ugc-image-upload user-follow-modify user-follow-read user-library-read user-library-modify user-read-private user-read-birthdate user-read-email user-top-read user-read-playback-state user-modify-playback-state user-read-currently-playing user-read-recently-played".split_whitespace().map(|x|x.to_owned()).collect::<Vec<String>>());
        let tok = TokenBuilder::default()
            .access_token("test-access_token")
            .expires_in(Duration::from_secs(1))
            .expires_at(Utc::now())
            .scope(scope)
            .refresh_token("...")
            .build()
            .unwrap();
        assert!(!tok.is_expired());
        sleep(Duration::from_secs(2));
        assert!(tok.is_expired());
    }

    #[test]
    fn test_parse_response_code() {
        let spotify = SpotifyBuilder::default().build().unwrap();

        let url = "http://localhost:8888/callback";
        let code = spotify.parse_response_code(url);
        assert_eq!(code, None);

        let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw";
        let code = spotify.parse_response_code(url);
        assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));

        let url = "http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN#_=_";
        let code = spotify.parse_response_code(url);
        assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));

        let url = "http://localhost:8888/callback?state=sN&code=AQD0yXvFEOvw#_=_";
        let code = spotify.parse_response_code(url);
        assert_eq!(code, Some("AQD0yXvFEOvw".to_string()));
    }
}

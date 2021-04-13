//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients interchangeably.

// TODO: check this
// Disable all modules when both client features are enabled or when none are.
// This way only the compile error below gets shown instead of a whole list of
// confusing errors..

#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

#[cfg(all(feature = "client-reqwest", feature = "client-ureq"))]
compile_error!(
    "`client-reqwest` and `client-ureq` features cannot both be enabled at \
    the same time, if you want to use `client-ureq` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "client-reqwest", feature = "client-ureq")))]
compile_error!(
    "You have to enable at least one of the available clients with the \
    `client-reqwest` or `client-ureq` features."
);

use crate::{ClientResult, Spotify};

use std::collections::HashMap;
use std::fmt;

use maybe_async::maybe_async;
use serde_json::Value;

#[cfg(feature = "client-reqwest")]
pub use self::reqwest::ReqwestClient as HTTPClient;
#[cfg(feature = "client-ureq")]
pub use self::ureq::UreqClient as HTTPClient;

pub type Headers = HashMap<String, String>;
pub type Query = HashMap<String, String>;
pub type Form = HashMap<String, String>;

pub mod headers {
    use crate::auth::Token;

    // Common headers as constants
    pub const CLIENT_ID: &str = "client_id";
    pub const CODE: &str = "code";
    pub const GRANT_AUTH_CODE: &str = "authorization_code";
    pub const GRANT_CLIENT_CREDS: &str = "client_credentials";
    pub const GRANT_REFRESH_TOKEN: &str = "refresh_token";
    pub const GRANT_TYPE: &str = "grant_type";
    pub const REDIRECT_URI: &str = "redirect_uri";
    pub const REFRESH_TOKEN: &str = "refresh_token";
    pub const RESPONSE_CODE: &str = "code";
    pub const RESPONSE_TYPE: &str = "response_type";
    pub const SCOPE: &str = "scope";
    pub const SHOW_DIALOG: &str = "show_dialog";
    pub const STATE: &str = "state";

    /// Generates an HTTP token authorization header with proper formatting
    pub fn bearer_auth(tok: &Token) -> (String, String) {
        let auth = "authorization".to_owned();
        let value = format!("Bearer {}", tok.access_token);

        (auth, value)
    }

    /// Generates an HTTP basic authorization header with proper formatting
    pub fn basic_auth(user: &str, password: &str) -> (String, String) {
        let auth = "authorization".to_owned();
        let value = format!("{}:{}", user, password);
        let value = format!("Basic {}", base64::encode(value));

        (auth, value)
    }
}

/// This trait represents the interface to be implemented for an HTTP client,
/// which is kept separate from the Spotify client for cleaner code. Thus, it
/// also requires other basic traits that are needed for the Spotify client.
///
/// When a request doesn't need to pass parameters, the empty or default value
/// of the payload type should be passed, like `json!({})` or `Query::new()`.
/// This avoids using `Option<T>` because `Value` itself may be null in other
/// different ways (`Value::Null`, an empty `Value::Object`...), so this removes
/// redundancy and edge cases (a `Some(Value::Null), for example, doesn't make
/// much sense).
#[maybe_async]
pub trait BaseHTTPClient: Default + Clone + fmt::Debug {
    // This internal function should always be given an object value in JSON.
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> ClientResult<String>;

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String>;

    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> ClientResult<String>;

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String>;

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String>;
}

/// HTTP-related methods for the Spotify client. It wraps the basic HTTP client
/// with features needed of higher level.
///
/// The Spotify client has two different wrappers to perform requests:
///
/// * Basic wrappers: `get`, `post`, `put`, `delete`, `post_form`. These only
///   append the configured Spotify API URL to the relative URL provided so that
///   it's not forgotten. They're used in the authentication process to request
///   an access token and similars.
/// * Endpoint wrappers: `endpoint_get`, `endpoint_post`, `endpoint_put`,
///   `endpoint_delete`. These append the authentication headers for endpoint
///   requests to reduce the code needed for endpoints and make them as concise
///   as possible.
impl Spotify {
    /// If it's a relative URL like "me", the prefix is appended to it.
    /// Otherwise, the same URL is returned.
    fn endpoint_url(&self, url: &str) -> String {
        // Using the client's prefix in case it's a relative route.
        if !url.starts_with("http") {
            self.prefix.clone() + url
        } else {
            url.to_string()
        }
    }

    /// The headers required for authenticated requests to the API
    fn auth_headers(&self) -> ClientResult<Headers> {
        let mut auth = Headers::new();
        let (key, val) = headers::bearer_auth(self.get_token()?);
        auth.insert(key, val);

        Ok(auth)
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);
        self.http.get(&url, headers, payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);
        self.http.post(&url, headers, payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);
        self.http.post_form(&url, headers, payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);
        self.http.put(&url, headers, payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);
        self.http.delete(&url, headers, payload).await
    }

    /// The wrapper for the endpoints, which also includes the required
    /// autentication.
    #[inline]
    #[maybe_async]
    pub(crate) async fn endpoint_get(&self, url: &str, payload: &Query) -> ClientResult<String> {
        let headers = self.auth_headers()?;
        self.get(url, Some(&headers), payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn endpoint_post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let headers = self.auth_headers()?;
        self.post(url, Some(&headers), payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn endpoint_put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let headers = self.auth_headers()?;
        self.put(url, Some(&headers), payload).await
    }

    #[inline]
    #[maybe_async]
    pub(crate) async fn endpoint_delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        let headers = self.auth_headers()?;
        self.delete(url, Some(&headers), payload).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rspotify_client::SpotifyBuilder;
    use rspotify_auth2::TokenBuilder;
    use rspotify_macros::scopes;
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    fn test_bearer_auth() {
        let access_token = "access_token";
        let tok = TokenBuilder::default()
            .access_token(access_token)
            .build()
            .unwrap();
        let (auth, value) = headers::bearer_auth(&tok);
        assert_eq!(auth, "authorization");
        assert_eq!(value, "Bearer access_token");
    }

    #[test]
    fn test_basic_auth() {
        let (auth, value) = headers::basic_auth("ramsay", "123456");
        assert_eq!(auth, "authorization");
        assert_eq!(value, "Basic cmFtc2F5OjEyMzQ1Ng==");
    }

    #[test]
    fn test_endpoint_url() {
        let spotify = SpotifyBuilder::default().build().unwrap();
        assert_eq!(
            spotify.endpoint_url("me/player/play"),
            "https://api.spotify.com/v1/me/player/play"
        );
        assert_eq!(
            spotify.endpoint_url("http://api.spotify.com/v1/me/player/play"),
            "http://api.spotify.com/v1/me/player/play"
        );
        assert_eq!(
            spotify.endpoint_url("https://api.spotify.com/v1/me/player/play"),
            "https://api.spotify.com/v1/me/player/play"
        );
    }

    #[test]
    fn test_auth_headers() {
        let tok = TokenBuilder::default()
            .access_token("test-access_token")
            .expires_in(Duration::seconds(1))
            .expires_at(Utc::now())
            .scope(scopes!("playlist-read-private"))
            .refresh_token("...")
            .build()
            .unwrap();

        let spotify = SpotifyBuilder::default().token(tok).build().unwrap();

        let headers = spotify.auth_headers().unwrap();
        assert_eq!(
            headers.get("authorization"),
            Some(&"Bearer test-access_token".to_owned())
        );
    }
}

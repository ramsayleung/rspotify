//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients
//! interchangeably.

#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

use crate::client::{ClientResult, Spotify};

use std::collections::HashMap;

use maybe_async::maybe_async;
use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type FormData = HashMap<String, String>;

pub mod headers {
    use crate::oauth2::Token;

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

/// The default headers will be overriden if its value is other than None.
///
/// When any of the request doesn't need parameters, the empty or default value
/// of the payload type should be passed. For example, `json!({})` to `get`.
/// This avoids using `Option<T>` because `Value` itself may be null in
/// other different ways (`Value::Null`, an empty `Value::Object`...), so this
/// removes redundancy and edge cases (a `Some(Value::Null), for example,
/// doesn't make much sense).
#[maybe_async]
pub trait BaseClient {
    // This internal function should always be given an object value in JSON.
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
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
        payload: &FormData,
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

/// HTTP-related methods for the client.
impl Spotify {
    /// If it's a relative URL (`"me"`), the prefix is appended to it
    /// (`"https://api.spotify.com/v1/me"`). Otherwise, the same URL is
    /// returned.
    fn endpoint_url(&self, url: &str) -> String {
        // Using the client's prefix in case it's a relative route.
        if !url.starts_with("http") {
            self.prefix.clone() + url
        } else {
            url.to_string()
        }
    }
}

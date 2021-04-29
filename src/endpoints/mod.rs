pub mod base;
pub mod oauth;
pub mod pagination;

pub use base::BaseClient;
pub use oauth::OAuthClient;

use crate::{
    model::{idtypes::IdType, Id},
    ClientResult,
};

use serde::Deserialize;

/// Converts a JSON response from Spotify into its model.
pub(in crate) fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> ClientResult<T> {
    serde_json::from_str::<T>(input).map_err(Into::into)
}

/// Append device ID to an API path.
pub(in crate) fn append_device_id(path: &str, device_id: Option<&str>) -> String {
    let mut new_path = path.to_string();
    if let Some(_device_id) = device_id {
        if path.contains('?') {
            new_path.push_str(&format!("&device_id={}", _device_id));
        } else {
            new_path.push_str(&format!("?device_id={}", _device_id));
        }
    }
    new_path
}

#[inline]
pub(in crate) fn join_ids<'a, T: 'a + IdType>(ids: impl IntoIterator<Item = &'a Id<T>>) -> String {
    ids.into_iter().collect::<Vec<_>>().join(",")
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
        payload: &Query<'_>,
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
        payload: &Form<'_>,
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
    pub(crate) async fn endpoint_get(
        &self,
        url: &str,
        payload: &Query<'_>,
    ) -> ClientResult<String> {
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

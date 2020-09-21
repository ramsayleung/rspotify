//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients
//! interchangeably.

#[cfg(feature = "client-reqwest")]
mod reqwest;

use crate::client::ClientResult;

use maybe_async::maybe_async;
use serde_json::Value;
use std::collections::HashMap;

/// TODO: this should not be public
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[maybe_async]
pub trait BaseClient {
    async fn request(
        &self,
        method: HTTPMethod,
        url: &str,
        payload: Option<&Value>,
    ) -> ClientResult<String>;

    async fn get(&self, url: &str, params: &mut HashMap<String, String>) -> ClientResult<String>;

    #[inline]
    async fn post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(HTTPMethod::POST, url, Some(payload)).await
    }

    #[inline]
    async fn put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(HTTPMethod::PUT, url, Some(payload)).await
    }

    #[inline]
    async fn delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(HTTPMethod::DELETE, url, Some(payload)).await
    }
}

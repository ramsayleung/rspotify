//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients
//! interchangeably.

#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

use crate::client::ClientResult;

use maybe_async::maybe_async;
use serde_json::Value;

/// TODO: this should not be public
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[maybe_async]
pub trait BaseClient {
    async fn get(&self, url: &str, params: &Value) -> ClientResult<String>;
    async fn post(&self, url: &str, payload: &Value) -> ClientResult<String>;
    async fn put(&self, url: &str, payload: &Value) -> ClientResult<String>;
    async fn delete(&self, url: &str, payload: &Value) -> ClientResult<String>;
}

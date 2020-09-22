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

pub type Headers = std::collections::HashMap<String, String>;

pub mod headers {
    pub const CONTENT_TYPE: &str = "Content-Type";
    pub const AUTHORIZATION: &str = "Authorization";

    pub fn bearer_auth(token: &str) -> String {
        format!("Bearer {}", token)
    }

    pub fn basic_auth(user: &str, password: &str) -> String {
        format!("Basic {}:{}", user, password)
    }
}

/// The default headers will be overriden if its value is other than None.
#[maybe_async]
pub trait BaseClient {
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        params: &Value,
    ) -> ClientResult<String>;
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
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

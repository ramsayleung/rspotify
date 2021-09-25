use std::collections::HashMap;
use std::fmt;

use maybe_async::maybe_async;
use rspotify_model::ApiError;
use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

/// Any kind of error when performing an HTTP request
#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    /// Error specific to the ureq queries
    #[cfg(feature = "client-ureq")]
    #[error("ureq: {0}")]
    Ureq(#[from] crate::ureq::Error),

    /// Error specific to the reqwest queries
    #[cfg(feature = "client-reqwest")]
    #[error("reqwest: {0}")]
    Reqwest(#[from] crate::reqwest::Error),

    /// Something failed server-side (a logic error, the server is down, etc)
    #[error("unsuccessful status code: {0}")]
    StatusCode(u16, Option<ApiError>),
}

pub type HttpResult<T> = Result<T, HttpError>;

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
pub trait BaseHttpClient: Send + Default + Clone + fmt::Debug {
    // This internal function should always be given an object value in JSON.
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> HttpResult<String>;

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String>;

    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> HttpResult<String>;

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String>;

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String>;
}

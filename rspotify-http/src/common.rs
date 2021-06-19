use std::collections::HashMap;
use std::fmt;

use maybe_async::maybe_async;
use rspotify_model::ApiError;
use serde_json::Value;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("request unauthorized")]
    Unauthorized,

    #[error("exceeded request limit")]
    RateLimited(Option<usize>),

    #[error("request error: {0}")]
    Request(String),

    #[error("status code {0}: {1}")]
    StatusCode(u16, String),

    #[error("spotify error: {0}")]
    Api(#[from] ApiError),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

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
    async fn get(&self, url: &str, headers: Option<&Headers>, payload: &Query) -> Result<String>;

    async fn post(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> Result<String>;

    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> Result<String>;

    async fn put(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> Result<String>;

    async fn delete(&self, url: &str, headers: Option<&Headers>, payload: &Value)
        -> Result<String>;
}

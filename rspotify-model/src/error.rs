use serde::Deserialize;
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ModelResult<T> = Result<T, ModelError>;

/// Matches errors that are returned from the Spotfiy
/// API as part of the JSON response object.
#[derive(Debug, Error, Deserialize)]
pub enum ApiError {
    /// See [Error Object](https://developer.spotify.com/documentation/web-api/reference/#object-errorobject)
    #[error("{status}: {message}")]
    #[serde(alias = "error")]
    Regular { status: u16, message: String },

    /// See [Play Error Object](https://developer.spotify.com/documentation/web-api/reference/#object-playererrorobject)
    #[error("{status} ({reason}): {message}")]
    #[serde(alias = "error")]
    Player {
        status: u16,
        message: String,
        reason: String,
    },
}

/// Groups up the kinds of errors that may happen in this crate.
#[derive(Debug, Error)]
pub enum ModelError {
    #[error("json parse error: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("input/output error: {0}")]
    Io(#[from] std::io::Error),
}

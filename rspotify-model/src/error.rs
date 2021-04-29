use serde::Deserialize;

/// Matches errors that are returned from the Spotfiy
/// API as part of the JSON response object.
#[derive(Debug, thiserror::Error, Deserialize)]
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

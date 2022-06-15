//! All objects related to the auth flows defined by Spotify API

use crate::custom_serde::{duration_second, space_separated_scopes};

use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    fs, io,
    path::Path,
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Spotify access token information
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization/)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    /// An access token that can be provided in subsequent calls
    pub access_token: String,
    /// The time period for which the access token is valid.
    #[serde(with = "duration_second")]
    pub expires_in: Duration,
    /// The valid time for which the access token is available represented
    /// in ISO 8601 combined date and time.
    pub expires_at: Option<DateTime<Utc>>,
    /// A token that can be sent to the Spotify Accounts service
    /// in place of an authorization code
    pub refresh_token: Option<String>,
    /// A list of [scopes](https://developer.spotify.com/documentation/general/guides/authorization/scopes/)
    /// which have been granted for this `access_token`
    ///
    /// You may use the `scopes!` macro in
    /// [`rspotify-macros`](https://docs.rs/rspotify-macros) to build it at
    /// compile time easily.
    // The token response from spotify is singular, hence the rename to `scope`
    #[serde(default, with = "space_separated_scopes", rename = "scope")]
    pub scopes: HashSet<String>,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            access_token: String::new(),
            expires_in: Duration::seconds(0),
            expires_at: Some(Utc::now()),
            refresh_token: None,
            scopes: HashSet::new(),
        }
    }
}

impl Token {
    /// Tries to initialize the token from a cache file.
    pub fn from_cache<T: AsRef<Path>>(path: T) -> Result<Self, ReadTokenCacheError> {
        let json = read_file(path.as_ref()).map_err(ReadTokenCacheError::Reading)?;

        let tok =
            serde_json::from_slice::<Token>(&*json).map_err(ReadTokenCacheError::Deserializing)?;

        Ok(tok)
    }

    /// Saves the token information into its cache file.
    pub fn write_cache<T: AsRef<Path>>(&self, path: T) -> Result<(), WriteTokenCacheError> {
        // `serde_json::to_vec` only errors if our `Serialize` implementation
        // fails, which it shouldn’t, or we try to serialize a map with
        // non-string keys, which we don’t.
        let json = serde_json::to_vec(&self).unwrap();

        write_file(path.as_ref(), &*json).map_err(|inner| WriteTokenCacheError { inner })?;

        Ok(())
    }

    /// Check if the token is expired. It includes a margin of 10 seconds (which
    /// is how much a request would take in the worst case scenario).
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(true, |expiration| {
            Utc::now() + Duration::seconds(10) >= expiration
        })
    }

    /// Generates an HTTP token authorization header with proper formatting
    pub fn auth_headers(&self) -> HashMap<String, String> {
        let auth = "authorization".to_owned();
        let value = format!("Bearer {}", self.access_token);

        let mut headers = HashMap::new();
        headers.insert(auth, value);
        headers
    }
}

/// An error reading a cached [`Token`].
#[derive(Debug, Error)]
#[error("failed to read token from cache")]
pub enum ReadTokenCacheError {
    /// There was an error reading the cache file into memory.
    Reading(#[source] ReadFileError),

    /// There was an error deserializing the contents of the cache file.
    Deserializing(#[source] serde_json::Error),
}

/// An error writing a [`Token`] to its cache.
#[derive(Debug, Error)]
#[non_exhaustive]
#[error("failed to write token to cache")]
pub struct WriteTokenCacheError {
    /// The underlying error in writing the [`Token`] file cache.
    #[source]
    pub inner: WriteFileError,
}

fn read_file(path: &Path) -> Result<Vec<u8>, ReadFileError> {
    fs::read(path).map_err(|inner| ReadFileError {
        inner,
        path: Box::from(path),
    })
}

/// An error reading a file.
#[derive(Debug, Error)]
#[error("failed to read file {}", path.display())]
pub struct ReadFileError {
    // Intentionally not exposed to allow future API evolution, e.g., moving
    // this to a enum variants `Open(io::Error)` and `Read(io::Error)`
    #[source]
    inner: io::Error,
    path: Box<Path>,
}

impl ReadFileError {
    /// Returns a shared reference to the underlying I/O that caused this.
    #[must_use]
    pub fn io(&self) -> &io::Error {
        &self.inner
    }

    /// Consumes this error type, returning the underlying inner I/O error.
    ///
    /// It is not recommended to use this method just to unify error types, as
    /// you will lose valuable information in the error message.
    #[must_use]
    pub fn into_io(self) -> io::Error {
        self.inner
    }

    /// Returns a shared reference to the path that could not be read.
    #[must_use]
    pub fn path(&self) -> &Path {
        &*self.path
    }
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), WriteFileError> {
    fs::write(path, bytes).map_err(|inner| WriteFileError {
        inner,
        path: Box::from(path),
    })
}

/// An error writing a file.
#[derive(Debug, Error)]
#[error("failed to write file {}", path.display())]
pub struct WriteFileError {
    // Intentionally not exposed to allow future API evolution, e.g., moving
    // this to an enum variant `Open(io::Error)` and `Write(io::Error)`
    #[source]
    inner: io::Error,
    path: Box<Path>,
}

impl WriteFileError {
    /// Returns a shared reference to the underlying I/O that caused this.
    #[must_use]
    pub fn io(&self) -> &io::Error {
        &self.inner
    }

    /// Consumes this error type, returning the underlying inner I/O error.
    ///
    /// It is not recommended to use this method just to unify error types, as
    /// you will lose valuable information in the error message.
    #[must_use]
    pub fn into_io(self) -> io::Error {
        self.inner
    }

    /// Returns a shared reference to the path that could not be written to.
    #[must_use]
    pub fn path(&self) -> &Path {
        &*self.path
    }
}

#[cfg(test)]
mod test {
    use crate::Token;

    #[test]
    fn test_bearer_auth() {
        let tok = Token {
            access_token: "access_token".to_string(),
            ..Default::default()
        };

        let headers = tok.auth_headers();
        assert_eq!(headers.len(), 1);
        assert_eq!(
            headers.get("authorization"),
            Some(&"Bearer access_token".to_owned())
        );
    }
}

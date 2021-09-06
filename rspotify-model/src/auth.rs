//! All objects related to the auth flows defined by Spotify API

use crate::{
    custom_serde::{duration_second, space_separated_scopes},
    ModelResult,
};

use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    path::Path,
};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Spotify access token information
///
/// [Reference](https://developer.spotify.com/documentation/general/guides/authorization-guide/)
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    /// A list of [scopes](https://developer.spotify.com/documentation/general/guides/scopes/)
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
    pub fn from_cache<T: AsRef<Path>>(path: T) -> ModelResult<Self> {
        let mut file = fs::File::open(path)?;
        let mut tok_str = String::new();
        file.read_to_string(&mut tok_str)?;
        let tok = serde_json::from_str::<Token>(&tok_str)?;

        Ok(tok)
    }

    /// Saves the token information into its cache file.
    pub fn write_cache<T: AsRef<Path>>(&self, path: T) -> ModelResult<()> {
        let token_info = serde_json::to_string(&self)?;

        let mut file = fs::OpenOptions::new().write(true).create(true).open(path)?;
        file.set_len(0)?;
        file.write_all(token_info.as_bytes())?;

        Ok(())
    }

    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at
            .map_or(true, |x| Utc::now().timestamp() > x.timestamp())
    }
}

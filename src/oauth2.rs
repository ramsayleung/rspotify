//! The module contains function about authorization and client-credential
// Use 3rd party library
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

// Use built-in library
use std::env;

/// Spotify access token information.
#[derive(Builder, Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: String,
}

/// Simple client credentials object for Spotify.
#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub id: String,
    pub secret: String,
}

impl ClientCredentialsBuilder {
    /// Parses the credentials from the environment variables
    /// `RSPOTIFY_CLIENT_ID` and `RSPOTIFY_CLIENT_SECRET`. You can optionally
    /// activate the `env-file` feature in order to read these variables from
    /// a `.env` file.
    pub fn from_env() -> Self {
        #[cfg(feature = "env-file")]
        {
            dotenv::dotenv().ok();
        }
        let id = env::var("RSPOTIFY_CLIENT_ID").ok();
        let secret = env::var("RSPOTIFY_CLIENT_SECRET").ok();
        ClientCredentialsBuilder {
            id,
            secret,
            ..Default::default()
        }
    }
}

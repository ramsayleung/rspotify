//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients interchangeably.

#[cfg(feature = "client-reqwest")]
mod reqwest;
#[cfg(feature = "client-ureq")]
mod ureq;

// #[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
// #[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]

#[cfg(all(feature = "client-reqwest", feature = "client-ureq"))]
compile_error!(
    "`client-reqwest` and `client-ureq` features cannot both be enabled at \
    the same time, if you want to use `client-ureq` you need to set \
    `default-features = false`"
);

#[cfg(not(any(feature = "client-reqwest", feature = "client-ureq")))]
compile_error!(
    "You have to enable at least one of the available clients with the \
    `client-reqwest` or `client-ureq` features."
);

use std::collections::HashMap;
use std::fmt;

use maybe_async::maybe_async;
use serde_json::Value;
use rspotify_model::ApiError;

#[cfg(feature = "client-reqwest")]
pub use self::reqwest::ReqwestClient as Client;
#[cfg(feature = "client-ureq")]
pub use self::ureq::UreqClient as Client;

pub type Headers = HashMap<String, String>;
pub type Query<'a> = HashMap<&'a str, &'a str>;
pub type Form<'a> = HashMap<&'a str, &'a str>;

pub mod headers {
    // Common headers as constants
    pub const CLIENT_ID: &str = "client_id";
    pub const CODE: &str = "code";
    pub const GRANT_AUTH_CODE: &str = "authorization_code";
    pub const GRANT_CLIENT_CREDS: &str = "client_credentials";
    pub const GRANT_REFRESH_TOKEN: &str = "refresh_token";
    pub const GRANT_TYPE: &str = "grant_type";
    pub const REDIRECT_URI: &str = "redirect_uri";
    pub const REFRESH_TOKEN: &str = "refresh_token";
    pub const RESPONSE_CODE: &str = "code";
    pub const RESPONSE_TYPE: &str = "response_type";
    pub const SCOPE: &str = "scope";
    pub const SHOW_DIALOG: &str = "show_dialog";
    pub const STATE: &str = "state";
}

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
pub trait BaseClient: Default + Clone + fmt::Debug {
    // This internal function should always be given an object value in JSON.
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String>;

    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String>;

    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> Result<String>;

    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String>;

    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String>;
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::client::SpotifyBuilder;
    use crate::oauth2::TokenBuilder;
    use crate::scopes;
    use chrono::prelude::*;
    use chrono::Duration;

    #[test]
    fn test_bearer_auth() {
        let access_token = "access_token";
        let tok = TokenBuilder::default()
            .access_token(access_token)
            .build()
            .unwrap();
        let (auth, value) = headers::bearer_auth(&tok);
        assert_eq!(auth, "authorization");
        assert_eq!(value, "Bearer access_token");
    }

    #[test]
    fn test_basic_auth() {
        let (auth, value) = headers::basic_auth("ramsay", "123456");
        assert_eq!(auth, "authorization");
        assert_eq!(value, "Basic cmFtc2F5OjEyMzQ1Ng==");
    }

    #[test]
    fn test_endpoint_url() {
        let spotify = SpotifyBuilder::default().build().unwrap();
        assert_eq!(
            spotify.endpoint_url("me/player/play"),
            "https://api.spotify.com/v1/me/player/play"
        );
        assert_eq!(
            spotify.endpoint_url("http://api.spotify.com/v1/me/player/play"),
            "http://api.spotify.com/v1/me/player/play"
        );
        assert_eq!(
            spotify.endpoint_url("https://api.spotify.com/v1/me/player/play"),
            "https://api.spotify.com/v1/me/player/play"
        );
    }

    #[test]
    fn test_auth_headers() {
        let tok = TokenBuilder::default()
            .access_token("test-access_token")
            .expires_in(Duration::seconds(1))
            .expires_at(Utc::now())
            .scope(scopes!("playlist-read-private"))
            .refresh_token("...")
            .build()
            .unwrap();

        let spotify = SpotifyBuilder::default().token(tok).build().unwrap();

        let headers = spotify.auth_headers().unwrap();
        assert_eq!(
            headers.get("authorization"),
            Some(&"Bearer test-access_token".to_owned())
        );
    }
}

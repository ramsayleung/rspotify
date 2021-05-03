//! The HTTP client may vary depending on which one the user configures. This
//! module contains the required logic to use different clients interchangeably.

// Disable all modules when both client features are enabled or when none are.
// This way only the compile error below gets shown instead of a whole list of
// confusing errors..

#[cfg(feature = "client-reqwest")]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
mod reqwest;

#[cfg(feature = "client-ureq")]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
mod ureq;

#[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
mod common;

#[cfg(feature = "client-reqwest")]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub use self::reqwest::ReqwestClient as HttpClient;

#[cfg(feature = "client-ureq")]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub use self::ureq::UreqClient as HttpClient;

#[cfg(any(feature = "client-reqwest", feature = "client-ureq"))]
#[cfg(not(all(feature = "client-reqwest", feature = "client-ureq")))]
pub use common::{Error, Result, BaseHttpClient, Form, Headers, Query};

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

mod base;
mod oauth;
pub mod pagination;

pub use base::BaseClient;
pub use oauth::OAuthClient;

use crate::ClientResult;

use std::fmt::Write as _;

use serde::Deserialize;

/// Converts a JSON response from Spotify into its model.
pub(crate) fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> ClientResult<T> {
    serde_json::from_str::<T>(input).map_err(Into::into)
}

/// Append device ID to an API path.
pub(crate) fn append_device_id(path: &str, device_id: Option<&str>) -> String {
    let mut new_path = path.to_string();
    if let Some(device_id) = device_id {
        if path.contains('?') {
            let _ = write!(new_path, "&device_id={device_id}");
        } else {
            let _ = write!(new_path, "?device_id={device_id}");
        }
    }
    new_path
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{model::Token, scopes, ClientCredsSpotify, Config};
    use chrono::{prelude::*, Duration};

    #[test]
    fn test_append_device_id_without_question_mark() {
        let path = "me/player/play";
        let device_id = Some("fdafdsadfa");
        let new_path = append_device_id(path, device_id);
        assert_eq!(new_path, "me/player/play?device_id=fdafdsadfa");
    }

    #[test]
    fn test_append_device_id_with_question_mark() {
        let path = "me/player/shuffle?state=true";
        let device_id = Some("fdafdsadfa");
        let new_path = append_device_id(path, device_id);
        assert_eq!(
            new_path,
            "me/player/shuffle?state=true&device_id=fdafdsadfa"
        );
    }

    #[test]
    fn test_api_url() {
        let mut spotify = ClientCredsSpotify::default();
        assert_eq!(
            spotify.api_url("me/player/play"),
            "https://api.spotify.com/v1/me/player/play"
        );

        spotify.config = Config {
            api_base_url: String::from("http://localhost:8080/api/v1/"),
            ..Default::default()
        };
        assert_eq!(
            spotify.api_url("me/player/play"),
            "http://localhost:8080/api/v1/me/player/play"
        );

        // Also works without trailing character
        spotify.config = Config {
            api_base_url: String::from("http://localhost:8080/api/v1"),
            ..Default::default()
        };
        assert_eq!(
            spotify.api_url("me/player/play"),
            "http://localhost:8080/api/v1/me/player/play"
        );
    }

    #[test]
    fn test_auth_url() {
        let mut spotify = ClientCredsSpotify::default();
        assert_eq!(
            spotify.auth_url("api/token"),
            "https://accounts.spotify.com/api/token"
        );

        spotify.config = Config {
            auth_base_url: String::from("http://localhost:8080/accounts/"),
            ..Default::default()
        };
        assert_eq!(
            spotify.auth_url("api/token"),
            "http://localhost:8080/accounts/api/token"
        );

        // Also works without trailing character
        spotify.config = Config {
            auth_base_url: String::from("http://localhost:8080/accounts"),
            ..Default::default()
        };
        assert_eq!(
            spotify.auth_url("api/token"),
            "http://localhost:8080/accounts/api/token"
        );
    }

    #[maybe_async::test(feature = "__sync", async(feature = "__async", tokio::test))]
    async fn test_auth_headers() {
        let tok = Token {
            access_token: "test-access_token".to_string(),
            expires_in: Duration::try_seconds(1).unwrap(),
            expires_at: Some(Utc::now()),
            scopes: scopes!("playlist-read-private"),
            refresh_token: Some("...".to_string()),
        };

        let headers = tok.auth_headers();
        assert_eq!(
            headers.get("authorization"),
            Some(&"Bearer test-access_token".to_owned())
        );
    }
}

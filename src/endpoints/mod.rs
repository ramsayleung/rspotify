pub mod base;
pub mod oauth;
pub mod pagination;

pub use base::BaseClient;
pub use oauth::OAuthClient;

use crate::{
    endpoints::pagination::Paginator,
    model::{idtypes::IdType, Id},
    ClientResult, Token,
};

use std::pin::Pin;

use serde::Deserialize;

/// Converts a JSON response from Spotify into its model.
pub(in crate) fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> ClientResult<T> {
    serde_json::from_str::<T>(input).map_err(Into::into)
}

/// Append device ID to an API path.
pub(in crate) fn append_device_id(path: &str, device_id: Option<&str>) -> String {
    let mut new_path = path.to_string();
    if let Some(_device_id) = device_id {
        if path.contains('?') {
            new_path.push_str(&format!("&device_id={}", _device_id));
        } else {
            new_path.push_str(&format!("?device_id={}", _device_id));
        }
    }
    new_path
}

// TODO: move to `lib.rs`
#[inline]
pub(in crate) fn join_ids<'a, T: 'a + IdType>(ids: impl IntoIterator<Item = &'a Id<T>>) -> String {
    ids.into_iter().collect::<Vec<_>>().join(",")
}

// TODO: move to `lib.rs` or integrate into Token.
/// Generates an HTTP token authorization header with proper formatting
pub fn bearer_auth(tok: &Token) -> (String, String) {
    let auth = "authorization".to_owned();
    let value = format!("Bearer {}", tok.access_token);

    (auth, value)
}

// TODO: move to `lib.rs` or integrate into Credentials.
/// Generates an HTTP basic authorization header with proper formatting
pub fn basic_auth(user: &str, password: &str) -> (String, String) {
    let auth = "authorization".to_owned();
    let value = format!("{}:{}", user, password);
    let value = format!("Basic {}", base64::encode(value));

    (auth, value)
}

#[cfg(test)]
mod test {
    use super::append_device_id;

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
}

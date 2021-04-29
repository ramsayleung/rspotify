pub mod base;
pub mod oauth;
pub mod pagination;

pub use base::BaseClient;
pub use oauth::OAuthClient;

use crate::{
    model::{idtypes::IdType, Id},
    ClientResult, Token,
};

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

#[inline]
pub(in crate) fn join_ids<'a, T: 'a + IdType>(ids: impl IntoIterator<Item = &'a Id<T>>) -> String {
    ids.into_iter().collect::<Vec<_>>().join(",")
}

/// Generates an HTTP token authorization header with proper formatting
pub fn bearer_auth(tok: &Token) -> (String, String) {
    let auth = "authorization".to_owned();
    let value = format!("Bearer {}", tok.access_token);

    (auth, value)
}

/// Generates an HTTP basic authorization header with proper formatting
pub fn basic_auth(user: &str, password: &str) -> (String, String) {
    let auth = "authorization".to_owned();
    let value = format!("{}:{}", user, password);
    let value = format!("Basic {}", base64::encode(value));

    (auth, value)
}

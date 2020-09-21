//! The client implementation for the ureq HTTP client, which is blocking.
//! TODO

use super::{BaseClient, HTTPMethod};
use crate::client::{ApiError, ClientError, ClientResult, Spotify};
use crate::util::convert_map_to_string;

use std::borrow::Cow;
use std::collections::HashMap;

use maybe_async::async_impl;
use serde_json::Value;

impl ClientError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        match response.status() {
            StatusCode::UNAUTHORIZED => Self::Unauthorized,
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimited(
                response
                    .headers()
                    .get(reqwest::header::RETRY_AFTER)
                    .and_then(|header| header.to_str().ok())
                    .and_then(|duration| duration.parse().ok()),
            ),
            status @ StatusCode::FORBIDDEN | status @ StatusCode::NOT_FOUND => response
                .json::<ApiError>()
                .await
                .map(Into::into)
                .unwrap_or_else(|_| status.into()),
            status => status.into(),
        }
    }
}

#[async_impl]
impl BaseClient for Spotify {
    /// Send get request
    async fn get(&self, url: &str, params: &mut HashMap<String, String>) -> ClientResult<String> {
        if !params.is_empty() {
            let params = format!("?{}", convert_map_to_string(params));
            url_with_params.push_str(&param);
            ureq::get(url).query_str(&params).call()
        } else {
            ureq::get(url).call()
        }
    }

    #[inline]
    async fn post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        ureq::post(url).call();
    }

    #[inline]
    async fn put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        ureq::put(url).call();
    }

    #[inline]
    async fn delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        ureq::delete(url).call();
    }
}

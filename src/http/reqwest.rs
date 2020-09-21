//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use super::BaseClient;
use crate::client::{ApiError, ClientError, ClientResult, Spotify};

use std::borrow::Cow;

use maybe_async::async_impl;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
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

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err.to_string())
    }
}

impl From<reqwest::StatusCode> for ClientError {
    fn from(code: reqwest::StatusCode) -> Self {
        Self::StatusCode(
            code.as_u16(),
            code.canonical_reason().unwrap_or("unknown").to_string(),
        )
    }
}

impl Spotify {
    async fn request(
        &self,
        method: Method,
        url: &str,
        payload: &Value,
    ) -> ClientResult<String> {
        // TODO: Cow<str> may not be necessary, and checking if it starts with
        // http shouldn't happen.
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.spotify.com/v1/", &url].concat().into();
        }

        let mut headers = HeaderMap::new();
        // TODO: these `unwrap` should be removed
        headers.insert(
            AUTHORIZATION,
            self.auth_headers()?
                .parse()
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = self
            .client
            .request(method, &url.into_owned())
            .headers(headers)
            .json(payload)
            .send()
            .await
            .map_err(ClientError::from)?;

        if response.status().is_success() {
            response.text().await.map_err(Into::into)
        } else {
            Err(ClientError::from_response(response).await)
        }
    }
}

#[async_impl]
impl BaseClient for Spotify {
    #[inline]
    async fn get(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(Method::GET, url, payload).await
    }

    #[inline]
    async fn post(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(Method::POST, url, payload).await
    }

    #[inline]
    async fn put(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(Method::PUT, url, payload).await
    }

    #[inline]
    async fn delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(Method::DELETE, url, payload).await
    }
}

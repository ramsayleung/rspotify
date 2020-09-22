//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use maybe_async::async_impl;
use reqwest::header::{self, HeaderMap};
use reqwest::{Method, StatusCode};
use serde_json::Value;

use std::convert::TryInto;

use super::{BaseClient, Headers};
use crate::client::{ApiError, ClientError, ClientResult, Spotify};

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
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        // Using the client's prefix in case it's a relative route.
        let url = if !url.starts_with("http") {
            self.prefix.clone() + &url
        } else {
            url.to_string()
        };

        // The default headers may be overriden for any value that isn't None.
        //
        // The values parsed for the headers values are of type `HeaderValue`,
        // which won't fail as long as its contents are ASCII.
        let headers = match headers {
            Some(headers) => headers.try_into().unwrap(),
            None => {
                let mut headers = HeaderMap::new();
                headers.insert(header::AUTHORIZATION, self.auth_headers()?.parse().unwrap());
                headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
                headers
            }
        };

        log::debug!(
            "Sending {:?} request to `{}` with headers `{:?}` and payload `{:?}`",
            method, url, headers, payload
        );

        let response = self
            .client
            .request(method, &url)
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
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::GET, url, headers, payload).await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::POST, url, headers, payload).await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::PUT, url, headers, payload).await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::DELETE, url, headers, payload).await
    }
}

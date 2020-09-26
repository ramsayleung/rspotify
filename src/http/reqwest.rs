//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use maybe_async::async_impl;
use reqwest::{Method, StatusCode};
use serde_json::Value;

use std::convert::TryInto;

use super::{headers, BaseClient, Content, FormData, Headers};
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
    async fn request<'a>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        payload: Option<Content<'a>>,
    ) -> ClientResult<String> {
        let url = self.endpoint_url(url);

        // The default auth headers are used if none were specified.
        let mut auth;
        let headers = match headers {
            Some(h) => h,
            None => {
                auth = Headers::new();
                let (key, val) = headers::bearer_auth(self.get_token()?);
                auth.insert(key, val);
                &auth
            }
        };
        // The headers need to be converted into a `reqwest::HeaderMap`, which
        // won't fail as long as its contents are ASCII. This is an internal
        // function, so the condition will always be true.
        //
        // The content-type header will be set automatically.
        let headers = headers.try_into().unwrap();

        let mut request = self.client.request(method.clone(), &url).headers(headers);

        log::info!("Making request {:?} with payload {:?}", request, payload);

        if let Some(payload) = payload {
            request = match payload {
                Content::Json(payload) => match method {
                    Method::GET => request.query(payload),
                    Method::POST | Method::PUT | Method::DELETE => request.json(payload),
                    // Method: Options, Head, Trace haven't implemented in `rspotify` yet, just leave it alone.
                    _ => request,
                },
                Content::Form(payload) => {
                    // `Request::form` won't work for `GET` requests
                    assert!(method != Method::GET);
                    request.form(payload)
                }
            };
        }

        let response = request.send().await.map_err(ClientError::from)?;

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
        payload: Option<&Value>,
    ) -> ClientResult<String> {
        self.request(
            Method::GET,
            url,
            headers,
            payload.map(|x| Content::Json(x)),
        )
        .await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::POST, url, headers, Some(Content::Json(payload)))
            .await
    }

    #[inline]
    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &FormData,
    ) -> ClientResult<String> {
        self.request(Method::POST, url, headers, Some(Content::Form(payload)))
            .await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::PUT, url, headers, Some(Content::Json(payload)))
            .await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(Method::DELETE, url, headers, Some(Content::Json(payload)))
            .await
    }
}

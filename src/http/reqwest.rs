use super::{BaseClient, HTTPMethod};
use crate::client::{ApiError, ClientError, ClientResult, Spotify};
use crate::util::convert_map_to_string;

use std::borrow::Cow;
use std::collections::HashMap;

use maybe_async::async_impl;
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Method, StatusCode};
use serde_json::Value;

impl From<HTTPMethod> for Method {
    fn from(method: HTTPMethod) -> Method {
        match method {
            HTTPMethod::GET => Method::GET,
            HTTPMethod::POST => Method::POST,
            HTTPMethod::PUT => Method::PUT,
            HTTPMethod::DELETE => Method::DELETE,
        }
    }
}

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

impl From<reqwest::StatusCode> for ClientError {
    fn from(code: reqwest::StatusCode) -> Self {
        Self::StatusCode(code)
    }
}

#[async_impl]
impl BaseClient for Spotify {
    async fn request(
        &self,
        method: HTTPMethod,
        url: &str,
        payload: Option<&Value>,
    ) -> ClientResult<String> {
        // This should be improved: Cow<str> may not be necessary, and checking
        // if it starts with http shouldn't happen.
        let mut url: Cow<str> = url.into();
        if !url.starts_with("http") {
            url = ["https://api.spotify.com/v1/", &url].concat().into();
        }

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, self.auth_headers().await.parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        let response = {
            let builder = self
                .client
                .request(method.into(), &url.into_owned())
                .headers(headers);

            if let Some(json) = payload {
                builder = builder.json(json)
            }

            builder.send().await.map_err(ClientError::from)?
        };

        if response.status().is_success() {
            response.text().await.map_err(Into::into)
        } else {
            Err(ClientError::from_response(response).await)
        }
    }

    /// Send get request
    async fn get(&self, url: &str, params: &mut HashMap<String, String>) -> ClientResult<String> {
        if !params.is_empty() {
            let param: String = convert_map_to_string(params);
            let mut url_with_params = url.to_owned();
            url_with_params.push('?');
            url_with_params.push_str(&param);
            self.request(HTTPMethod::GET, &url_with_params, None).await
        } else {
            self.request(HTTPMethod::GET, url, None).await
        }
    }

    /// Send post request
    async fn post(&self, url: &str, payload: Option<&Value>) -> ClientResult<String> {
        self.request(HTTPMethod::POST, url, Some(payload)).await
    }

    /// Send put request
    async fn put(&self, url: &str, payload: Option<&Value>) -> ClientResult<String> {
        self.request(HTTPMethod::PUT, url, Some(payload)).await
    }

    /// send delete request
    async fn delete(&self, url: &str, payload: &Value) -> ClientResult<String> {
        self.request(HTTPMethod::DELETE, url, Some(payload)).await
    }
}

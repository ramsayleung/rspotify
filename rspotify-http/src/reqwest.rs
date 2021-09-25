//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use super::{BaseHttpClient, Form, Headers, HttpError, HttpResult, Query};

use std::convert::TryInto;

use maybe_async::async_impl;
use reqwest::{Method, RequestBuilder};
use serde_json::Value;

/// Custom enum that contains all the possible errors that may occur when using
/// `reqwest`.
#[derive(thiserror::Error, Debug)]
pub enum ReqwestError {
    #[error("request: {0}")]
    Client(#[from] reqwest::Error),

    #[error("status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

#[derive(Default, Debug, Clone)]
pub struct ReqwestClient {
    /// reqwest needs an instance of its client to perform requests.
    client: reqwest::Client,
}

impl ReqwestClient {
    async fn request<D>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        add_data: D,
    ) -> HttpResult<String>
    where
        D: Fn(RequestBuilder) -> RequestBuilder,
    {
        let mut request = self.client.request(method.clone(), url);

        // Setting the headers, if any
        if let Some(headers) = headers {
            // The headers need to be converted into a `reqwest::HeaderMap`,
            // which won't fail as long as its contents are ASCII. This is an
            // internal function, so the condition cannot be broken by the user
            // and will always be true.
            //
            // The content-type header will be set automatically.
            let headers = headers.try_into().unwrap();

            request = request.headers(headers);
        }

        // Configuring the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Finally performing the request and handling the response
        log::info!("Making request {:?}", request);
        let response = request
            .send()
            .await
            .map_err(|error| HttpError::Reqwest(Error::Client(error)))?;

        // Making sure that the status code is OK
        if !response.status().is_success() {
            return Err(HttpError::Reqwest(Error::StatusCode(response)));
        }

        response
            .text()
            .await
            .map_err(|error| HttpError::Reqwest(Error::Client(error)))
    }
}

#[async_impl]
impl BaseHttpClient for ReqwestClient {
    #[inline]
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> HttpResult<String> {
        self.request(Method::GET, url, headers, |req| req.query(payload))
            .await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String> {
        self.request(Method::POST, url, headers, |req| req.json(payload))
            .await
    }

    #[inline]
    async fn post_form<'a>(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'a>,
    ) -> HttpResult<String> {
        self.request(Method::POST, url, headers, |req| req.form(payload))
            .await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String> {
        self.request(Method::PUT, url, headers, |req| req.json(payload))
            .await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> HttpResult<String> {
        self.request(Method::DELETE, url, headers, |req| req.json(payload))
            .await
    }
}

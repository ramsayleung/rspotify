//! The client implementation for the ureq HTTP client, which is blocking.

use super::{BaseHttpClient, Form, Headers, Query};

use std::{io, time::Duration};

use maybe_async::sync_impl;
use serde_json::Value;
use ureq::{Request, Response};

/// Custom enum that contains all the possible errors that may occur when using
/// `ureq`.
///
/// Sample usage:
///
/// ```
/// use rspotify_http::{HttpError, HttpClient, BaseHttpClient};
///
/// let client = HttpClient::default();
/// let response = client.get("wrongurl", None, &Default::default());
/// match response {
///     Ok(data) => println!("request succeeded: {:?}", data),
///     Err(HttpError::Transport(e)) => eprintln!("request failed: {}", e),
///     Err(HttpError::Io(e)) => eprintln!("failed to decode response: {}", e),
///     Err(HttpError::StatusCode(response)) => {
///         let code = response.status();
///         match response.into_json::<rspotify_model::ApiError>() {
///             Ok(api_error) => eprintln!("status code {}: {:?}", code, api_error),
///             Err(_) => eprintln!("status code {}", code),
///         }
///     },
/// }
/// ```
#[derive(thiserror::Error, Debug)]
pub enum UreqError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("transport: {0}")]
    Transport(#[from] ureq::Transport),

    /// There was an error when trying to decode the response
    #[error("I/O: {0}")]
    Io(#[from] io::Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Spotify with more information, which can be
    /// serialized into `rspotify_model::ApiError`.
    #[error("status code {}", ureq::Response::status(.0))]
    StatusCode(ureq::Response),
}

#[derive(Debug, Clone)]
pub struct UreqClient {
    agent: ureq::Agent,
}

impl Default for UreqClient {
    fn default() -> Self {
        let agent = ureq::AgentBuilder::new()
            .try_proxy_from_env(true)
            .timeout(Duration::from_secs(10))
            .build();
        Self { agent }
    }
}

impl UreqClient {
    /// The request handling in ureq is split in three parts:
    ///
    /// * The initial request (POST, GET, ...) is given as the `request`
    ///   parameter.
    /// * This method will add whichever headers and additional data is needed
    ///   for all requests.
    /// * The request is finished and performed with the `send_request` function
    ///   (JSON, a form...).
    fn request<D>(
        &self,
        mut request: Request,
        headers: Option<&Headers>,
        send_request: D,
    ) -> Result<String, UreqError>
    where
        D: Fn(Request) -> Result<Response, ureq::Error>,
    {
        // Setting the headers, which will be the token auth if unspecified.
        if let Some(headers) = headers {
            for (key, val) in headers.iter() {
                request = request.set(key, val);
            }
        }

        log::info!("Making request {:?}", request);
        // Converting errors from ureq into our custom error types
        match send_request(request) {
            Ok(response) => response.into_string().map_err(Into::into),
            Err(err) => match err {
                ureq::Error::Status(_, response) => Err(UreqError::StatusCode(response)),
                ureq::Error::Transport(transport) => Err(UreqError::Transport(transport)),
            },
        }
    }
}

#[sync_impl]
impl BaseHttpClient for UreqClient {
    type Error = UreqError;

    #[inline]
    fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        let request = self.agent.get(url);
        let sender = |mut req: Request| {
            for (key, val) in payload.iter() {
                req = req.query(key, val);
            }
            req.call()
        };
        self.request(request, headers, sender)
    }

    #[inline]
    fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        let request = self.agent.post(url);
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }

    #[inline]
    fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'_>,
    ) -> Result<String, Self::Error> {
        let request = self.agent.post(url);
        let sender = |req: Request| {
            let payload = payload
                .iter()
                .map(|(key, val)| (*key, *val))
                .collect::<Vec<_>>();

            req.send_form(&payload)
        };

        self.request(request, headers, sender)
    }

    #[inline]
    fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        let request = self.agent.put(url);
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }

    #[inline]
    fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        let request = self.agent.delete(url);
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }
}

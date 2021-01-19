//! The client implementation for the ureq HTTP client, which is blocking.

use super::{headers, BaseClient, Form, Headers, Query};
use crate::client::{ClientError, ClientResult, Spotify};

use maybe_async::sync_impl;
use serde_json::Value;
use ureq::{Request, Response};

impl ClientError {
    pub fn from_response(r: ureq::Response) -> Self {
        ClientError::StatusCode(r.status(), r.status_text().to_string())
    }
}

impl Spotify {
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
    ) -> ClientResult<String>
    where
        D: Fn(Request) -> Result<Response, ureq::Error>,
    {
        // Setting the headers, which will be the token auth if unspecified.
        match headers {
            Some(headers) => {
                for (key, val) in headers.iter() {
                    request = request.set(&key, &val);
                }
            }
            None => {
                let (key, val) = headers::bearer_auth(self.get_token()?);
                request = request.set(&key, &val);
            }
        }

        log::info!("Making request {:?}", request);
        match send_request(request) {
            Ok(response) => response.into_string().map_err(Into::into),
            Err(ureq::Error::Status(_, response)) => Err(ClientError::from_response(response)),
            Err(err) => {
                // Some kind of IO/transport error
                Err(ClientError::Request(err.to_string()))
            }
        }
    }
}

#[sync_impl]
impl BaseClient for Spotify {
    #[inline]
    fn get(&self, url: &str, headers: Option<&Headers>, payload: &Query) -> ClientResult<String> {
        let request = ureq::get(&self.endpoint_url(url));
        let sender = |mut req: Request| {
            for (key, val) in payload.iter() {
                req = req.query(&key, &val)
            }
            req.call()
        };
        self.request(request, headers, sender)
    }

    #[inline]
    fn post(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> ClientResult<String> {
        let request = ureq::post(&self.endpoint_url(url));
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }

    #[inline]
    fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form,
    ) -> ClientResult<String> {
        let request = ureq::post(&self.endpoint_url(url));
        let sender = |req: Request| {
            let payload = payload
                .iter()
                .map(|(key, val)| (key.as_str(), val.as_str()))
                .collect::<Vec<_>>();

            req.send_form(&payload)
        };

        self.request(request, headers, sender)
    }

    #[inline]
    fn put(&self, url: &str, headers: Option<&Headers>, payload: &Value) -> ClientResult<String> {
        let request = ureq::put(&self.endpoint_url(url));
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }

    #[inline]
    fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        let request = ureq::delete(&self.endpoint_url(url));
        let sender = |req: Request| req.send_json(payload.clone());
        self.request(request, headers, sender)
    }
}

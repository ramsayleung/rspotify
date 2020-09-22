//! The client implementation for the ureq HTTP client, which is blocking.
//! TODO

use super::{BaseClient, Content, headers, Headers, FormData};
use crate::client::{ClientError, ClientResult, Spotify};

use maybe_async::sync_impl;
use serde_json::Value;

impl ClientError {
    pub fn from_response(r: ureq::Response) -> Self {
        ClientError::StatusCode(r.status(), r.status_text().to_string())
    }
}

fn convert_to_ureq_form<'a>(data: &'a FormData) -> Vec<(&'a str, &'a str)> {
    data.iter()
        .map(|(key, val)| (key.as_str(), val.as_str()))
        .collect()
}

impl Spotify {
    fn request<'a>(
        &self,
        req: &mut ureq::Request,
        headers: Option<&Headers>,
        payload: Content<'a>
    ) -> ClientResult<String> {
        // Setting the headers, which will be the token auth if unspecified.
        match headers {
            Some(headers) => {
                for (key, val) in headers.iter() {
                    req.set(&key, &val);
                }
            },
            None => {
                let (key, val) = headers::bearer_auth(self.get_token()?);
                req.set(&key, &val);
            }
        }

        log::info!("Making request {:?} with payload {:?}", req, payload);

        // TODO: maybe it'd be better to take ownership of the content to
        // avoid this clone.
        let response = match payload {
            Content::Json(value) => req.send_json(value.clone()),
            Content::Form(value) => req.send_form(&convert_to_ureq_form(value)),
        };

        if response.ok() {
            response.into_string().map_err(Into::into)
        } else {
            Err(ClientError::from_response(response))
        }
    }
}

#[sync_impl]
impl BaseClient for Spotify {
    #[inline]
    fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(&mut ureq::get(&self.endpoint_url(url)), headers, Content::Json(payload))
    }

    #[inline]
    fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(&mut ureq::post(&self.endpoint_url(url)), headers, Content::Json(payload))
    }

    #[inline]
    fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &FormData,
    ) -> ClientResult<String> {
        self.request(&mut ureq::post(&self.endpoint_url(url)), headers, Content::Form(payload))
    }

    #[inline]
    fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(&mut ureq::put(&self.endpoint_url(url)), headers, Content::Json(payload))
    }

    #[inline]
    fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> ClientResult<String> {
        self.request(&mut ureq::delete(&self.endpoint_url(url)), headers, Content::Json(payload))
    }
}

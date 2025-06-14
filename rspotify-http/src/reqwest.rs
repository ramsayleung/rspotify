//! The client implementation for the reqwest HTTP client, which is async by
//! default.

use super::{BaseHttpClient, Form, Headers, Query};

use std::convert::TryInto;

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

use maybe_async::async_impl;
use reqwest::Method;
use serde_json::Value;

#[cfg(not(feature = "reqwest-middleware"))]
use reqwest::{Client, Error, RequestBuilder};

/// Custom enum that contains all the possible errors that may occur when using
/// [`reqwest`].
///
/// Sample usage:
///
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use rspotify_http::{HttpError, HttpClient, BaseHttpClient};
///
/// let client = HttpClient::default();
/// let response = client.get("wrongurl", None, &Default::default()).await;
/// match response {
///     Ok(data) => println!("request succeeded: {:?}", data),
///     Err(HttpError::Client(e)) => eprintln!("request failed: {}", e),
///     Err(HttpError::StatusCode(response)) => {
///         let code = response.status().as_u16();
///         match response.json::<rspotify_model::ApiError>().await {
///             Ok(api_error) => eprintln!("status code {}: {:?}", code, api_error),
///             Err(_) => eprintln!("status code {}", code),
///         }
///     },
/// }
/// # }
/// ```
#[derive(thiserror::Error, Debug)]
pub enum ReqwestError {
    /// The request couldn't be completed because there was an error when trying
    /// to do so
    #[error("request: {0}")]
    Client(#[from] Error),

    /// The request was made, but the server returned an unsuccessful status
    /// code, such as 404 or 503. In some cases, the response may contain a
    /// custom message from Spotify with more information, which can be
    /// serialized into `rspotify_model::ApiError`.
    #[error("status code {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    /// reqwest needs an instance of its client to perform requests.
    client: Client,
}

#[cfg(not(target_arch = "wasm32"))]
fn default_reqwest_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()
        // building with these options cannot fail
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn default_reqwest_client() -> reqwest::Client {
    reqwest::ClientBuilder::new()
        .build()
        // building with these options cannot fail
        .unwrap()
}

#[cfg(not(feature = "reqwest-middleware"))]
impl Default for ReqwestClient {
    fn default() -> Self {
        Self {
            client: default_reqwest_client(),
        }
    }
}

impl ReqwestClient {
    async fn request<D>(
        &self,
        method: Method,
        url: &str,
        headers: Option<&Headers>,
        add_data: D,
    ) -> Result<String, ReqwestError>
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
        let response = request.send().await?;

        // Making sure that the status code is OK
        if response.status().is_success() {
            #[cfg_attr(not(feature = "reqwest-middleware"), allow(clippy::useless_conversion))]
            Ok(response.text().await.map_err(Error::from)?)
        } else {
            Err(ReqwestError::StatusCode(response))
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_impl(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_impl)]
impl BaseHttpClient for ReqwestClient {
    type Error = ReqwestError;

    #[inline]
    async fn get(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Query,
    ) -> Result<String, Self::Error> {
        self.request(Method::GET, url, headers, |req| req.query(payload))
            .await
    }

    #[inline]
    async fn post(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| req.json(payload))
            .await
    }

    #[inline]
    async fn post_form(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Form<'_>,
    ) -> Result<String, Self::Error> {
        self.request(Method::POST, url, headers, |req| req.form(payload))
            .await
    }

    #[inline]
    async fn put(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::PUT, url, headers, |req| req.json(payload))
            .await
    }

    #[inline]
    async fn delete(
        &self,
        url: &str,
        headers: Option<&Headers>,
        payload: &Value,
    ) -> Result<String, Self::Error> {
        self.request(Method::DELETE, url, headers, |req| req.json(payload))
            .await
    }
}

#[cfg(feature = "reqwest-middleware")]
use middleware::*;
#[cfg(feature = "reqwest-middleware")]
pub use middleware::{Middleware, ReqwestClientBuilder};
#[cfg(feature = "reqwest-middleware")]
mod middleware {
    use std::sync::Arc;

    pub use reqwest_middleware::{
        ClientWithMiddleware as Client, Error, Middleware, RequestBuilder,
    };

    use super::{default_reqwest_client, ReqwestClient};
    use reqwest_middleware::ClientBuilder;

    impl Default for ReqwestClient {
        fn default() -> Self {
            let reqwest_client = default_reqwest_client();
            let client = ClientBuilder::new(reqwest_client).build();
            Self { client }
        }
    }

    pub struct ReqwestClientBuilder {
        builder: ClientBuilder,
    }

    impl Default for ReqwestClientBuilder {
        fn default() -> Self {
            let builder = ClientBuilder::new(default_reqwest_client());
            Self { builder }
        }
    }

    impl ReqwestClientBuilder {
        pub fn with<M: Middleware>(self, middleware: M) -> Self {
            Self {
                builder: self.builder.with(middleware),
            }
        }

        pub fn with_arc(self, middleware: Arc<dyn Middleware>) -> Self {
            Self {
                builder: self.builder.with_arc(middleware),
            }
        }

        pub fn build(self) -> ReqwestClient {
            ReqwestClient {
                client: self.builder.build(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests for reqwest with `reqwest-middleware` enabled
    #[cfg(feature = "reqwest-middleware")]
    mod middleware {
        use super::super::*;
        use reqwest::{Request, Response};
        use reqwest_middleware::Next;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        /// Example middleware that flips `has_run` to `true` after it's handled a request.
        #[derive(Default)]
        pub struct TestMiddleware {
            // this defaults to false
            has_run: AtomicBool,
        }

        impl TestMiddleware {
            fn get_has_run(&self) -> bool {
                self.has_run.load(Ordering::Relaxed)
            }

            // called by the `Middleware` implementation when handling a request.
            fn set_has_run(&self) {
                self.has_run.store(true, Ordering::Relaxed);
            }
        }

        #[async_trait::async_trait]
        impl Middleware for TestMiddleware {
            async fn handle(
                &self,
                req: Request,
                extensions: &mut http::Extensions,
                next: Next<'_>,
            ) -> Result<Response, Error> {
                // sets `has_run` to `true` indicating we've handled a request.
                self.set_has_run();
                next.run(req, extensions).await
            }
        }

        #[tokio::test]
        pub async fn test_reqwest_middleware_client() {
            // Client that should run our middleware when handling requests
            let middleware = Arc::new(TestMiddleware::default());
            let client = ReqwestClientBuilder::default()
                .with_arc(middleware.clone())
                .build();

            // Setup mock server to handle the request
            let mock_server = wiremock::MockServer::start().await;
            let mock = wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/"))
                .respond_with(wiremock::ResponseTemplate::new(200));
            mock_server.register(mock).await;

            // Verify middleware hasn't been run yet (has_run is false before the request)
            assert!(!middleware.get_has_run());

            // Make request, causing the middleware to run
            let url = &mock_server.uri();
            client.get(url, None, &Query::new()).await.unwrap();

            // Verify middleware has run (has_run should have flipped to true)
            assert!(middleware.get_has_run());
        }
    }
}

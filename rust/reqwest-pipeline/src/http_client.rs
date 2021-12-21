use crate::ReqwestPipelineError;
use crate::{Body, Request, Response};
use async_trait::async_trait;
use bytes::Bytes;
use futures::TryStreamExt;
use serde::Serialize;
use std::sync::Arc;

/// Construct a new HTTP client with the `reqwest` backend.
pub fn new_http_client() -> Arc<dyn HttpClient> {
    Arc::new(reqwest::Client::new())
}

/// An HTTP client which can send requests.
#[async_trait]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    /// Send out a request using `azure_core`'s types.
    ///
    /// This function will be the only one remaining in the trait as soon as the trait stabilizes.
    /// It will be renamed to `execute_request`. The other helper functions (ie
    /// `execute_request_check_status`) will be removed since the status check will be
    /// responsibility of another policy (not the transport one). It does not consume the request.
    /// Implementors are expected to clone the necessary parts of the request and pass them to the
    /// underlying transport.
    async fn execute_request(&self, request: &Request) -> Result<Response, ReqwestPipelineError>;

    async fn execute_request_raw(
        &self,
        request: http::Request<Bytes>,
    ) -> Result<http::Response<Bytes>, ReqwestPipelineError>;
}

#[async_trait]
impl HttpClient for reqwest::Client {
    async fn execute_request_raw(
        &self,
        request: http::Request<Bytes>,
    ) -> Result<http::Response<Bytes>, ReqwestPipelineError> {
        let mut reqwest_request = self.request(
            request.method().clone(),
            url::Url::parse(&request.uri().to_string()).unwrap(),
        );
        for (header, value) in request.headers() {
            reqwest_request = reqwest_request.header(header, value);
        }

        let reqwest_request = reqwest_request.body(request.into_body()).build()?;

        let reqwest_response = self.execute(reqwest_request).await?;

        let mut response = http::Response::builder().status(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response = response.header(key, value);
        }

        let response = response.body(reqwest_response.bytes().await?)?;

        Ok(response)
    }

    async fn execute_request(&self, request: &Request) -> Result<Response, ReqwestPipelineError> {
        let mut reqwest_request = self.request(
            request.method(),
            url::Url::parse(&request.uri().to_string()).unwrap(),
        );
        for header in request.headers() {
            reqwest_request = reqwest_request.header(header.0, header.1);
        }

        // We clone the body since we need to give ownership of it to Reqwest.
        let body = request.body().clone();

        let reqwest_request = match body {
            Body::Bytes(bytes) => reqwest_request.body(bytes).build()?,
            Body::SeekableStream(mut seekable_stream) => {
                seekable_stream.reset().await?;

                reqwest_request
                    .body(reqwest::Body::wrap_stream(seekable_stream))
                    .build()?
            }
        };

        let reqwest_response = self.execute(reqwest_request).await?;
        let mut response = crate::ResponseBuilder::new(reqwest_response.status());

        for (key, value) in reqwest_response.headers() {
            response.with_header(key, value.clone());
        }

        let response = response.with_pinned_stream(Box::pin(
            reqwest_response
                .bytes_stream()
                .map_err(crate::StreamError::ReadError),
        ));

        Ok(response)
    }
}

/// Serialize a type to json.
pub fn to_json<T>(value: &T) -> Result<Bytes, serde_json::Error>
where
    T: ?Sized + Serialize,
{
    Ok(Bytes::from(serde_json::to_vec(value)?))
}

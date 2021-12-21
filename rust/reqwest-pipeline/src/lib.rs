extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url;

#[macro_use]
mod macros;
pub mod bytes_stream;
pub mod context;
pub mod http_client;
#[cfg(feature = "mock_transport_framework")]
pub mod mock;
pub mod options;
mod pageable;
pub mod pipeline;
pub mod policies;
pub mod prelude;
pub mod request;
pub mod response;
pub mod seekable_stream;
mod sleep;

pub use bytes_stream::*;
pub use context::*;
pub use http_client::*;
pub use options::*;
pub use pageable::*;
pub use pipeline::*;
pub use policies::*;
pub use request::*;
pub use response::*;
pub use seekable_stream::*;

use http::StatusCode;

pub type Result<T> = std::result::Result<T, ReqwestPipelineError>;

/// An empty HTTP body.
#[allow(clippy::declare_interior_mutable_const)]
pub const EMPTY_BODY: bytes::Bytes = bytes::Bytes::from_static(&[]);

/// Add a new query pair into the target URL's query string.
pub trait AppendToUrlQuery {
    fn append_to_url_query(&self, url: &mut url::Url);
}

impl<T> AppendToUrlQuery for Option<T>
where
    T: AppendToUrlQuery,
{
    fn append_to_url_query(&self, url: &mut url::Url) {
        if let Some(i) = self {
            i.append_to_url_query(url);
        }
    }
}

/// View a type as an HTTP header.
///
/// While not restricted by the type system, please add HTTP headers only. In particular, do not
/// interact with the body of the request.
pub trait AddAsHeader {
    fn add_as_header(
        &self,
        request: &mut crate::Request,
    ) -> std::result::Result<(), HTTPHeaderError>;
}

pub fn add_optional_header<T: AddAsHeader>(
    item: &Option<T>,
    request: &mut crate::Request,
) -> std::result::Result<(), HTTPHeaderError> {
    if let Some(item) = item {
        item.add_as_header(request)?
    }
    Ok(())
}

pub fn add_mandatory_header<T: AddAsHeader>(
    item: &T,
    request: &mut crate::Request,
) -> std::result::Result<(), HTTPHeaderError> {
    item.add_as_header(request)
}

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum ReqwestPipelineError {
    /// Error returned when the table to be created already exists
    #[error("Error parsing url: {source}")]
    UrlParseError {
        #[from]
        source: url::ParseError,
    },

    #[error("Error parsing url: {source}")]
    HttpError {
        #[from]
        source: http::Error,
    },

    #[error("Error in transport: {source}")]
    TransportError {
        #[from]
        source: reqwest::Error,
    },

    #[error("Error streaming data: {source}")]
    StreamError {
        #[from]
        source: StreamError,
    },

    #[error("HTTP error status (status: {:?}, body: {:?})", status, body)]
    ErrorStatusCode { status: StatusCode, body: String },

    #[error("pipeline error: {0}")]
    PipelineError(#[from] PipelineError),

    #[error("HTTP header error: {0}")]
    HTTPHeaderError(#[from] HTTPHeaderError),

    #[error("policy error: {0}")]
    PolicyError(Box<dyn std::error::Error + Send + Sync>),

    #[error("json error: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// An error originating from a streaming response.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum StreamError {
    #[error("error polling stream: {0}")]
    PollError(std::io::Error),
    #[error("error reading stream: {0}")]
    ReadError(reqwest::Error),
}

/// An error originating from a pipeline.
#[derive(Debug, thiserror::Error)]
pub enum PipelineError {
    #[error("invalid pipeline: last policy is not a TransportPolicy: {0:?}")]
    InvalidTailPolicy(String),
}

/// An error caused by an HTTP header.
#[derive(Debug, thiserror::Error)]
pub enum HTTPHeaderError {
    #[error("{0}")]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),
    #[error("{0}")]
    InvalidHeaderName(#[from] http::header::InvalidHeaderName),
}

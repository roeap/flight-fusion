extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url;

#[macro_use]
mod macros;
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

    #[error("policy error: {0}")]
    PolicyError(Box<dyn std::error::Error + Send + Sync>),
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

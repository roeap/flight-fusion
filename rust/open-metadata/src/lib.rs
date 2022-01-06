#[macro_use]
extern crate serde_derive;

pub mod clients;
pub mod generated;
mod operations;
pub mod prelude;
mod request_options;

use http::StatusCode;

type Result<T> = std::result::Result<T, OpenMetadataError>;

pub enum EntityIdentifier {
    /// The unique entity ID
    ID(String),
    /// The fully qualified name for the entity
    FQN(String),
}

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum OpenMetadataError {
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
        source: reqwest_pipeline::ReqwestPipelineError,
    },

    #[error("HTTP error status (status: {:?}, body: {:?})", status, body)]
    ErrorStatusCode { status: StatusCode, body: String },

    #[error("Error processing json: {source}")]
    JsonError {
        #[from]
        source: serde_json::Error,
    },
}

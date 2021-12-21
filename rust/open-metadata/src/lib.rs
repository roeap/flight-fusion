#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod clients;

use http::StatusCode;

type Result<T> = std::result::Result<T, OpenMetadataError>;

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
        source: reqwest::Error,
    },

    #[error("HTTP error status (status: {:?}, body: {:?})", status, body)]
    ErrorStatusCode { status: StatusCode, body: String },
}

use arrow::error::ArrowError;

#[derive(thiserror::Error, Debug)]
pub enum FusionClientError {
    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Errors during communication with flight server
    #[error("Table: already exists")]
    TransportError {
        #[from]
        source: tonic::transport::Error,
    },

    /// Errors during communication with flight server
    #[error("Unexpected return status: {source}")]
    ReturnCodeError {
        #[from]
        source: tonic::Status,
    },

    #[error("Unexpected return status: {source}")]
    CorruptReturnMessage {
        #[from]
        source: prost::DecodeError,
    },

    #[error(transparent)]
    ArrowError(#[from] ArrowError),

    #[error(transparent)]
    InvalidUri(#[from] http::uri::InvalidUri),
}

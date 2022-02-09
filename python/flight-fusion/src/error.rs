use flight_fusion_client::error::FusionClientError as InnerFusionClientError;
use pyo3::{exceptions::PyException, PyErr};

#[derive(thiserror::Error, Debug)]
pub enum FlightFusionClientError {
    /// Error returned
    #[error(transparent)]
    ExecutionError(#[from] InnerFusionClientError),

    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Errors during communication with flight server
    #[error("Io Error: {source}")]
    IoError {
        #[from]
        source: tokio::io::Error,
    },

    /// Error decoding server return messages
    #[error("Failed reading server response: {source}")]
    CorruptReturnMessage {
        #[from]
        source: prost::DecodeError,
    },

    /// Error encoding response
    #[error("Failed encoding response: {source}")]
    EncodeError {
        #[from]
        source: prost::EncodeError,
    },
}

impl From<FlightFusionClientError> for PyErr {
    fn from(err: FlightFusionClientError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

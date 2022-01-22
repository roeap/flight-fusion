use flight_fusion::error::FusionServiceError as InnerFusionServiceError;
use pyo3::{exceptions::PyException, PyErr};

#[derive(thiserror::Error, Debug)]
pub enum FlightFusionServerError {
    /// Error returned
    #[error(transparent)]
    ExecutionError(#[from] InnerFusionServiceError),

    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Errors during communication with flight server
    #[error("Table: already exists")]
    IoError {
        #[from]
        source: tokio::io::Error,
    },
}

impl From<FlightFusionServerError> for PyErr {
    fn from(err: FlightFusionServerError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

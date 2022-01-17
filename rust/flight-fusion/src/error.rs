use flight_fusion_ipc::FlightFusionError;
use futures::Stream;
use std::pin::Pin;

#[derive(thiserror::Error, Debug)]
pub enum FusionServiceError {
    /// Error returned when an input is in an unexpected format or contains invalid data
    #[error("Malformed input {0}")]
    InputError(String),

    /// Error returned when a table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    #[error(transparent)]
    StorageError(#[from] crate::area_store::AreaStoreError),
}

/// Result type for fallible operations defined in this crate
pub type Result<T> = std::result::Result<T, FusionServiceError>;

/// Result type for fallible streaming operations defined in this crate
pub type ResultStream<T> = Result<Pin<Box<dyn Stream<Item = Result<T>> + Send + Sync + 'static>>>;

pub fn to_fusion_err(e: impl std::error::Error) -> FlightFusionError {
    FlightFusionError::ExternalError(e.to_string())
}

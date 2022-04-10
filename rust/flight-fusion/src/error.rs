use area_store::{catalog::error::AreaCatalogError, store::error::AreaStoreError};
use arrow_deps::{
    datafusion::error::DataFusionError,
    deltalake::{operations::DeltaCommandError, DeltaTableError},
};
use flight_fusion_ipc::FlightFusionIpcError;
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

    /// Error returned when trying to execute an unknown flight action
    #[error("No actions available for {0}")]
    UnknownAction(String),

    /// Error returned when no return data is generated
    #[error("Empty dataset evaluation query {0}")]
    NoReturnData(String),

    /// Error returned when when no more specific error is defined
    #[error("Generic error: {0}")]
    Generic(String),

    #[error(transparent)]
    StorageError(#[from] AreaStoreError),

    #[error(transparent)]
    CatalogError(#[from] AreaCatalogError),

    #[error(transparent)]
    QueryError(#[from] DataFusionError),

    #[error(transparent)]
    TransportError(#[from] tonic::Status),

    #[error(transparent)]
    IpcError(#[from] FlightFusionIpcError),

    #[error(transparent)]
    DeltaCommand(#[from] DeltaCommandError),

    #[error(transparent)]
    DeltaTable(#[from] DeltaTableError),
}

impl FusionServiceError {
    pub fn generic(message: impl Into<String>) -> FusionServiceError {
        let msg: String = message.into();
        FusionServiceError::Generic(msg)
    }

    pub fn input(message: impl Into<String>) -> FusionServiceError {
        let msg: String = message.into();
        FusionServiceError::InputError(msg)
    }

    pub fn unknown_action(message: impl Into<String>) -> FusionServiceError {
        let msg: String = message.into();
        FusionServiceError::UnknownAction(msg)
    }
}

/// Result type for fallible operations defined in this crate
pub type Result<T> = std::result::Result<T, FusionServiceError>;

/// Result type for fallible streaming operations defined in this crate
pub type ResultStream<T> = Result<Pin<Box<dyn Stream<Item = Result<T>> + Send + Sync + 'static>>>;

#[derive(thiserror::Error, Debug)]
pub enum FlightFusionError {
    /// Error returned when the table to be created already exists
    #[error("No actions available for {0}")]
    UnknownAction(String),

    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Errors during communication with flight server
    #[error("Table: already exists")]
    TransportError {
        #[from]
        source: tonic::transport::Error
    },
}

pub type Result<T> = std::result::Result<T, FlightFusionError>;

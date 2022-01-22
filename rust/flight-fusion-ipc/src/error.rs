#[derive(thiserror::Error, Debug)]
pub enum FlightFusionError {
    /// Error returned when the table to be created already exists
    #[error("No actions available for {0}")]
    UnknownAction(String),

    /// Error returned when the table to be created already exists
    #[error("Error in dependent crate {0}")]
    ExternalError(String),

    /// Error returned when the table to be created already exists
    #[error("Empty dataset evaluation query {0}")]
    NoReturnData(String),

    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Error returned when an input is in an unexpected format
    #[error("Malformed input {0}")]
    InputError(String),

    /// Errors during communication with flight server
    #[error("Table: already exists")]
    TransportError {
        #[from]
        source: tonic::transport::Error,
    },

    /// Error returned when encoding messages
    #[error(transparent)]
    EncodeError(#[from] prost::EncodeError),

    /// Error returned when encoding messages
    #[error(transparent)]
    DecodeError(#[from] prost::DecodeError),

    /// Error returned when the table to be created already exists
    #[error("Generic error: {0}")]
    Generic(String),
}

impl FlightFusionError {
    pub fn generic(message: impl Into<String>) -> FlightFusionError {
        let msg: String = message.into();
        FlightFusionError::Generic(msg)
    }

    pub fn external(message: impl Into<String>) -> FlightFusionError {
        let msg: String = message.into();
        FlightFusionError::ExternalError(msg)
    }

    pub fn input(message: impl Into<String>) -> FlightFusionError {
        let msg: String = message.into();
        FlightFusionError::InputError(msg)
    }
}

pub type Result<T> = std::result::Result<T, FlightFusionError>;

pub fn to_flight_fusion_err(e: impl std::error::Error) -> FlightFusionError {
    FlightFusionError::ExternalError(e.to_string())
}

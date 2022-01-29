#[derive(thiserror::Error, Debug)]
pub enum FlightFusionIpcError {
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

impl FlightFusionIpcError {
    pub fn generic(message: impl Into<String>) -> FlightFusionIpcError {
        let msg: String = message.into();
        FlightFusionIpcError::Generic(msg)
    }
}

pub type Result<T> = std::result::Result<T, FlightFusionIpcError>;

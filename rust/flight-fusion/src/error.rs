#[derive(thiserror::Error, Debug)]
pub enum FlightFusionError {
    /// Error returned when an input is in an unexpected format or contains invalid data
    #[error("Malformed input {0}")]
    InputError(String),

    /// Error returned when a table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),
}

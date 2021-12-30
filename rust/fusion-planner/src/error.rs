#[derive(thiserror::Error, Debug)]
pub enum FusionPlannerError {
    /// Error returned when the table to be created already exists
    #[error("Error in dependent crate {0}")]
    ExternalError(String),
}

use arrow_deps::{arrow::error::ArrowError, datafusion::error::DataFusionError};

#[derive(thiserror::Error, Debug)]
pub enum FusionPlannerError {
    /// Error returned when the table to be created already exists
    #[error("Error in dependent crate {0}")]
    ExternalError(String),

    /// An error encountered when generating a logical execution plan
    #[error("Error generating query plan {0}")]
    PlanningError(String),

    /// An error encountered when trying to process missing or malformed inputs.
    #[error("Missing or malformed input data {0}")]
    InputError(String),

    /// An error involving a Datafusion operation occurred.
    #[error(transparent)]
    DatafusionError(#[from] DataFusionError),

    /// An error involving an Arrow operation occurred.
    #[error(transparent)]
    ArrowError(#[from] ArrowError),
}

pub type Result<T> = std::result::Result<T, FusionPlannerError>;

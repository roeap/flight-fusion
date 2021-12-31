use arrow_deps::{datafusion::error::DataFusionError, arrow::error::ArrowError};

#[derive(thiserror::Error, Debug)]
pub enum FusionPlannerError {
    /// Error returned when the table to be created already exists
    #[error("Error in dependent crate {0}")]
    ExternalError(String),

    /// Error returned when the table to be created already exists
    #[error("Error generating query plan {0}")]
    PlanningError(String),

    /// An error involving a Datafusion operation occurred.
    #[error(transparent)]
    DatafusionError(#[from] DataFusionError),

    /// An error involving an Arrow operation occurred.
    #[error(transparent)]
    ArrowError(#[from] ArrowError),
}

pub type Result<T> = std::result::Result<T, FusionPlannerError>;

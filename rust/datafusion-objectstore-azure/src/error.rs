//! Custom error type for `DataFusion-ObjectStore-Azure`

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Enum with all errors in this crate.
/// PartialEq is to enable testing for specific error types
#[derive(Debug, PartialEq)]
pub enum AzureError {
    /// Returned when functionality is not yet available.
    NotImplemented(String),
    /// Wrapper for Azure errors
    Azure(String),
}

impl Display for AzureError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AzureError::NotImplemented(desc) => write!(f, "Not yet implemented: {}", desc),
            AzureError::Azure(desc) => write!(f, "Azure error: {}", desc),
        }
    }
}

impl Error for AzureError {}

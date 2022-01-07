//! Abstractions and implementations for writing data to delta tables
use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::basic::LogicalType;
pub use error::*;
pub use writer::*;

pub mod error;
pub mod json;
mod stats;
pub mod time_utils;
pub mod writer;

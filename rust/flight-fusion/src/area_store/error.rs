use arrow_deps::{
    arrow::{datatypes::SchemaRef, error::ArrowError},
    datafusion::parquet::errors::ParquetError,
};
use std::sync::Arc;

/// Enum representing an error when calling [`DeltaWriter`].
#[derive(thiserror::Error, Debug)]
pub enum DeltaWriterError {
    /// Partition column is missing in a record written to delta.
    #[error("Missing partition column: {0}")]
    MissingPartitionColumn(String),

    /// The Arrow RecordBatch schema does not match the expected schema.
    #[error("Arrow RecordBatch schema does not match: RecordBatch schema: {record_batch_schema}, {expected_schema}")]
    SchemaMismatch {
        /// The record batch schema.
        record_batch_schema: SchemaRef,
        /// The schema of the target delta table.
        expected_schema: Arc<arrow_deps::arrow::datatypes::Schema>,
    },

    /// An Arrow RecordBatch could not be created from the JSON buffer.
    #[error("Arrow RecordBatch created from JSON buffer is a None value")]
    EmptyRecordBatch,

    /// A record was written that was not a JSON object.
    #[error("Record {0} is not a JSON object")]
    InvalidRecord(String),

    /// Arrow returned an error.
    #[error("Arrow interaction failed: {source}")]
    Arrow {
        /// The wrapped [`ArrowError`]
        #[from]
        source: ArrowError,
    },

    /// Parquet write failed.
    #[error("Parquet write failed: {source}")]
    Parquet {
        /// The wrapped [`ParquetError`]
        #[from]
        source: ParquetError,
    },

    /// Error returned from std::io
    #[error("std::io::Error: {source}")]
    Io {
        /// The wrapped [`std::io::Error`]
        #[from]
        source: std::io::Error,
    },
}

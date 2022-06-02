use arrow_deps::{
    arrow::{datatypes::SchemaRef, error::ArrowError},
    datafusion::{error::DataFusionError, parquet::errors::ParquetError},
    deltalake::DeltaTableError,
};
use file_cache::DiskCacheError;
use std::sync::Arc;

/// Enum representing an error when calling [`DeltaWriter`].
#[derive(thiserror::Error, Debug)]
pub enum AreaStoreError {
    /// Error returned when a table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

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

    #[error("object_store::Error: {source}")]
    ObjectStore {
        /// The wrapped [`object_store::Error`]
        #[from]
        source: object_store::Error,
    },

    #[error("Error in file cache: {source}")]
    Cache {
        /// The wrapped [`DiskCacheError`]
        #[from]
        source: DiskCacheError,
    },

    #[error("Error in Datafusion: {source}")]
    Datafusion {
        /// The wrapped [`DataFusionError`]
        #[from]
        source: DataFusionError,
    },

    #[error("Error in Delta: {source}")]
    Delta {
        /// The wrapped [`DeltaTableError`]
        #[from]
        source: DeltaTableError,
    },
}

pub type Result<T> = std::result::Result<T, AreaStoreError>;

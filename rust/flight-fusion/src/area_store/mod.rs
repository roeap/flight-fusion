//! Abstractions and implementations for writing data to delta tables
use arrow_deps::arrow::{datatypes::SchemaRef, datatypes::*, error::ArrowError, record_batch::*};
use arrow_deps::datafusion::parquet::{basic::LogicalType, errors::ParquetError};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
pub use writer::*;

pub mod json;
mod stats;
pub mod writer;

const NULL_PARTITION_VALUE_DATA_PATH: &str = "__HIVE_DEFAULT_PARTITION__";

#[derive(Debug, PartialEq, Eq)]
pub enum ColumnValueStat {
    /// Composite HashMap representation of statistics.
    Column(HashMap<String, ColumnValueStat>),
    /// Json representation of statistics.
    Value(Value),
}

impl ColumnValueStat {
    /// Returns the HashMap representation of the ColumnValueStat.
    pub fn as_column(&self) -> Option<&HashMap<String, ColumnValueStat>> {
        match self {
            ColumnValueStat::Column(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the serde_json representation of the ColumnValueStat.
    pub fn as_value(&self) -> Option<&Value> {
        match self {
            ColumnValueStat::Value(v) => Some(v),
            _ => None,
        }
    }
}

/// Struct used to represent nullCount in add action statistics.
#[derive(Debug, PartialEq, Eq)]
pub enum ColumnCountStat {
    /// Composite HashMap representation of statistics.
    Column(HashMap<String, ColumnCountStat>),
    /// Json representation of statistics.
    Value(i64),
}

impl ColumnCountStat {
    /// Returns the HashMap representation of the ColumnCountStat.
    pub fn as_column(&self) -> Option<&HashMap<String, ColumnCountStat>> {
        match self {
            ColumnCountStat::Column(m) => Some(m),
            _ => None,
        }
    }

    /// Returns the serde_json representation of the ColumnCountStat.
    pub fn as_value(&self) -> Option<i64> {
        match self {
            ColumnCountStat::Value(v) => Some(*v),
            _ => None,
        }
    }
}

/// Statistics associated with Add actions contained in the Delta log.
#[derive(Debug, Default)]
pub struct Stats {
    /// Number of records in the file associated with the log action.
    pub num_records: i64,

    // start of per column stats
    /// Contains a value smaller than all values present in the file for all columns.
    pub min_values: HashMap<String, ColumnValueStat>,
    /// Contains a value larger than all values present in the file for all columns.
    pub max_values: HashMap<String, ColumnValueStat>,
    /// The number of null values for all columns.
    pub null_count: HashMap<String, ColumnCountStat>,
}

type NullCounts = HashMap<String, ColumnCountStat>;
type MinAndMaxValues = (
    HashMap<String, ColumnValueStat>,
    HashMap<String, ColumnValueStat>,
);

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

    /// Indicates that a partial write was performed and error records were discarded.
    #[error("Failed to write some values to parquet. Sample error: {sample_error}.")]
    PartialParquetWrite {
        /// Vec of tuples where the first element of each tuple is the skipped value and the second element is the [`ParquetError`] associated with it.
        skipped_values: Vec<(Value, ParquetError)>,
        /// A sample [`ParquetError`] representing the overall partial write.
        sample_error: ParquetError,
    },

    // TODO: derive Debug for Stats in delta-rs
    /// Serialization of delta log statistics failed.
    #[error("Serialization of delta log statistics failed")]
    StatsSerializationFailed {
        /// The stats object that failed serialization.
        stats: Stats,
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
}

/// Utility functions for handling timestamps
pub mod time_utils {
    use arrow_deps::arrow::temporal_conversions;
    use parquet_format::TimeUnit;

    /// Convert an ISO-8601/RFC3339 timestamp string to a numeric microsecond epoch representation.
    /// Stats strings are written with millisecond precision as described by the delta protocol.
    pub fn timestamp_micros_from_stats_string(s: &str) -> Result<i64, chrono::format::ParseError> {
        chrono::DateTime::parse_from_rfc3339(s).map(|dt| dt.timestamp_millis() * 1000)
    }

    /// Convert the timestamp to a ISO-8601 style format suitable for JSON statistics.
    pub fn timestamp_to_delta_stats_string(n: i64, time_unit: &TimeUnit) -> String {
        let dt = match time_unit {
            TimeUnit::MILLIS(_) => temporal_conversions::timestamp_ms_to_datetime(n),
            TimeUnit::MICROS(_) => temporal_conversions::timestamp_us_to_datetime(n),
            TimeUnit::NANOS(_) => temporal_conversions::timestamp_ns_to_datetime(n),
        };

        format!("{}", dt.format("%Y-%m-%dT%H:%M:%S%.3fZ"))
    }
}

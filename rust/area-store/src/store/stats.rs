use super::*;
use crate::error::Result;
use arrow_deps::arrow::{
    self,
    array::{
        as_boolean_array, as_primitive_array, as_struct_array, make_array, Array, ArrayData,
        StructArray,
    },
    buffer::MutableBuffer,
    datatypes::DataType,
};
use arrow_deps::datafusion::parquet::{
    self,
    errors::ParquetError,
    file::{metadata::RowGroupMetaData, statistics::Statistics},
    schema::types::{ColumnDescriptor, SchemaDescriptor},
};
use parquet_format::FileMetaData;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub type NullCounts = HashMap<String, ColumnCountStat>;
pub type MinAndMaxValues = (
    HashMap<String, ColumnValueStat>,
    HashMap<String, ColumnValueStat>,
);
use arrow_deps::arrow::temporal_conversions;
use parquet_format::TimeUnit;

#[derive(Debug, Clone)]
// TODO move and rename to something useful
pub struct Add {
    size: i64,
    stats: Stats,
    modification_time: i64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, PartialEq, Eq, Clone)]
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
#[derive(Debug, Default, Clone)]
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

pub(crate) fn create_add(
    partition_values: &HashMap<String, Option<String>>,
    null_counts: NullCounts,
    path: String,
    size: i64,
    file_metadata: &FileMetaData,
) -> Result<Add> {
    let (min_values, max_values) =
        min_max_values_from_file_metadata(partition_values, file_metadata)?;

    let stats = Stats {
        num_records: file_metadata.num_rows,
        min_values,
        max_values,
        null_count: null_counts,
    };

    // Determine the modification timestamp to include in the add action - milliseconds since epoch
    // Err should be impossible in this case since `SystemTime::now()` is always greater than `UNIX_EPOCH`
    let modification_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let modification_time = modification_time.as_millis() as i64;

    Ok(Add {
        size,
        stats,
        modification_time,
    })
}

pub(crate) fn apply_null_counts(
    array: &StructArray,
    null_counts: &mut HashMap<String, ColumnCountStat>,
    nest_level: i32,
) {
    let fields = match array.data_type() {
        DataType::Struct(fields) => fields,
        _ => unreachable!(),
    };

    array
        .columns()
        .iter()
        .zip(fields)
        .for_each(|(column, field)| {
            let key = field.name().to_owned();

            match column.data_type() {
                // Recursive case
                DataType::Struct(_) => {
                    let col_struct = null_counts
                        .entry(key)
                        .or_insert_with(|| ColumnCountStat::Column(HashMap::new()));

                    match col_struct {
                        ColumnCountStat::Column(map) => {
                            apply_null_counts(
                                as_struct_array(&(*column).clone()),
                                map,
                                nest_level + 1,
                            );
                        }
                        _ => unreachable!(),
                    }
                }
                // Base case
                _ => {
                    let col_struct = null_counts
                        .entry(key.clone())
                        .or_insert_with(|| ColumnCountStat::Value(0));

                    match col_struct {
                        ColumnCountStat::Value(n) => {
                            let null_count = column.null_count() as i64;
                            let n = null_count + *n;
                            null_counts.insert(key, ColumnCountStat::Value(n));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        });
}

fn min_max_values_from_file_metadata(
    partition_values: &HashMap<String, Option<String>>,
    file_metadata: &FileMetaData,
) -> std::result::Result<MinAndMaxValues, ParquetError> {
    let type_ptr = parquet::schema::types::from_thrift(file_metadata.schema.as_slice());
    let schema_descriptor = type_ptr.map(|type_| Arc::new(SchemaDescriptor::new(type_)))?;

    let mut min_values: HashMap<String, ColumnValueStat> = HashMap::new();
    let mut max_values: HashMap<String, ColumnValueStat> = HashMap::new();

    let row_group_metadata: std::result::Result<Vec<RowGroupMetaData>, ParquetError> =
        file_metadata
            .row_groups
            .iter()
            .map(|rg| RowGroupMetaData::from_thrift(schema_descriptor.clone(), rg.clone()))
            .collect();
    let row_group_metadata = row_group_metadata?;

    for i in 0..schema_descriptor.num_columns() {
        let column_descr = schema_descriptor.column(i);

        // If max rep level is > 0, this is an array element or a struct element of an array or something downstream of an array.
        // delta/databricks only computes null counts for arrays - not min max.
        // null counts are tracked at the record batch level, so skip any column with max_rep_level
        // > 0
        if column_descr.max_rep_level() > 0 {
            continue;
        }

        let column_path = column_descr.path();
        let column_path_parts = column_path.parts();

        // Do not include partition columns in statistics
        if partition_values.contains_key(&column_path_parts[0]) {
            continue;
        }

        let statistics: Vec<&Statistics> = row_group_metadata
            .iter()
            .filter_map(|g| g.column(i).statistics())
            .collect();

        let _ = apply_min_max_for_column(
            statistics.as_slice(),
            column_descr.clone(),
            column_path_parts,
            &mut min_values,
            &mut max_values,
        )?;
    }

    Ok((min_values, max_values))
}

fn apply_min_max_for_column(
    statistics: &[&Statistics],
    column_descr: Arc<ColumnDescriptor>,
    column_path_parts: &[String],
    min_values: &mut HashMap<String, ColumnValueStat>,
    max_values: &mut HashMap<String, ColumnValueStat>,
) -> std::result::Result<(), ParquetError> {
    match (column_path_parts.len(), column_path_parts.first()) {
        // Base case - we are at the leaf struct level in the path
        (1, _) => {
            let (min, max) = min_and_max_from_parquet_statistics(statistics, column_descr.clone())?;

            if let Some(min) = min {
                let min = ColumnValueStat::Value(min);
                min_values.insert(column_descr.name().to_string(), min);
            }

            if let Some(max) = max {
                let max = ColumnValueStat::Value(max);
                max_values.insert(column_descr.name().to_string(), max);
            }

            Ok(())
        }
        // Recurse to load value at the appropriate level of HashMap
        (_, Some(key)) => {
            let child_min_values = min_values
                .entry(key.to_owned())
                .or_insert_with(|| ColumnValueStat::Column(HashMap::new()));
            let child_max_values = max_values
                .entry(key.to_owned())
                .or_insert_with(|| ColumnValueStat::Column(HashMap::new()));

            match (child_min_values, child_max_values) {
                (ColumnValueStat::Column(mins), ColumnValueStat::Column(maxes)) => {
                    let remaining_parts: Vec<String> = column_path_parts
                        .iter()
                        .skip(1)
                        .map(|s| s.to_string())
                        .collect();

                    apply_min_max_for_column(
                        statistics,
                        column_descr,
                        remaining_parts.as_slice(),
                        mins,
                        maxes,
                    )?;

                    Ok(())
                }
                _ => {
                    unreachable!();
                }
            }
        }
        // column path parts will always have at least one element.
        (_, None) => {
            unreachable!();
        }
    }
}

#[inline]
fn is_utf8(opt: Option<LogicalType>) -> bool {
    matches!(opt.as_ref(), Some(LogicalType::String))
}

fn min_and_max_from_parquet_statistics(
    statistics: &[&Statistics],
    column_descr: Arc<ColumnDescriptor>,
) -> std::result::Result<(Option<Value>, Option<Value>), ParquetError> {
    let stats_with_min_max: Vec<&Statistics> = statistics
        .iter()
        .filter(|s| s.has_min_max_set())
        .copied()
        .collect();

    if stats_with_min_max.is_empty() {
        return Ok((None, None));
    }

    let (data_size, data_type) = match stats_with_min_max.first() {
        Some(Statistics::Boolean(_)) => (std::mem::size_of::<bool>(), DataType::Boolean),
        Some(Statistics::Int32(_)) => (std::mem::size_of::<i32>(), DataType::Int32),
        Some(Statistics::Int64(_)) => (std::mem::size_of::<i64>(), DataType::Int64),
        Some(Statistics::Float(_)) => (std::mem::size_of::<f32>(), DataType::Float32),
        Some(Statistics::Double(_)) => (std::mem::size_of::<f64>(), DataType::Float64),
        Some(Statistics::ByteArray(_)) if is_utf8(column_descr.logical_type()) => {
            (0, DataType::Utf8)
        }
        _ => {
            // NOTE: Skips
            // Statistics::Int96(_)
            // Statistics::ByteArray(_)
            // Statistics::FixedLenByteArray(_)

            return Ok((None, None));
        }
    };

    if data_type == DataType::Utf8 {
        return Ok(min_max_strings_from_stats(&stats_with_min_max));
    }

    let arrow_buffer_capacity = stats_with_min_max.len() * data_size;

    let min_array = arrow_array_from_bytes(
        data_type.clone(),
        arrow_buffer_capacity,
        stats_with_min_max.iter().map(|s| s.min_bytes()).collect(),
    );

    let max_array = arrow_array_from_bytes(
        data_type.clone(),
        arrow_buffer_capacity,
        stats_with_min_max.iter().map(|s| s.max_bytes()).collect(),
    );

    match data_type {
        DataType::Boolean => {
            let min = arrow::compute::min_boolean(as_boolean_array(&min_array));
            let min = min.map(Value::Bool);

            let max = arrow::compute::max_boolean(as_boolean_array(&max_array));
            let max = max.map(Value::Bool);

            Ok((min, max))
        }
        DataType::Int32 => {
            let min_array = as_primitive_array::<arrow::datatypes::Int32Type>(&min_array);
            let min = arrow::compute::min(min_array);
            let min = min.map(|i| Value::Number(Number::from(i)));

            let max_array = as_primitive_array::<arrow::datatypes::Int32Type>(&max_array);
            let max = arrow::compute::max(max_array);
            let max = max.map(|i| Value::Number(Number::from(i)));

            Ok((min, max))
        }
        DataType::Int64 => {
            let min_array = as_primitive_array::<arrow::datatypes::Int64Type>(&min_array);
            let min = arrow::compute::min(min_array);
            let max_array = as_primitive_array::<arrow::datatypes::Int64Type>(&max_array);
            let max = arrow::compute::max(max_array);

            match column_descr.logical_type().as_ref() {
                Some(LogicalType::Timestamp { unit, .. }) => {
                    let min = min.map(|n| Value::String(timestamp_to_delta_stats_string(n, unit)));
                    let max = max.map(|n| Value::String(timestamp_to_delta_stats_string(n, unit)));
                    Ok((min, max))
                }
                _ => {
                    let min = min.map(|i| Value::Number(Number::from(i)));
                    let max = max.map(|i| Value::Number(Number::from(i)));

                    Ok((min, max))
                }
            }
        }
        DataType::Float32 => {
            let min_array = as_primitive_array::<arrow::datatypes::Float32Type>(&min_array);
            let min = arrow::compute::min(min_array)
                .and_then(|f| Number::from_f64(f as f64).map(Value::Number));

            let max_array = as_primitive_array::<arrow::datatypes::Float32Type>(&max_array);
            let max = arrow::compute::max(max_array)
                .and_then(|f| Number::from_f64(f as f64).map(Value::Number));

            Ok((min, max))
        }
        DataType::Float64 => {
            let min_array = as_primitive_array::<arrow::datatypes::Float64Type>(&min_array);
            let min =
                arrow::compute::min(min_array).and_then(|f| Number::from_f64(f).map(Value::Number));

            let max_array = as_primitive_array::<arrow::datatypes::Float64Type>(&max_array);
            let max =
                arrow::compute::max(max_array).and_then(|f| Number::from_f64(f).map(Value::Number));

            Ok((min, max))
        }
        _ => Ok((None, None)),
    }
}

fn min_max_strings_from_stats(
    stats_with_min_max: &[&Statistics],
) -> (Option<Value>, Option<Value>) {
    let min_string_candidates = stats_with_min_max
        .iter()
        .filter_map(|s| std::str::from_utf8(s.min_bytes()).ok());

    let min_value = min_string_candidates
        .min()
        .map(|s| Value::String(s.to_string()));

    let max_string_candidates = stats_with_min_max
        .iter()
        .filter_map(|s| std::str::from_utf8(s.max_bytes()).ok());

    let max_value = max_string_candidates
        .max()
        .map(|s| Value::String(s.to_string()));

    (min_value, max_value)
}

fn arrow_array_from_bytes(
    data_type: DataType,
    capacity: usize,
    byte_arrays: Vec<&[u8]>,
) -> Arc<dyn Array> {
    let mut buffer = MutableBuffer::new(capacity);

    for arr in byte_arrays.iter() {
        buffer.extend_from_slice(arr);
    }

    let builder = ArrayData::builder(data_type)
        .len(byte_arrays.len())
        .add_buffer(buffer.into());

    // TODO remove panic
    let data = builder.build().unwrap();

    make_array(data)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_record_batch;
    use std::collections::HashMap;

    #[test]
    fn test_apply_null_counts() {
        let record_batch = get_record_batch(None, true);
        let mut ref_null_counts = HashMap::new();
        ref_null_counts.insert("id".to_string(), ColumnCountStat::Value(3));
        ref_null_counts.insert("value".to_string(), ColumnCountStat::Value(1));
        ref_null_counts.insert("modified".to_string(), ColumnCountStat::Value(0));

        let mut null_counts = HashMap::new();
        apply_null_counts(&record_batch.into(), &mut null_counts, 0);

        assert_eq!(null_counts, ref_null_counts)
    }
}

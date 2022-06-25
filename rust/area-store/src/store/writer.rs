//! Main writer API to write record batches to delta table
use super::{
    stats::{apply_null_counts, create_add, Add, NullCounts},
    *,
};
use crate::error::{AreaStoreError, Result};
use arrow_deps::arrow::{
    self,
    array::{as_primitive_array, Array, UInt32Array},
    compute::{lexicographical_partition_ranges, lexsort_to_indices, take, SortColumn},
    datatypes::{DataType, Schema as ArrowSchema, SchemaRef as ArrowSchemaRef, *},
};
use arrow_deps::datafusion::parquet::{
    arrow::ArrowWriter, basic::Compression, errors::ParquetError,
    file::properties::WriterProperties,
};
use arrow_deps::deltalake::writer::utils::ShareableBuffer;
use object_store::{path::Path, DynObjectStore};
use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;
use uuid::Uuid;

const NULL_PARTITION_VALUE_DATA_PATH: &str = "__HIVE_DEFAULT_PARTITION__";

pub struct DeltaWriter {
    pub(crate) storage: Arc<DynObjectStore>,
    pub(crate) arrow_schema_ref: Arc<ArrowSchema>,
    pub(crate) writer_properties: WriterProperties,
    pub(crate) partition_columns: Vec<String>,
    pub(crate) arrow_writers: HashMap<String, PartitionWriter>,
}

impl std::fmt::Debug for DeltaWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeltaWriter")
    }
}

impl DeltaWriter {
    /// Create a new DeltaWriter instance
    pub fn new(
        storage: Arc<DynObjectStore>,
        schema: ArrowSchemaRef,
        partition_columns: Option<Vec<String>>,
    ) -> Self {
        // Initialize writer properties for the underlying arrow writer
        let writer_properties = WriterProperties::builder()
            // NOTE: Consider extracting config for writer properties and setting more than just compression
            .set_compression(Compression::SNAPPY)
            .build();

        Self {
            storage,
            arrow_schema_ref: schema,
            writer_properties,
            partition_columns: partition_columns.unwrap_or_default(),
            arrow_writers: HashMap::new(),
        }
    }

    /// Write [`RecordBatch`] to internal buffers
    ///
    /// Divides a single record batch into into multiple according to table partitioning.
    /// Values are written to arrow buffers, to collect data until it should be written to disk.
    pub fn write(&mut self, values: &RecordBatch) -> Result<()> {
        let arrow_schema = self.partition_arrow_schema();
        for result in self.divide_by_partition_values(values)? {
            let partition_key = self
                .get_partition_key(&self.partition_columns, &result.partition_values)?
                .join("/");
            match self.arrow_writers.get_mut(&partition_key) {
                Some(writer) => {
                    writer.write(&result.record_batch)?;
                }
                None => {
                    let mut writer = PartitionWriter::new(
                        arrow_schema.clone(),
                        result.partition_values,
                        self.writer_properties.clone(),
                    )?;
                    writer.write(&result.record_batch)?;
                    let _ = self.arrow_writers.insert(partition_key, writer);
                }
            }
        }

        Ok(())
    }

    /// Writes the existing parquet bytes to storage and resets internal state to handle another file.
    pub async fn flush(&mut self, location: &Path) -> Result<Vec<Add>> {
        let writers = std::mem::take(&mut self.arrow_writers);
        let mut actions = Vec::new();

        for (_, mut writer) in writers {
            let metadata = writer.arrow_writer.close()?;
            let file_loc = self.next_data_path(location, &writer.partition_values, None)?;

            let obj_bytes = bytes::Bytes::from(writer.buffer.to_vec());
            let file_size = obj_bytes.len() as i64;
            self.storage.put(&file_loc, obj_bytes).await.unwrap();

            // Replace self null_counts with an empty map. Use the other for stats.
            let null_counts = std::mem::take(&mut writer.null_counts);

            actions.push(create_add(
                &writer.partition_values,
                null_counts,
                location.as_ref().to_string(),
                file_size,
                &metadata,
            )?);
        }

        Ok(actions)
    }

    fn divide_by_partition_values(&mut self, values: &RecordBatch) -> Result<Vec<PartitionResult>> {
        divide_by_partition_values(
            self.partition_arrow_schema(),
            self.partition_columns.clone(),
            values,
        )
    }

    // TODO: parquet files have a 5 digit zero-padded prefix and a "c\d{3}" suffix that
    // I have not been able to find documentation for yet.
    fn next_data_path(
        &self,
        location: &Path,
        partition_values: &HashMap<String, Option<String>>,
        part: Option<i32>,
    ) -> Result<Path> {
        // TODO: what does 00000 mean?
        // TODO (roeap): my understanding is, that the values are used as a counter - i.e. if a single batch of
        // data written to one partition needs to be split due to desired file size constraints.
        let first_part = match part {
            Some(count) => format!("{:0>5}", count),
            _ => "00000".to_string(),
        };
        let uuid_part = Uuid::new_v4();
        // TODO: what does c000 mean?
        let last_part = "c000";

        // NOTE: If we add a non-snappy option, file name must change
        let file_name = format!(
            "part-{}-{}-{}.snappy.parquet",
            first_part, uuid_part, last_part
        );

        let mut new_path = location.parts().collect::<Vec<_>>();

        if self.partition_columns.is_empty() {
            new_path.push(file_name.into());
            return Ok(Path::from_iter(new_path));
        }

        self.get_partition_key(&self.partition_columns.clone(), partition_values)?
            .into_iter()
            .for_each(|s| new_path.push(s.into()));

        new_path.push(file_name.into());

        Ok(Path::from_iter(new_path))
    }

    pub(crate) fn get_partition_key(
        &self,
        partition_columns: &[String],
        partition_values: &HashMap<String, Option<String>>,
    ) -> Result<Vec<String>> {
        let mut parts = Vec::new();
        for k in partition_columns.iter() {
            let partition_value = partition_values
                .get(k)
                .ok_or_else(|| AreaStoreError::MissingPartitionColumn(k.to_string()))?;

            let partition_value = partition_value
                .as_deref()
                .unwrap_or(NULL_PARTITION_VALUE_DATA_PATH);
            let part = format!("{}={}", k, partition_value);

            parts.push(part);
        }
        Ok(parts)
    }

    /// Returns the current byte length of the in memory buffer.
    /// This may be used by the caller to decide when to finalize the file write.
    pub fn buffer_len(&self) -> usize {
        self.arrow_writers.values().map(|w| w.buffer_len()).sum()
    }

    /// Returns the number of records held in the current buffer.
    pub fn buffered_record_batch_count(&self) -> usize {
        self.arrow_writers
            .values()
            .map(|w| w.buffered_record_batch_count)
            .sum()
    }

    /// Resets internal state.
    pub fn reset(&mut self) {
        self.arrow_writers.clear();
    }

    /// Returns the arrow schema representation of the delta table schema defined for the wrapped
    /// table.
    pub fn arrow_schema(&self) -> ArrowSchemaRef {
        self.arrow_schema_ref.clone()
    }

    /// Returns the arrow schema representation of the partitioned files written to table
    pub fn partition_arrow_schema(&self) -> ArrowSchemaRef {
        Arc::new(ArrowSchema::new(
            self.arrow_schema_ref
                .fields()
                .iter()
                .filter(|f| !self.partition_columns.contains(f.name()))
                .map(|f| f.to_owned())
                .collect::<Vec<_>>(),
        ))
    }
}

/// Helper container for partitioned record batches
pub struct PartitionResult {
    /// values found in partition columns
    pub partition_values: HashMap<String, Option<String>>,
    /// remaining dataset with partition column values removed
    pub record_batch: RecordBatch,
}

pub struct PartitionWriter {
    arrow_schema: Arc<ArrowSchema>,
    writer_properties: WriterProperties,
    pub(super) buffer: ShareableBuffer,
    pub(super) arrow_writer: ArrowWriter<ShareableBuffer>,
    pub(super) partition_values: HashMap<String, Option<String>>,
    pub(super) null_counts: NullCounts,
    pub(super) buffered_record_batch_count: usize,
}

impl PartitionWriter {
    pub fn new(
        arrow_schema: Arc<ArrowSchema>,
        partition_values: HashMap<String, Option<String>>,
        writer_properties: WriterProperties,
    ) -> std::result::Result<Self, ParquetError> {
        let buffer = ShareableBuffer::default();
        let arrow_writer = ArrowWriter::try_new(
            buffer.clone(),
            arrow_schema.clone(),
            Some(writer_properties.clone()),
        )?;
        let null_counts = NullCounts::new();
        let buffered_record_batch_count = 0;

        Ok(Self {
            arrow_schema,
            writer_properties,
            buffer,
            arrow_writer,
            partition_values,
            null_counts,
            buffered_record_batch_count,
        })
    }

    /// Writes the record batch in-memory and updates internal state accordingly.
    ///
    /// This method buffers the write stream internally so it can be invoked for many
    /// record batches and flushed after the appropriate number of bytes has been written.
    pub fn write(&mut self, record_batch: &RecordBatch) -> Result<()> {
        if record_batch.schema() != self.arrow_schema {
            return Err(AreaStoreError::SchemaMismatch {
                record_batch_schema: record_batch.schema(),
                expected_schema: self.arrow_schema.clone(),
            });
        }

        // Copy current cursor bytes so we can recover from failures
        let buffer_bytes = self.buffer.to_vec();
        match self.arrow_writer.write(record_batch) {
            Ok(_) => {
                self.buffered_record_batch_count += 1;
                apply_null_counts(&record_batch.clone().into(), &mut self.null_counts, 0);
                Ok(())
            }
            // If a write fails we need to reset the state of the PartitionWriter
            Err(e) => {
                let new_buffer = ShareableBuffer::from_bytes(buffer_bytes.as_slice());
                let _ = std::mem::replace(&mut self.buffer, new_buffer.clone());
                let arrow_writer = ArrowWriter::try_new(
                    new_buffer,
                    self.arrow_schema.clone(),
                    Some(self.writer_properties.clone()),
                )?;
                let _ = std::mem::replace(&mut self.arrow_writer, arrow_writer);
                // TODO we used to clear partition values here, but since we pre-partition the
                // record batches, we should never try with mismatching partition values.

                Err(e.into())
            }
        }
    }

    /// Returns the current byte length of the in memory buffer.
    /// This may be used by the caller to decide when to finalize the file write.
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
}

/// Partition a RecordBatch along partition columns
pub fn divide_by_partition_values(
    arrow_schema: ArrowSchemaRef,
    partition_columns: Vec<String>,
    values: &RecordBatch,
) -> Result<Vec<PartitionResult>> {
    let mut partitions = Vec::new();

    if partition_columns.is_empty() {
        partitions.push(PartitionResult {
            partition_values: HashMap::new(),
            record_batch: values.clone(),
        });
        return Ok(partitions);
    }

    let schema = values.schema();

    // collect all columns in order relevant for partitioning
    let sort_columns = partition_columns
        .clone()
        .into_iter()
        .map(|col| SortColumn {
            values: values.column(schema.index_of(&col).unwrap()).clone(),
            options: None,
        })
        .collect::<Vec<_>>();

    let indices = lexsort_to_indices(sort_columns.as_slice(), None).unwrap();
    let sorted_partition_columns = sort_columns
        .iter()
        .map(|c| SortColumn {
            values: take(c.values.as_ref(), &indices, None).unwrap(),
            options: None,
        })
        .collect::<Vec<_>>();

    let partition_ranges = lexicographical_partition_ranges(sorted_partition_columns.as_slice())?;

    for range in partition_ranges {
        // get row indices for current partition
        let idx: UInt32Array = (range.start..range.end)
            .map(|i| Some(indices.value(i)))
            .into_iter()
            .collect();

        let partition_key_iter = sorted_partition_columns.iter().map(|c| {
            stringified_partition_value(&c.values.slice(range.start, range.end - range.start))
                .unwrap()
        });

        let mut partition_values = HashMap::new();
        for (key, value) in partition_columns.clone().iter().zip(partition_key_iter) {
            partition_values.insert(key.clone(), value);
        }

        let batch_data = arrow_schema
            .fields()
            .iter()
            .map(|f| values.column(schema.index_of(f.name()).unwrap()).clone())
            .map(move |col| take(col.as_ref(), &idx, None).unwrap())
            .collect::<Vec<_>>();

        partitions.push(PartitionResult {
            partition_values,
            record_batch: RecordBatch::try_new(arrow_schema.clone(), batch_data).unwrap(),
        });
    }

    Ok(partitions)
}

// very naive implementation for plucking the partition value from the first element of a column array.
// ideally, we would do some validation to ensure the record batch containing the passed partition column contains only distinct values.
// if we calculate stats _first_, we can avoid the extra iteration by ensuring max and min match for the column.
// however, stats are optional and can be added later with `dataChange` false log entries, and it may be more appropriate to add stats _later_ to speed up the initial write.
// a happy middle-road might be to compute stats for partition columns only on the initial write since we should validate partition values anyway, and compute additional stats later (at checkpoint time perhaps?).
// also this does not currently support nested partition columns and many other data types.
// TODO is this comment still valid, since we should be sure now, that the arrays where this
// gets aplied have a single unique value
fn stringified_partition_value(arr: &Arc<dyn Array>) -> Result<Option<String>> {
    let data_type = arr.data_type();

    if arr.is_null(0) {
        return Ok(None);
    }

    let s = match data_type {
        DataType::Int8 => as_primitive_array::<Int8Type>(arr).value(0).to_string(),
        DataType::Int16 => as_primitive_array::<Int16Type>(arr).value(0).to_string(),
        DataType::Int32 => as_primitive_array::<Int32Type>(arr).value(0).to_string(),
        DataType::Int64 => as_primitive_array::<Int64Type>(arr).value(0).to_string(),
        DataType::UInt8 => as_primitive_array::<UInt8Type>(arr).value(0).to_string(),
        DataType::UInt16 => as_primitive_array::<UInt16Type>(arr).value(0).to_string(),
        DataType::UInt32 => as_primitive_array::<UInt32Type>(arr).value(0).to_string(),
        DataType::UInt64 => as_primitive_array::<UInt64Type>(arr).value(0).to_string(),
        DataType::Utf8 => {
            let data = arrow::array::as_string_array(arr);

            data.value(0).to_string()
        }
        // TODO: handle more types
        _ => {
            unimplemented!("Unimplemented data type: {:?}", data_type);
        }
    };

    Ok(Some(s))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_record_batch;
    use object_store::local::LocalFileSystem;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_divide_record_batch_no_partition() {
        let root = TempDir::new().unwrap();
        let storage = LocalFileSystem::new_with_prefix(root.path()).unwrap();
        let batch = get_record_batch(None, false);
        let schema = batch.schema();
        let partition_cols = vec![];
        let mut writer = DeltaWriter::new(Arc::new(storage), schema, Some(partition_cols));
        let partitions = writer.divide_by_partition_values(&batch).unwrap();

        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0].record_batch, batch)
    }

    #[tokio::test]
    async fn test_divide_record_batch_single_partition() {
        let root = TempDir::new().unwrap();
        let storage = LocalFileSystem::new_with_prefix(root.path()).unwrap();

        let batch = get_record_batch(None, false);
        let schema = batch.schema();
        let partition_cols = vec!["modified".to_string()];
        let mut writer = DeltaWriter::new(Arc::new(storage), schema, Some(partition_cols.clone()));

        let partitions = writer.divide_by_partition_values(&batch).unwrap();

        let expected_keys = vec![
            String::from("modified=2021-02-01"),
            String::from("modified=2021-02-02"),
        ];
        validate_partition_map(partitions, &partition_cols, expected_keys, &writer)
    }

    #[tokio::test]
    async fn test_divide_record_batch_multiple_partitions() {
        let root = TempDir::new().unwrap();
        let storage = LocalFileSystem::new_with_prefix(root.path()).unwrap();
        let batch = get_record_batch(None, false);
        let schema = batch.schema();
        let partition_cols = vec!["modified".to_string(), "id".to_string()];
        let mut writer = DeltaWriter::new(Arc::new(storage), schema, Some(partition_cols.clone()));

        let partitions = writer.divide_by_partition_values(&batch).unwrap();

        let expected_keys = vec![
            String::from("modified=2021-02-01/id=A"),
            String::from("modified=2021-02-01/id=B"),
            String::from("modified=2021-02-02/id=A"),
            String::from("modified=2021-02-02/id=B"),
        ];
        validate_partition_map(partitions, &partition_cols, expected_keys, &writer)
    }

    #[tokio::test]
    async fn test_write_no_partitions() {
        let root = TempDir::new().unwrap();
        let storage = LocalFileSystem::new_with_prefix(root.path()).unwrap();
        let location = Path::default();
        let batch = get_record_batch(None, false);
        let schema = batch.schema();
        let partition_cols = vec![];
        let mut writer = DeltaWriter::new(Arc::new(storage), schema, Some(partition_cols));

        writer.write(&batch).unwrap();
        let adds = writer.flush(&location).await.unwrap();
        assert_eq!(adds.len(), 1);
    }

    #[tokio::test]
    async fn test_write_multiple_partitions() {
        let root = TempDir::new().unwrap();
        let storage = LocalFileSystem::new_with_prefix(root.path()).unwrap();
        let location = Path::default();
        let batch = get_record_batch(None, false);
        let schema = batch.schema();
        let partition_cols = vec!["modified".to_string(), "id".to_string()];
        let mut writer = DeltaWriter::new(Arc::new(storage), schema, Some(partition_cols));

        writer.write(&batch).unwrap();
        let adds = writer.flush(&location).await.unwrap();
        assert_eq!(adds.len(), 4);

        let expected_keys = vec![
            String::from("modified=2021-02-01/id=A"),
            String::from("modified=2021-02-01/id=B"),
            String::from("modified=2021-02-02/id=A"),
            String::from("modified=2021-02-02/id=B"),
        ];
        let table_dir = root.path();
        for key in expected_keys {
            let partition_dir = table_dir.join(key);
            assert!(partition_dir.exists())
        }
    }

    fn validate_partition_map(
        partitions: Vec<PartitionResult>,
        partition_cols: &[String],
        expected_keys: Vec<String>,
        writer: &DeltaWriter,
    ) {
        assert_eq!(partitions.len(), expected_keys.len());
        for result in partitions {
            let partition_key = writer
                .get_partition_key(partition_cols, &result.partition_values)
                .unwrap()
                .join("/");
            assert!(expected_keys.contains(&partition_key));
            let ref_batch = get_record_batch(Some(partition_key.clone()), false);
            assert_eq!(ref_batch, result.record_batch);
        }
    }
}

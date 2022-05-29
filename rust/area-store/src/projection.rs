use arrow_deps::arrow::{
    array::{new_null_array, ArrayRef},
    datatypes::{Field, Schema, SchemaRef},
    error::{ArrowError, Result as ArrowResult},
    record_batch::{RecordBatch, RecordBatchOptions},
};
use arrow_deps::datafusion::{
    error::{DataFusionError, Result as DFResult},
    scalar::ScalarValue,
};
use observability_deps::tracing::info;
use std::{collections::HashMap, sync::Arc, vec};

/// The base configurations to provide when creating a physical plan for
/// any given file format.
#[derive(Debug, Clone)]
pub struct FileScanConfig {
    /// Schema before projection. It contains the columns that are expected
    /// to be in the files without the table partition columns.
    pub file_schema: SchemaRef,
    /// Columns on which to project the data. Indexes that are higher than the
    /// number of columns of `file_schema` refer to `table_partition_cols`.
    pub projection: Option<Vec<usize>>,
    /// The partitioning column names
    pub table_partition_cols: Vec<Field>,
}

impl FileScanConfig {
    /// Project the schema and the statistics on the given column indices
    pub fn project(&self) -> SchemaRef {
        if self.projection.is_none() && self.table_partition_cols.is_empty() {
            return Arc::clone(&self.file_schema);
        }

        let proj_iter: Box<dyn Iterator<Item = usize>> = match &self.projection {
            Some(proj) => Box::new(proj.iter().copied()),
            None => {
                Box::new(0..(self.file_schema.fields().len() + self.table_partition_cols.len()))
            }
        };

        let mut table_fields = vec![];
        for idx in proj_iter {
            if idx < self.file_schema.fields().len() {
                table_fields.push(self.file_schema.field(idx).clone());
            } else {
                let partition_idx = idx - self.file_schema.fields().len();
                table_fields.push(self.table_partition_cols[partition_idx].clone());
            }
        }

        Arc::new(Schema::new(table_fields))
    }

    pub fn projected_file_column_names(&self) -> Option<Vec<String>> {
        self.projection.as_ref().map(|p| {
            p.iter()
                .filter(|col_idx| **col_idx < self.file_schema.fields().len())
                .map(|col_idx| self.file_schema.field(*col_idx).name())
                .cloned()
                .collect()
        })
    }

    pub fn file_column_projection_indices(&self) -> Option<Vec<usize>> {
        self.projection.as_ref().map(|p| {
            p.iter()
                .filter(|col_idx| **col_idx < self.file_schema.fields().len())
                .copied()
                .collect()
        })
    }
}

/// A utility which can adapt file-level record batches to a table schema which may have a schema
/// obtained from merging multiple file-level schemas.
///
/// This is useful for enabling schema evolution in partitioned datasets.
///
/// This has to be done in two stages.
///
/// 1. Before reading the file, we have to map projected column indexes from the table schema to
///    the file schema.
///
/// 2. After reading a record batch we need to map the read columns back to the expected columns
///    indexes and insert null-valued columns wherever the file schema was missing a colum present
///    in the table schema.
#[derive(Clone, Debug)]
pub struct SchemaAdapter {
    /// Schema for the table
    table_schema: SchemaRef,
}

impl SchemaAdapter {
    pub fn new(table_schema: SchemaRef) -> SchemaAdapter {
        Self { table_schema }
    }

    /// Map a column index in the table schema to a column index in a particular
    /// file schema
    /// Panics if index is not in range for the table schema
    pub fn map_column_index(&self, index: usize, file_schema: &Schema) -> Option<usize> {
        let field = self.table_schema.field(index);
        file_schema.index_of(field.name()).ok()
    }

    /// Map projected column indexes to the file schema. This will fail if the table schema
    /// and the file schema contain a field with the same name and different types.
    pub fn map_projections(
        &self,
        file_schema: &Schema,
        projections: &[usize],
    ) -> DFResult<Vec<usize>> {
        let mut mapped: Vec<usize> = vec![];
        for idx in projections {
            let field = self.table_schema.field(*idx);
            if let Ok(mapped_idx) = file_schema.index_of(field.name().as_str()) {
                if file_schema.field(mapped_idx).data_type() == field.data_type() {
                    mapped.push(mapped_idx)
                } else {
                    let msg = format!("Failed to map column projection for field {}. Incompatible data types {:?} and {:?}", field.name(), file_schema.field(mapped_idx).data_type(), field.data_type());
                    info!("{}", msg);
                    return Err(DataFusionError::Execution(msg));
                }
            }
        }
        Ok(mapped)
    }

    /// Re-order projected columns by index in record batch to match table schema column ordering. If the record
    /// batch does not contain a column for an expected field, insert a null-valued column at the
    /// required column index.
    pub fn adapt_batch(&self, batch: RecordBatch, projections: &[usize]) -> DFResult<RecordBatch> {
        let batch_rows = batch.num_rows();

        let batch_schema = batch.schema();

        let mut cols: Vec<ArrayRef> = Vec::with_capacity(batch.columns().len());
        let batch_cols = batch.columns().to_vec();

        for field_idx in projections {
            let table_field = &self.table_schema.fields()[*field_idx];
            if let Some((batch_idx, _name)) =
                batch_schema.column_with_name(table_field.name().as_str())
            {
                cols.push(batch_cols[batch_idx].clone());
            } else {
                cols.push(new_null_array(table_field.data_type(), batch_rows))
            }
        }

        let projected_schema = Arc::new(self.table_schema.clone().project(projections)?);

        // Necessary to handle empty batches
        let mut options = RecordBatchOptions::default();
        options.row_count = Some(batch.num_rows());

        Ok(RecordBatch::try_new_with_options(
            projected_schema,
            cols,
            &options,
        )?)
    }
}

/// A helper that projects partition columns into the file record batches.
#[derive(Debug)]
pub struct PartitionColumnProjector {
    /// Mapping between the indexes in the list of partition columns and the target
    /// schema. Sorted by index in the target schema so that we can iterate on it to
    /// insert the partition columns in the target record batch.
    projected_partition_indexes: Vec<(usize, usize)>,
    /// The schema of the table once the projection was applied.
    projected_schema: SchemaRef,
}

impl PartitionColumnProjector {
    // Create a projector to insert the partitioning columns into batches read from files
    // - projected_schema: the target schema with both file and partitioning columns
    // - table_partition_cols: all the partitioning column names
    pub fn new(projected_schema: SchemaRef, table_partition_cols: &[String]) -> Self {
        let mut idx_map = HashMap::new();
        for (partition_idx, partition_name) in table_partition_cols.iter().enumerate() {
            if let Ok(schema_idx) = projected_schema.index_of(partition_name) {
                idx_map.insert(partition_idx, schema_idx);
            }
        }

        let mut projected_partition_indexes: Vec<_> = idx_map.into_iter().collect();
        projected_partition_indexes.sort_by(|(_, a), (_, b)| a.cmp(b));

        Self {
            projected_partition_indexes,
            projected_schema,
        }
    }

    // Transform the batch read from the file by inserting the partitioning columns
    // to the right positions as deduced from `projected_schema`
    // - file_batch: batch read from the file, with internal projection applied
    // - partition_values: the list of partition values, one for each partition column
    pub fn project(
        &mut self,
        file_batch: RecordBatch,
        partition_values: &[ScalarValue],
    ) -> ArrowResult<RecordBatch> {
        let expected_cols =
            self.projected_schema.fields().len() - self.projected_partition_indexes.len();

        if file_batch.columns().len() != expected_cols {
            return Err(ArrowError::SchemaError(format!(
                "Unexpected batch schema from file, expected {} cols but got {}",
                expected_cols,
                file_batch.columns().len()
            )));
        }

        let mut cols = file_batch.columns().to_vec();
        for &(pidx, sidx) in &self.projected_partition_indexes {
            cols.insert(
                sidx,
                partition_values[pidx].to_array_of_size(file_batch.num_rows()),
            )
        }
        RecordBatch::try_new(Arc::clone(&self.projected_schema), cols)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::build_table_i32;
    use arrow_deps::arrow::{self, datatypes::DataType};
    use arrow_deps::datafusion::{assert_batches_eq, test_util::aggr_test_schema};

    #[test]
    fn file_scan_config_no_projection() {
        let file_schema = aggr_test_schema();
        let conf = FileScanConfig {
            file_schema: Arc::clone(&file_schema),
            projection: None,
            table_partition_cols: vec![Field::new("date", DataType::Utf8, true)],
        };

        let proj_schema = conf.project();
        assert_eq!(proj_schema.fields().len(), file_schema.fields().len() + 1);
        assert_eq!(
            proj_schema.field(file_schema.fields().len()).name(),
            "date",
            "partition columns are the last columns"
        );

        let col_names = conf.projected_file_column_names();
        assert_eq!(col_names, None);

        let col_indices = conf.file_column_projection_indices();
        assert_eq!(col_indices, None);
    }

    #[test]
    fn physical_plan_config_with_projection() {
        let file_schema = aggr_test_schema();
        let conf = FileScanConfig {
            file_schema: Arc::clone(&file_schema),
            projection: Some(vec![file_schema.fields().len(), 0]),
            table_partition_cols: vec![Field::new("date", DataType::Utf8, true)],
        };

        let proj_schema = conf.project();
        assert_eq!(
            proj_schema
                .fields()
                .iter()
                .map(|f| f.name().clone())
                .collect::<Vec<String>>(),
            vec!["date".to_owned(), "c1".to_owned()]
        );

        let col_names = conf.projected_file_column_names();
        assert_eq!(col_names, Some(vec!["c1".to_owned()]));

        let col_indices = conf.file_column_projection_indices();
        assert_eq!(col_indices, Some(vec![0]));
    }

    #[test]
    fn partition_column_projector() {
        let file_batch = build_table_i32(
            ("a", &vec![0, 1, 2]),
            ("b", &vec![-2, -1, 0]),
            ("c", &vec![10, 11, 12]),
        );
        // let partition_cols = vec!["year".to_owned(), "month".to_owned(), "day".to_owned()];
        let partition_cols = vec![
            Field::new("year", DataType::Utf8, true),
            Field::new("month", DataType::Utf8, true),
            Field::new("day", DataType::Utf8, true),
        ];

        // create a projected schema
        let conf = FileScanConfig {
            file_schema: file_batch.schema(),
            // keep all cols from file and 2 from partitioning
            projection: Some(vec![
                0,
                1,
                2,
                file_batch.schema().fields().len(),
                file_batch.schema().fields().len() + 2,
            ]),
            table_partition_cols: partition_cols.clone(),
        };
        let proj_schema = conf.project();
        // created a projector for that projected schema
        let partition_col_names = partition_cols
            .iter()
            .map(|c| c.name().clone())
            .collect::<Vec<_>>();
        let mut proj = PartitionColumnProjector::new(proj_schema, &partition_col_names);

        // project first batch
        let projected_batch = proj
            .project(
                // file_batch is ok here because we kept all the file cols in the projection
                file_batch,
                &[
                    ScalarValue::Utf8(Some("2021".to_owned())),
                    ScalarValue::Utf8(Some("10".to_owned())),
                    ScalarValue::Utf8(Some("26".to_owned())),
                ],
            )
            .expect("Projection of partition columns into record batch failed");
        let expected = vec![
            "+---+----+----+------+-----+",
            "| a | b  | c  | year | day |",
            "+---+----+----+------+-----+",
            "| 0 | -2 | 10 | 2021 | 26  |",
            "| 1 | -1 | 11 | 2021 | 26  |",
            "| 2 | 0  | 12 | 2021 | 26  |",
            "+---+----+----+------+-----+",
        ];
        assert_batches_eq!(expected, &[projected_batch]);

        // project another batch that is larger than the previous one
        let file_batch = build_table_i32(
            ("a", &vec![5, 6, 7, 8, 9]),
            ("b", &vec![-10, -9, -8, -7, -6]),
            ("c", &vec![12, 13, 14, 15, 16]),
        );
        let projected_batch = proj
            .project(
                // file_batch is ok here because we kept all the file cols in the projection
                file_batch,
                &[
                    ScalarValue::Utf8(Some("2021".to_owned())),
                    ScalarValue::Utf8(Some("10".to_owned())),
                    ScalarValue::Utf8(Some("27".to_owned())),
                ],
            )
            .expect("Projection of partition columns into record batch failed");
        let expected = vec![
            "+---+-----+----+------+-----+",
            "| a | b   | c  | year | day |",
            "+---+-----+----+------+-----+",
            "| 5 | -10 | 12 | 2021 | 27  |",
            "| 6 | -9  | 13 | 2021 | 27  |",
            "| 7 | -8  | 14 | 2021 | 27  |",
            "| 8 | -7  | 15 | 2021 | 27  |",
            "| 9 | -6  | 16 | 2021 | 27  |",
            "+---+-----+----+------+-----+",
        ];
        assert_batches_eq!(expected, &[projected_batch]);

        // project another batch that is smaller than the previous one
        let file_batch = build_table_i32(
            ("a", &vec![0, 1, 3]),
            ("b", &vec![2, 3, 4]),
            ("c", &vec![4, 5, 6]),
        );
        let projected_batch = proj
            .project(
                // file_batch is ok here because we kept all the file cols in the projection
                file_batch,
                &[
                    ScalarValue::Utf8(Some("2021".to_owned())),
                    ScalarValue::Utf8(Some("10".to_owned())),
                    ScalarValue::Utf8(Some("28".to_owned())),
                ],
            )
            .expect("Projection of partition columns into record batch failed");
        let expected = vec![
            "+---+---+---+------+-----+",
            "| a | b | c | year | day |",
            "+---+---+---+------+-----+",
            "| 0 | 2 | 4 | 2021 | 28  |",
            "| 1 | 3 | 5 | 2021 | 28  |",
            "| 3 | 4 | 6 | 2021 | 28  |",
            "+---+---+---+------+-----+",
        ];
        assert_batches_eq!(expected, &[projected_batch]);
    }

    #[test]
    fn schema_adapter_adapt_projections() {
        let table_schema = Arc::new(Schema::new(vec![
            Field::new("c1", DataType::Utf8, true),
            Field::new("c2", DataType::Int64, true),
            Field::new("c3", DataType::Int8, true),
        ]));

        let file_schema = Schema::new(vec![
            Field::new("c1", DataType::Utf8, true),
            Field::new("c2", DataType::Int64, true),
        ]);

        let file_schema_2 = Arc::new(Schema::new(vec![
            Field::new("c3", DataType::Int8, true),
            Field::new("c2", DataType::Int64, true),
        ]));

        let file_schema_3 = Arc::new(Schema::new(vec![Field::new("c3", DataType::Float32, true)]));

        let adapter = SchemaAdapter::new(table_schema);

        let projections1: Vec<usize> = vec![0, 1, 2];
        let projections2: Vec<usize> = vec![2];

        let mapped = adapter
            .map_projections(&file_schema, projections1.as_slice())
            .expect("mapping projections");

        assert_eq!(mapped, vec![0, 1]);

        let mapped = adapter
            .map_projections(&file_schema, projections2.as_slice())
            .expect("mapping projections");

        assert!(mapped.is_empty());

        let mapped = adapter
            .map_projections(&file_schema_2, projections1.as_slice())
            .expect("mapping projections");

        assert_eq!(mapped, vec![1, 0]);

        let mapped = adapter
            .map_projections(&file_schema_2, projections2.as_slice())
            .expect("mapping projections");

        assert_eq!(mapped, vec![0]);

        let mapped = adapter.map_projections(&file_schema_3, projections1.as_slice());

        assert!(mapped.is_err());
    }
}

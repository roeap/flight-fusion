use super::{DoGetHandler, DoPutHandler};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
    stream::MergeStream,
};
use area_store::{
    projection::{PartitionColumnProjector, SchemaAdapter},
    store::{AreaPath, AreaStore, StorageLocation},
    Path,
};
use arrow_deps::arrow::{
    datatypes::{DataType, Field as ArrowField, SchemaRef as ArrowSchemaRef},
    error::{ArrowError, Result as ArrowResult},
    record_batch::RecordBatch,
};
use arrow_deps::datafusion::{
    physical_plan::{
        common::{collect, AbortOnDropMany},
        SendableRecordBatchStream,
    },
    scalar::ScalarValue,
};
use arrow_deps::deltalake::{action::SaveMode as DeltaSaveMode, operations::DeltaCommands};
use async_trait::async_trait;
use flight_fusion_ipc::{
    delta_operation_request::Operation as DeltaOperation, DeltaOperationRequest,
    DeltaOperationResponse, DeltaReadOperation, SaveMode,
};
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[async_trait]
impl DoPutHandler<DeltaOperationRequest> for FlightFusionService {
    async fn handle_do_put(
        &self,
        ticket: DeltaOperationRequest,
        input: SendableRecordBatchStream,
    ) -> Result<DeltaOperationResponse> {
        if let Some(source) = ticket.source {
            let area_path = AreaPath::from(&source);
            if let StorageLocation::Local(path) = &self.area_store.root {
                let table_path = path.as_path().join(area_path.as_ref());
                std::fs::create_dir_all(&table_path)
                    .map_err(|err| FusionServiceError::Generic(err.to_string()))?;
            };
            let mut commands: DeltaCommands =
                match self.area_store.open_delta(&source.clone().into()).await {
                    Ok(table) => table.into(),
                    Err(_) => self
                        .area_store
                        .open_delta_uninitialized(&source.into())
                        .await?
                        .into(),
                };

            let batches = collect(input).await?;

            match ticket.operation {
                Some(DeltaOperation::Write(req)) => {
                    let mode = match SaveMode::from_i32(req.save_mode) {
                        Some(SaveMode::Append) => DeltaSaveMode::Append,
                        Some(SaveMode::Overwrite) => DeltaSaveMode::Overwrite,
                        Some(SaveMode::ErrorIfExists) => DeltaSaveMode::ErrorIfExists,
                        _ => todo!(),
                    };
                    commands
                        .write(batches, mode, Some(req.partition_by))
                        .await?;
                }
                _ => todo!(),
            };
            Ok(DeltaOperationResponse::default())
        } else {
            // TODO migrate errors and raise something more meaningful
            Err(FusionServiceError::generic("Source not found"))
        }
    }
}

fn to_scalar_value(field: &ArrowField, serialized_value: &Option<String>) -> Result<ScalarValue> {
    let value = serialized_value.clone().unwrap();
    match field.data_type() {
        DataType::Utf8 => Ok(ScalarValue::Utf8(Some(value))),
        DataType::Int16 => Ok(ScalarValue::Int16(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Int32 => Ok(ScalarValue::Int32(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Int64 => Ok(ScalarValue::Int64(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Float32 => Ok(ScalarValue::Float32(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Float64 => Ok(ScalarValue::Float64(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Date32 => Ok(ScalarValue::Date32(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        DataType::Date64 => Ok(ScalarValue::Date64(Some(value.parse().map_err(|_| {
            FusionServiceError::DataConversion("failed converting to ScalarValue".to_string())
        })?))),
        _ => todo!(
            "Conversion for datatype not implemented: {:}",
            field.data_type()
        ),
    }
}

#[async_trait]
impl DoGetHandler<DeltaOperationRequest> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: DeltaOperationRequest,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(source) = ticket.source {
            let table = self.area_store.open_delta(&source.into()).await?;
            let files = table
                .get_file_uris()
                .zip(table.get_partition_values())
                .collect::<Vec<_>>();

            let schema: ArrowSchemaRef = Arc::new(table.get_schema()?.try_into()?);

            let column_indices = if let Some(operation) = ticket.operation {
                match operation {
                    DeltaOperation::Read(DeltaReadOperation { column_names, .. }) => {
                        if column_names.is_empty() {
                            None
                        } else {
                            Some(
                                column_names
                                    .iter()
                                    .map(|c| schema.index_of(c))
                                    .collect::<std::result::Result<Vec<_>, _>>()?,
                            )
                        }
                    }
                    _ => unimplemented!("unexpected operation"),
                }
            } else {
                None
            };

            let (sender, receiver) = mpsc::channel::<ArrowResult<RecordBatch>>(files.len());
            let mut join_handles = Vec::with_capacity(files.len());

            for (file_path, partition_values) in files {
                let path = self.area_store.get_path_from_raw(file_path);
                join_handles.push(spawn_execution(
                    sender.clone(),
                    self.area_store.clone(),
                    path,
                    schema.clone(),
                    partition_values.clone(),
                    column_indices.clone(),
                ));
            }

            let projected_schema = match column_indices {
                Some(indices) => Arc::new(schema.project(&indices)?),
                None => schema,
            };
            Ok(Box::pin(MergeStream::new(
                projected_schema,
                receiver,
                AbortOnDropMany(join_handles),
            )))
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
}

pub(crate) fn spawn_execution(
    mut output: mpsc::Sender<ArrowResult<RecordBatch>>,
    area_store: Arc<AreaStore>,
    path: Path,
    table_schema: ArrowSchemaRef,
    partition_values: HashMap<String, Option<String>>,
    column_indices: Option<Vec<usize>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = match area_store.open_file(&path.into(), None).await {
            Err(e) => {
                // If send fails, plan being torn
                // down, no place to send the error
                let arrow_error = ArrowError::ExternalError(Box::new(e));
                output.send(Err(arrow_error)).await.ok();
                return;
            }
            Ok(stream) => stream,
        };

        let adapter = SchemaAdapter::new(table_schema.clone());

        while let Some(item) = stream.next().await {
            let file_batch = if let Some(indices) = &column_indices {
                // TODO remove panics
                let mapped = adapter.map_projections(&stream.schema(), indices).unwrap();
                item.unwrap().project(&mapped).unwrap()
            } else {
                item.unwrap()
            };

            let table_partition_cols = partition_values.keys().cloned().collect::<Vec<_>>();
            let projected_schema = match &column_indices {
                Some(indices) => Arc::new(table_schema.project(indices).unwrap()),
                None => table_schema.clone(),
            };
            let mut proj = PartitionColumnProjector::new(projected_schema, &table_partition_cols);

            let mut partition_scalars = vec![];
            for (key, val) in partition_values.iter() {
                let field = table_schema.field_with_name(key).unwrap();
                // TODO remove panics
                let scalar = to_scalar_value(field, val).unwrap();
                partition_scalars.push(scalar);
            }

            let new_batch = proj.project(file_batch, &partition_scalars);

            output.send(new_batch).await.ok();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_fusion_handler, get_input_stream};
    use arrow_deps::deltalake::open_table;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, delta_operation_request::Operation,
        AreaSourceReference, AreaTableLocation, DeltaOperationRequest, DeltaReadOperation,
        DeltaWriteOperation, SaveMode,
    };

    #[tokio::test]
    async fn test_put_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let handler = get_fusion_handler(root.path());
        let table_dir = root.path().join("_ff_data/new_table");

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["modified".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();
        let mut dt = open_table(table_dir.to_str().unwrap()).await.unwrap();
        assert_eq!(dt.version(), 0);
        assert_eq!(dt.get_file_uris().count(), 2);

        // Append data to table
        let plan = get_input_stream(None, false);
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version(), 1);
        assert_eq!(dt.get_file_uris().count(), 4);

        // Overwrite table
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference { table: Some(table) }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Overwrite.into(),
                partition_by: vec!["modified".to_string()],
                ..Default::default()
            })),
        };
        let plan = get_input_stream(None, false);
        let _ = handler.handle_do_put(request, plan).await.unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version(), 2);
        assert_eq!(dt.get_file_uris().count(), 2);
    }

    #[tokio::test]
    async fn test_read_table() {
        let root = tempfile::tempdir().unwrap();
        let path = root.path();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(path);

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["modified".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();

        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Read(DeltaReadOperation::default())),
        };

        let data_stream = handler.execute_do_get(request).await.unwrap();
        let data = arrow_deps::datafusion::physical_plan::common::collect(data_stream)
            .await
            .unwrap();

        assert_eq!(data[0].schema(), ref_schema)
    }

    #[tokio::test]
    async fn test_read_table_partitioned() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["id".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();

        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Read(DeltaReadOperation::default())),
        };

        let data_stream = handler.execute_do_get(request).await.unwrap();
        let data = arrow_deps::datafusion::physical_plan::common::collect(data_stream)
            .await
            .unwrap();

        assert_eq!(data[0].schema(), ref_schema)
    }

    #[tokio::test]
    async fn test_read_table_columns() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["modified".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();

        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Read(DeltaReadOperation {
                column_names: vec!["id".to_string(), "modified".to_string()],
                ..DeltaReadOperation::default()
            })),
        };

        let data_stream = handler.execute_do_get(request).await.unwrap();
        let data = arrow_deps::datafusion::physical_plan::common::collect(data_stream)
            .await
            .unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0, 2]).unwrap())
        )
    }

    #[tokio::test]
    async fn test_read_table_columns_no_partition() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["modified".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();

        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Read(DeltaReadOperation {
                column_names: vec!["id".to_string()],
                ..DeltaReadOperation::default()
            })),
        };

        let data_stream = handler.execute_do_get(request).await.unwrap();
        let data = arrow_deps::datafusion::physical_plan::common::collect(data_stream)
            .await
            .unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0]).unwrap())
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_read_table_columns_partition_only() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append.into(),
                partition_by: vec!["id".to_string()],
                ..Default::default()
            })),
        };

        // create table and write some data
        let _ = handler.handle_do_put(request.clone(), plan).await.unwrap();

        let request = DeltaOperationRequest {
            source: Some(AreaSourceReference {
                table: Some(table.clone()),
            }),
            operation: Some(Operation::Read(DeltaReadOperation {
                column_names: vec!["id".to_string()],
                ..DeltaReadOperation::default()
            })),
        };

        let data_stream = handler.execute_do_get(request).await.unwrap();
        let data = arrow_deps::datafusion::physical_plan::common::collect(data_stream)
            .await
            .unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0]).unwrap())
        )
    }
}

use super::{DoGetHandler, DoPutHandler};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
    stream::MergeStream,
};
use area_store::store::{AreaStore, DefaultAreaStore};
use arrow_deps::arrow::{
    datatypes::{Field as ArrowField, SchemaRef as ArrowSchemaRef},
    error::{ArrowError, Result as ArrowResult},
    record_batch::RecordBatch,
};
use arrow_deps::datafusion::{
    physical_plan::{collect, common::AbortOnDropMany, ExecutionPlan, SendableRecordBatchStream},
    prelude::SessionContext,
    scalar::ScalarValue,
};
use arrow_deps::deltalake::{
    action::SaveMode as DeltaSaveMode, open_table, operations::DeltaCommands,
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    delta_operation_request::Operation as DeltaOperation, DeltaOperationRequest,
    DeltaOperationResponse, SaveMode,
};
use futures::channel::mpsc;
use futures::{SinkExt, StreamExt};
use object_store::path::Path;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinHandle;

#[async_trait]
impl DoPutHandler<DeltaOperationRequest> for FlightFusionService {
    async fn handle_do_put(
        &self,
        ticket: DeltaOperationRequest,
        input: Arc<dyn ExecutionPlan>,
    ) -> Result<DeltaOperationResponse> {
        if let Some(source) = ticket.source {
            let full_path = self.area_store.get_full_table_path(&source)?;
            let mut delta_cmd = DeltaCommands::try_from_uri(full_path).await?;
            let session_ctx = SessionContext::new();
            let task_ctx = session_ctx.task_ctx();
            let batches = collect(input, task_ctx).await?;

            match ticket.operation {
                Some(DeltaOperation::Write(req)) => {
                    let mode = match SaveMode::from_i32(req.save_mode) {
                        Some(SaveMode::Append) => DeltaSaveMode::Append,
                        Some(SaveMode::Overwrite) => DeltaSaveMode::Overwrite,
                        Some(SaveMode::ErrorIfExists) => DeltaSaveMode::ErrorIfExists,
                        _ => todo!(),
                    };
                    delta_cmd
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

fn to_scalar_value(field: &ArrowField, serialized_value: &Option<String>) -> ScalarValue {
    println!("FIELD TYPE -> {:?}", field);
    ScalarValue::Utf8(serialized_value.to_owned())
}

#[async_trait]
impl DoGetHandler<DeltaOperationRequest> for FlightFusionService {
    async fn execute_do_get(
        &self,
        ticket: DeltaOperationRequest,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(source) = ticket.source {
            let full_path = self.area_store.get_full_table_path(&source)?;
            let table = open_table(&full_path).await?;
            let files = table
                .get_file_uris()
                .zip(table.get_partition_values())
                .collect::<Vec<_>>();

            let schema: ArrowSchemaRef = Arc::new(table.get_schema()?.try_into()?);

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
                ));
            }
            Ok(Box::pin(MergeStream::new(
                schema,
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
    area_store: Arc<DefaultAreaStore>,
    path: Path,
    projected_schema: ArrowSchemaRef,
    // partition_cols: Vec<String>,
    partition_values: HashMap<String, Option<String>>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut stream = match area_store.open_file(&path).await {
            Err(e) => {
                // If send fails, plan being torn
                // down, no place to send the error
                let arrow_error = ArrowError::ExternalError(Box::new(e));
                output.send(Err(arrow_error)).await.ok();
                return;
            }
            Ok(stream) => stream,
        };
        while let Some(item) = stream.next().await {
            let file_batch = item.unwrap();
            let mut cols = file_batch.columns().to_vec();

            for (key, val) in partition_values.iter() {
                let field = projected_schema.field_with_name(key).unwrap();
                let scalar = to_scalar_value(field, val);
                cols.push(scalar.to_array_of_size(file_batch.num_rows()));
            }

            let new_batch = RecordBatch::try_new(Arc::clone(&projected_schema), cols);
            output.send(new_batch).await.ok();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_fusion_handler, get_input_plan};
    use arrow_deps::deltalake::open_table;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, delta_operation_request::Operation,
        AreaSourceReference, AreaTableLocation, DeltaOperationRequest, DeltaReadOperation,
        DeltaWriteOperation, SaveMode,
    };

    #[tokio::test]
    async fn test_put_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
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
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let mut dt = open_table(table_dir.to_str().unwrap()).await.unwrap();
        assert_eq!(dt.version, 0);
        assert_eq!(dt.get_file_uris().count(), 2);

        // Append data to table
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version, 1);
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
        let _ = handler.handle_do_put(request, plan).await.unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version, 2);
        assert_eq!(dt.get_file_uris().count(), 2);
    }

    #[tokio::test]
    async fn test_read_table() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
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
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();

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

        assert_eq!(data[0].schema(), plan.schema())
    }
}

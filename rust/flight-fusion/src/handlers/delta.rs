use super::{utils::create_response_stream, *};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use arrow_deps::datafusion::physical_plan::{collect, ExecutionPlan};
use arrow_deps::deltalake::{
    action::SaveMode as DeltaSaveMode, open_table, operations::DeltaCommands,
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    delta_operation_request::Operation as DeltaOperation, DeltaOperationRequest,
    DeltaOperationResponse, SaveMode,
};
use std::sync::Arc;

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

#[async_trait]
impl DoGetHandler<DeltaOperationRequest> for FlightFusionService {
    async fn handle_do_get(
        &self,
        ticket: DeltaOperationRequest,
    ) -> Result<BoxedFlightStream<FlightData>> {
        if let Some(source) = ticket.source {
            let full_path = self.area_store.get_full_table_path(&source)?;
            let table = open_table(&full_path).await?;
            let files = table.get_file_uris().collect::<Vec<_>>();
            let mut batches = Vec::new();
            for file in files {
                let path = self.area_store.get_path_from_raw(file);
                batches.append(&mut self.area_store.read_file(&path).await?);
            }
            create_response_stream(batches).await
        } else {
            Err(FusionServiceError::InputError(
                "missing table reference".to_string(),
            ))
        }
    }
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

        // read table
        let data = handler.handle_do_get(request).await.unwrap();
    }
}

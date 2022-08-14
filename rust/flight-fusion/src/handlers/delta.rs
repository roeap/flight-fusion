use super::{DoGetHandler, DoPutHandler};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::{storage_url::StorageService, store::AreaPath};

use arrow_deps::datafusion::{
    physical_plan::{common::collect, SendableRecordBatchStream},
    prelude::SessionContext,
};
use arrow_deps::deltalake::{action::SaveMode as DeltaSaveMode, operations::DeltaCommands};
use async_trait::async_trait;
use flight_fusion_ipc::{
    area_source_reference::Table, delta_operation_request::Operation as DeltaOperation,
    DeltaOperationRequest, DeltaOperationResponse, SaveMode,
};
use observability_deps::{
    instrument,
    tracing::{self, debug},
};

#[async_trait]
impl DoPutHandler<DeltaOperationRequest> for FlightFusionService {
    #[instrument(skip(self, input))]
    async fn handle_do_put(
        &self,
        ticket: DeltaOperationRequest,
        input: SendableRecordBatchStream,
    ) -> Result<DeltaOperationResponse> {
        if let Some(source) = ticket.source {
            let area_path = AreaPath::from(&source);
            if let StorageService::Local(path) = self.area_store.root.clone().into() {
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

#[async_trait]
impl DoGetHandler<DeltaOperationRequest> for FlightFusionService {
    #[instrument(skip(self, ticket))]
    async fn execute_do_get(
        &self,
        ticket: DeltaOperationRequest,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(source) = ticket.source {
            let mut ctx = SessionContext::new();
            self.register_source(&mut ctx, &source).await?;
            let tbl_loc = source
                .table
                .ok_or_else(|| FusionServiceError::Generic("missing table name".to_string()))?;
            let columns = if let Some(DeltaOperation::Read(op)) = ticket.operation {
                if op.column_names.is_empty() {
                    "*".into()
                } else {
                    op.column_names.join(", ")
                }
            } else {
                "*".into()
            };
            match tbl_loc {
                Table::Location(tbl) => {
                    let query = format!("SELECT {} FROM {}", columns, tbl.name);
                    debug!("Executing query: {}", query);
                    Ok(ctx.sql(&query).await?.execute_stream().await?)
                }
                _ => todo!(),
            }
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
                partition_by: vec!["id".to_string()],
                // partition_by: vec![],
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
        assert_eq!(data[0].schema().fields().len(), ref_schema.fields().len())
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

        assert_eq!(data[0].schema().fields().len(), ref_schema.fields().len())
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
                // partition_by: vec!["modified".to_string()],
                partition_by: vec![],
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
                partition_by: vec!["value".to_string()],
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

        println!("{:?}", data[0].schema());

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0]).unwrap())
        )
    }
}

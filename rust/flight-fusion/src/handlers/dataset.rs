use super::{DoGetHandler, DoPutHandler};
use crate::{
    error::{FusionServiceError, Result},
    service::FlightFusionService,
};
use area_store::store::AreaPath;
use arrow_deps::datafusion::{physical_plan::SendableRecordBatchStream, prelude::SessionContext};
use async_trait::async_trait;
use flight_fusion_ipc::{
    area_source_reference::Table, CommandReadDataset, CommandWriteIntoDataset, ResultDoPutUpdate,
    SaveMode,
};
use observability_deps::{
    instrument,
    tracing::{self, debug},
};

#[async_trait]
impl DoPutHandler<CommandWriteIntoDataset> for FlightFusionService {
    #[instrument(skip(self, input, ticket))]
    async fn handle_do_put(
        &self,
        ticket: CommandWriteIntoDataset,
        input: SendableRecordBatchStream,
    ) -> Result<ResultDoPutUpdate> {
        if let Some(source) = ticket.source {
            let location: AreaPath = source.into();
            let _adds = self
                .area_store
                .put_batches(
                    input,
                    &location.into(),
                    SaveMode::from_i32(ticket.save_mode).unwrap_or(SaveMode::Overwrite),
                )
                .await?;

            Ok(ResultDoPutUpdate { statistics: None })
        } else {
            // TODO migrate errors and raise something more meaningful
            Err(FusionServiceError::generic("Source not found"))
        }
    }
}

#[async_trait]
impl DoGetHandler<CommandReadDataset> for FlightFusionService {
    #[instrument(skip(self, ticket))]
    async fn execute_do_get(
        &self,
        ticket: CommandReadDataset,
    ) -> Result<SendableRecordBatchStream> {
        if let Some(table) = ticket.source {
            let mut ctx = SessionContext::new();
            self.register_source(&mut ctx, &table).await?;
            let tbl_loc = table
                .table
                .ok_or_else(|| FusionServiceError::Generic("missing table name".to_string()))?;
            let columns = if ticket.column_names.is_empty() {
                "*".into()
            } else {
                ticket.column_names.join(", ")
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
    use crate::handlers::DoGetHandler;
    use crate::handlers::DoPutHandler;
    use crate::test_utils::{get_fusion_handler, get_input_stream};
    use arrow_deps::datafusion::physical_plan::common::collect;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        CommandReadDataset, CommandWriteIntoDataset, SaveMode,
    };

    #[tokio::test]
    async fn test_put_table() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let handler = get_fusion_handler(root.path());
        let table_dir = root.path().join("_ff_data/new_table");

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let request = CommandWriteIntoDataset {
            source: Some(AreaSourceReference { table: Some(table) }),
            save_mode: SaveMode::Overwrite.into(),
        };

        assert!(!table_dir.exists());

        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();

        assert!(table_dir.is_dir())
    }

    #[tokio::test]
    async fn test_put_table_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let handler = get_fusion_handler(root.path());
        let table_dir = root.path().join("_ff_data/new_table");

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };
        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Append.into(),
        };

        assert!(!table_dir.exists());

        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();

        assert!(table_dir.is_dir());

        let paths = std::fs::read_dir(&table_dir)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(paths.len(), 1);

        let plan = get_input_stream(None, false);
        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();
        let paths = std::fs::read_dir(&table_dir)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(paths.len(), 2);

        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Overwrite.into(),
        };

        let plan = get_input_stream(None, false);
        let _response = handler.handle_do_put(request.clone(), plan).await.unwrap();
        let paths = std::fs::read_dir(&table_dir)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert!(paths.len() == 1)
    }

    #[tokio::test]
    async fn test_put_get() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());
        // let table_dir = root.path().join("_ff_data/new_table");

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };

        let put_request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Append.into(),
        };

        let _ = handler
            .handle_do_put(put_request.clone(), plan)
            .await
            .unwrap();

        let get_request = CommandReadDataset {
            source: Some(table_ref.clone()),
            column_names: vec![],
            ..CommandReadDataset::default()
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(data[0].schema(), ref_schema)
    }

    #[tokio::test]
    async fn test_get_columns() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_stream(None, false);
        let ref_schema = plan.schema().clone();
        let handler = get_fusion_handler(root.path());

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };

        let put_request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Append.into(),
        };

        let _ = handler
            .handle_do_put(put_request.clone(), plan)
            .await
            .unwrap();

        let cols = ref_schema
            .fields()
            .iter()
            .map(|c| c.name().clone())
            .collect::<Vec<_>>();

        let get_request = CommandReadDataset {
            source: Some(table_ref.clone()),
            column_names: vec![cols[0].clone()],
        };

        let response = handler.execute_do_get(get_request.clone()).await.unwrap();
        let data = collect(response).await.unwrap();

        assert_eq!(
            data[0].schema(),
            std::sync::Arc::new(ref_schema.project(&[0]).unwrap())
        )
    }
}

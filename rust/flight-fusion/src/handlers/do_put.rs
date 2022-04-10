use super::*;
use crate::error::{FusionServiceError, Result};
use area_store::store::AreaStore;
use arrow_deps::datafusion::physical_plan::{collect, ExecutionPlan};
use async_trait::async_trait;
use flight_fusion_ipc::{
    flight_do_put_request::Command as DoPutCommand, CommandWriteIntoDataset, ResultDoPutUpdate,
    SaveMode,
};
use std::sync::Arc;

impl FusionActionHandler {
    pub async fn execute_do_put(
        &self,
        stream: Arc<FlightReceiverPlan>,
    ) -> Result<BoxedFlightStream<PutResult>> {
        let request_data = stream.ticket();
        let body = match &request_data.command {
            Some(action) => {
                let result_body = match action {
                    DoPutCommand::Storage(storage) => {
                        serialize_message(self.handle_do_put(storage.clone(), stream).await?)
                    }
                    DoPutCommand::Delta(delta) => {
                        serialize_message(self.handle_do_put(delta.clone(), stream).await?)
                    }
                };

                Ok(result_body)
            }
            None => Err(FusionServiceError::unknown_action("No action data passed")),
        }?;

        let result = vec![Ok(PutResult { app_metadata: body })];
        Ok(Box::pin(futures::stream::iter(result)) as BoxedFlightStream<PutResult>)
    }
}

// #[async_trait]
// impl DoPutHandler<PutMemoryTableRequest> for FusionActionHandler {
//     async fn handle_do_put(
//         &self,
//         ticket: PutMemoryTableRequest,
//         input: Arc<dyn ExecutionPlan>,
//     ) -> Result<ResultDoPutUpdate> {
//         let schema_ref = input.schema();
//         let batches = collect(input, Arc::new(RuntimeEnv::default())).await?;
//
//         // register received schema
//         let table_provider = MemTable::try_new(schema_ref.clone(), vec![batches])?;
//         let schema_provider = self.catalog.schema("schema");
//         schema_provider.register_table(ticket.name, Arc::new(table_provider))?;
//
//         self.catalog
//             .register_schema("schema".to_string(), schema_provider);
//
//         // TODO generate messages in channel
//         // https://github.com/jorgecarleitao/arrow2/blob/main/integration-testing/src/flight_server_scenarios/integration_test.rs
//         Ok(ResultDoPutUpdate { statistics: None })
//     }
// }

#[async_trait]
impl DoPutHandler<CommandWriteIntoDataset> for FusionActionHandler {
    async fn handle_do_put(
        &self,
        ticket: CommandWriteIntoDataset,
        input: Arc<dyn ExecutionPlan>,
    ) -> Result<ResultDoPutUpdate> {
        if let Some(source) = ticket.source {
            let location = self.area_store.get_table_location(&source)?;
            let session_ctx = SessionContext::new();
            let task_ctx = session_ctx.task_ctx();
            let batches = collect(input, task_ctx).await?;
            let _adds = self
                .area_store
                .put_batches(
                    batches,
                    &location,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_fusion_handler, get_input_plan};
    use area_store::store::flatten_list_stream;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        SaveMode,
    };

    #[tokio::test]
    async fn test_put_table() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
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

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();

        assert!(table_dir.is_dir())
    }

    #[tokio::test]
    async fn test_put_table_append_overwrite() {
        let root = tempfile::tempdir().unwrap();
        let plan = get_input_plan(None, false);
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

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();

        assert!(table_dir.is_dir());

        let table_location = handler.area_store.get_table_location(&table_ref).unwrap();
        let files = flatten_list_stream(&handler.area_store.object_store(), Some(&table_location))
            .await
            .unwrap();
        assert!(files.len() == 1);

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let files = flatten_list_stream(&handler.area_store.object_store(), Some(&table_location))
            .await
            .unwrap();
        assert!(files.len() == 2);

        let request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Overwrite.into(),
        };

        let _response = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let files = flatten_list_stream(&handler.area_store.object_store(), Some(&table_location))
            .await
            .unwrap();
        assert!(files.len() == 1)
    }
}

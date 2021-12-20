use super::*;
use async_trait::async_trait;
use datafusion::physical_plan::collect;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::{catalog::catalog::CatalogProvider, datasource::MemTable};
use deltalake::{action::SaveMode as DeltaSaveMode, commands::DeltaCommands};
use flight_fusion_ipc::{
    delta_operation_request, DeltaOperationRequest, DeltaOperationResponse, FlightFusionError,
    PutMemoryTableRequest, PutMemoryTableResponse, Result as FusionResult,
    SaveMode as FusionSaveMode,
};
use std::sync::Arc;

impl FusionActionHandler {
    pub async fn execute_do_put(
        &self,
        stream: Arc<FlightReceiverPlan>,
    ) -> FusionResult<BoxedFlightStream<PutResult>> {
        let request_data = stream.ticket();
        let body = match &request_data.operation {
            Some(action) => {
                let result_body = match action {
                    DoPutOperation::Memory(memory) => {
                        serialize_message(self.handle_do_put(memory.clone(), stream).await?)
                    }
                    DoPutOperation::Remote(_remote) => {
                        todo!()
                    }
                    DoPutOperation::Delta(req) => {
                        serialize_message(self.handle_do_put(req.clone(), stream).await?)
                    }
                };

                Ok(result_body)
            }
            None => Err(FlightFusionError::UnknownAction(
                "No action data passed".to_string(),
            )),
        }?;

        let result = vec![Ok(PutResult { app_metadata: body })];
        Ok(Box::pin(futures::stream::iter(result)) as BoxedFlightStream<PutResult>)
    }
}

#[async_trait]
impl DoPutHandler<PutMemoryTableRequest> for FusionActionHandler {
    async fn handle_do_put(
        &self,
        ticket: PutMemoryTableRequest,
        input: Arc<dyn ExecutionPlan>,
    ) -> FusionResult<PutMemoryTableResponse> {
        let schema_ref = input.schema();
        let batches = collect(input).await.unwrap();

        // register received schema
        let table_provider = MemTable::try_new(schema_ref.clone(), vec![batches]).unwrap();
        let schema_provider = self.catalog.schema("schema").unwrap();
        schema_provider
            .register_table(ticket.name, Arc::new(table_provider))
            .unwrap();

        self.catalog
            .register_schema("schema".to_string(), schema_provider);

        // TODO generate messages in channel
        // https://github.com/jorgecarleitao/arrow2/blob/main/integration-testing/src/flight_server_scenarios/integration_test.rs
        Ok(PutMemoryTableResponse {
            name: "created".to_string(),
        })
    }
}

#[async_trait]
impl DoPutHandler<DeltaOperationRequest> for FusionActionHandler {
    async fn handle_do_put(
        &self,
        ticket: DeltaOperationRequest,
        input: Arc<dyn ExecutionPlan>,
    ) -> FusionResult<DeltaOperationResponse> {
        let table_uri = ticket.table.expect("Table reference must be defined");
        let mut delta_cmd = DeltaCommands::try_from_uri(table_uri.location)
            .await
            .unwrap();
        let batches = collect(input).await.unwrap();

        match ticket.operation {
            Some(delta_operation_request::Operation::Write(req)) => {
                let mode = match FusionSaveMode::from_i32(req.save_mode) {
                    Some(FusionSaveMode::Append) => DeltaSaveMode::Append,
                    Some(FusionSaveMode::Overwrite) => DeltaSaveMode::Overwrite,
                    Some(FusionSaveMode::ErrorIfExists) => DeltaSaveMode::ErrorIfExists,
                    _ => todo!(),
                };
                delta_cmd
                    .write(batches, Some(mode), Some(req.partition_columns))
                    .await
                    .unwrap();
            }
            _ => todo!(),
        };

        Ok(DeltaOperationResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{generate_random_batch, get_fusion_handler, get_record_batch};
    use datafusion::{
        arrow::datatypes::{DataType, Field, Schema as ArrowSchema},
        physical_plan::memory::MemoryExec,
    };
    use deltalake::open_table;
    use flight_fusion_ipc::{
        delta_operation_request::Operation, DeltaOperationRequest, DeltaReference,
        DeltaWriteOperation, SaveMode,
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_put_delta() {
        let batch = get_record_batch(None, false);
        let schema = batch.schema().clone();
        let plan =
            Arc::new(MemoryExec::try_new(&[vec![batch.clone()]], schema.clone(), None).unwrap());

        let table_dir = tempfile::tempdir().unwrap();
        let table_path = table_dir.path();
        let table_uri = table_path.to_str().unwrap().to_string();

        let request = DeltaOperationRequest {
            table: Some(DeltaReference {
                location: table_uri.clone(),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append as i32,
                partition_columns: vec!["modified".to_string()],
                ..Default::default()
            })),
        };

        let handler = get_fusion_handler();

        // create table and write some data
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let mut dt = open_table(&table_uri).await.unwrap();
        assert_eq!(dt.version, 0);
        assert_eq!(dt.get_file_uris().len(), 2);

        // Append data to table
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version, 1);
        assert_eq!(dt.get_file_uris().len(), 4);

        // Overwrite table
        let request = DeltaOperationRequest {
            table: Some(DeltaReference {
                location: table_uri.clone(),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Overwrite as i32,
                partition_columns: vec!["modified".to_string()],
                ..Default::default()
            })),
        };
        let _ = handler.handle_do_put(request, plan).await.unwrap();
        dt.update().await.unwrap();
        assert_eq!(dt.version, 2);
        assert_eq!(dt.get_file_uris().len(), 2);
    }

    #[tokio::test]
    async fn asd() {
        let schema = Arc::new(ArrowSchema::new(vec![
            Field::new("col1", DataType::Float64, true),
            Field::new("col2", DataType::Float64, true),
            Field::new("col3", DataType::Float64, true),
        ]));
        let row_count = 4;

        let batch = generate_random_batch(row_count, schema.clone());
        let plan =
            Arc::new(MemoryExec::try_new(&[vec![batch.clone()]], schema.clone(), None).unwrap());

        let table_dir = tempfile::tempdir().unwrap();
        let table_path = table_dir.path();
        let table_uri = table_path.to_str().unwrap().to_string();

        let request = DeltaOperationRequest {
            table: Some(DeltaReference {
                location: table_uri.clone(),
            }),
            operation: Some(Operation::Write(DeltaWriteOperation {
                save_mode: SaveMode::Append as i32,
                partition_columns: vec![],
                ..Default::default()
            })),
        };

        let handler = get_fusion_handler();
        let _ = handler
            .handle_do_put(request.clone(), plan.clone())
            .await
            .unwrap();
        let mut dt = open_table(&table_uri).await.unwrap();
        assert_eq!(dt.version, 0);
    }
}

use super::*;
use async_trait::async_trait;
use datafusion::physical_plan::collect;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::{catalog::catalog::CatalogProvider, datasource::MemTable};
use deltalake::commands::DeltaCommands;
use flight_fusion_ipc::{
    DeltaOperationRequest, DeltaOperationResponse, FlightFusionError, PutMemoryTableRequest,
    PutMemoryTableResponse, Result as FusionResult,
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
        let mut delta_cmd = DeltaCommands::try_from_uri(table_uri.location).await.unwrap();
        let batches = collect(input).await.unwrap();

        delta_cmd
            .write(batches, None, Some(vec![]))
            .await
            .unwrap();

        Ok(DeltaOperationResponse::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{get_fusion_handler, get_record_batch_stream};

    #[tokio::test]
    async fn test_put_delta() {
        let handler = get_fusion_handler();
        let stream = get_record_batch_stream();
    }
}

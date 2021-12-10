use super::*;
use arrow_flight::FlightData;
use async_trait::async_trait;
use datafusion::{
    arrow::datatypes::Schema, catalog::catalog::CatalogProvider, datasource::MemTable,
};
use flight_fusion_ipc::{
    FlightFusionError, PutMemoryTableRequest, PutMemoryTableResponse, Result as FusionResult,
};
use std::convert::TryFrom;
use std::sync::Arc;

#[async_trait]
impl DoPutHandler<PutMemoryTableRequest> for FusionActionHandler {
    async fn handle_do_put(
        &self,
        ticket: PutMemoryTableRequest,
        flight_data: FlightData,
        mut stream: Streaming<FlightData>,
    ) -> FusionResult<PutMemoryTableResponse> {
        let schema = Schema::try_from(&flight_data)
            .map_err(|e| FlightFusionError::ExternalError(format!("Invalid schema: {:?}", e)))?;
        let schema_ref = Arc::new(schema.clone());

        // TODO handle dictionary batches, also ... find out how dictionary batches work.
        // once we can use arrow2, things might become clearer...
        // https://github.com/jorgecarleitao/arrow2/blob/main/integration-testing/src/flight_server_scenarios/integration_test.rs
        let dictionaries_by_field = vec![None; schema_ref.fields().len()];
        let mut batches = vec![];
        while let Some(flight_data) = stream
            .message()
            .await
            .map_err(|e| FlightFusionError::ExternalError(format!("Invalid schema: {:?}", e)))?
        {
            let batch = arrow_flight::utils::flight_data_to_arrow_batch(
                &flight_data,
                schema_ref.clone(),
                &dictionaries_by_field,
            )
            .unwrap();
            batches.push(batch);
        }

        // register received schema
        let table_provider = MemTable::try_new(schema_ref.clone(), vec![batches]).unwrap();
        // TODO get schema name from path
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

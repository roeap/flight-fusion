use super::*;
use arrow_flight::{FlightData, SchemaAsIpc};
use async_trait::async_trait;
use datafusion::prelude::{ExecutionConfig, ExecutionContext};
use flight_fusion_ipc::{FlightFusionError, Result as FusionResult, SqlTicket};
use tonic::Status;

#[async_trait]
impl DoGetHandler<SqlTicket> for FusionActionHandler {
    async fn handle_do_get(
        &self,
        ticket: SqlTicket,
    ) -> FusionResult<BoxedFlightStream<FlightData>> {
        let config = ExecutionConfig::new().with_information_schema(true);
        let mut ctx = ExecutionContext::with_config(config);
        ctx.register_catalog("catalog", self.catalog.clone());

        // execute the query
        let df = ctx
            .sql(ticket.query.as_str())
            .await
            .map_err(to_flight_fusion_err)?;
        let results = df.collect().await.map_err(to_flight_fusion_err)?;
        if results.is_empty() {
            return Err(FlightFusionError::NoReturnData(
                "There were no results from ticket".to_string(),
            ));
        }

        // add an initial FlightData message that sends schema
        let options = datafusion::arrow::ipc::writer::IpcWriteOptions::default();
        let schema_flight_data = SchemaAsIpc::new(&df.schema().clone().into(), &options).into();

        let mut flights: Vec<Result<FlightData, Status>> = vec![Ok(schema_flight_data)];
        let mut batches: Vec<Result<FlightData, Status>> = results
            .iter()
            .flat_map(|batch| {
                let (flight_dictionaries, flight_batch) =
                    arrow_flight::utils::flight_data_from_arrow_batch(batch, &options);
                flight_dictionaries
                    .into_iter()
                    .chain(std::iter::once(flight_batch))
                    .map(Ok)
            })
            .collect();

        // append batch vector to schema vector, so that the first message sent is the schema
        flights.append(&mut batches);

        let output = futures::stream::iter(flights);

        Ok(Box::pin(output) as BoxedFlightStream<FlightData>)
    }
}

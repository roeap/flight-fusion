use super::*;
use crate::error::{FusionServiceError, Result};
use arrow_deps::arrow::{ipc::writer::IpcWriteOptions, record_batch::RecordBatch};
use arrow_flight::{FlightData, SchemaAsIpc};
use tonic::Status;

pub(crate) async fn create_response_stream(
    results: Vec<RecordBatch>,
) -> Result<BoxedFlightStream<FlightData>> {
    if results.is_empty() {
        return Err(FusionServiceError::NoReturnData(
            "There were no results from ticket".to_string(),
        ));
    }
    let schema = results[0].schema();
    // TODO get rid of all the panics
    let options = IpcWriteOptions::default();
    let schema_flight_data = SchemaAsIpc::new(&schema, &options).into();

    let mut flights: Vec<std::result::Result<FlightData, Status>> = vec![Ok(schema_flight_data)];
    let mut batches: Vec<std::result::Result<FlightData, Status>> = results
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

pub(crate) fn meta_to_flight_info(
    meta: Result<AreaSourceMetadata>,
) -> std::result::Result<FlightInfo, tonic::Status> {
    match meta {
        Ok(_m) => {
            // TODO populate with meaningful data
            let descriptor = FlightDescriptor {
                r#type: DescriptorType::Cmd.into(),
                cmd: vec![],
                ..FlightDescriptor::default()
            };
            let endpoint = FlightEndpoint {
                ticket: None,
                location: vec![],
            };
            let info = FlightInfo {
                schema: vec![],
                flight_descriptor: Some(descriptor),
                endpoint: vec![endpoint],
                total_records: -1,
                total_bytes: -1,
            };
            Ok(info)
        }
        Err(e) => Err(tonic::Status::internal(e.to_string())),
    }
}

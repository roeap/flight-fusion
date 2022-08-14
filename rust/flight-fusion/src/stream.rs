use crate::error::{FusionServiceError, Result};
use arrow_deps::arrow_flight::{
    flight_descriptor::DescriptorType,
    utils::{flight_data_from_arrow_batch, flight_data_to_arrow_batch},
    FlightData, SchemaAsIpc,
};
use arrow_deps::datafusion::{
    arrow::datatypes::SchemaRef, arrow::ipc::writer::IpcWriteOptions,
    physical_plan::SendableRecordBatchStream,
};
use arrow_deps::datafusion::{
    arrow::{
        array::ArrayRef,
        datatypes::Schema as ArrowSchema,
        error::{ArrowError, Result as ArrowResult},
        record_batch::RecordBatch,
    },
    physical_plan::RecordBatchStream,
};
use flight_fusion_ipc::FlightDoPutRequest;
use futures::{stream::Stream, StreamExt};
use observability_deps::tracing::info;
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use std::task::Poll;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::{Status, Streaming};

pub type FlightDataSender = Sender<std::result::Result<FlightData, Status>>;
pub type FlightDataReceiver = Receiver<std::result::Result<FlightData, Status>>;

pub async fn raw_stream_to_flight_data_stream(
    mut stream: Streaming<FlightData>,
) -> Result<(SendableRecordBatchStream, FlightDoPutRequest)> {
    let flight_data = stream
        .message()
        .await?
        .ok_or_else(|| FusionServiceError::input("Must send some FlightData"))?;

    let descriptor = flight_data
        .flight_descriptor
        .clone()
        .ok_or_else(|| FusionServiceError::input("Must have a flight descriptor"))?;

    let ticket = match DescriptorType::from_i32(descriptor.r#type) {
        Some(DescriptorType::Cmd) => {
            let request_data = FlightDoPutRequest::decode(&mut descriptor.cmd.as_ref())
                .map_err(|e| FusionServiceError::input(e.to_string()))?;
            Ok(request_data)
        }
        Some(DescriptorType::Path) => Err(FusionServiceError::input(
            "Put operation not implemented for path",
        )),
        _ => Err(FusionServiceError::input(
            "Proper descriptor must be provided",
        )),
    }?;

    let schema = Arc::new(
        ArrowSchema::try_from(&flight_data)
            .map_err(|e| FusionServiceError::input(format!("Invalid schema: {:?}", e)))?,
    );

    Ok((Box::pin(FlightDataStream::new(stream, schema)), ticket))
}

pub struct FlightDataStream {
    stream: Streaming<FlightData>,
    schema: SchemaRef,
    dictionaries_by_id: HashMap<i64, ArrayRef>,
}

impl FlightDataStream {
    pub fn new(stream: Streaming<FlightData>, schema: SchemaRef) -> Self {
        Self {
            stream,
            schema,
            dictionaries_by_id: HashMap::new(),
        }
    }
}

impl Stream for FlightDataStream {
    type Item = ArrowResult<RecordBatch>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx).map(|x| match x {
            Some(flight_data_chunk_result) => {
                let converted_chunk = flight_data_chunk_result
                    .map_err(|e| ArrowError::from_external_error(Box::new(e)))
                    .and_then(|flight_data_chunk| {
                        flight_data_to_arrow_batch(
                            &flight_data_chunk,
                            self.schema.clone(),
                            &self.dictionaries_by_id,
                        )
                    });
                Some(converted_chunk)
            }
            None => None,
        })
    }
}

impl RecordBatchStream for FlightDataStream {
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
}

/// Convert a single RecordBatch into an iterator of FlightData (containing
/// dictionaries and batches)
fn create_flight_iter(
    batch: &RecordBatch,
    options: &IpcWriteOptions,
) -> Box<dyn Iterator<Item = std::result::Result<FlightData, Status>>> {
    let (flight_dictionaries, flight_batch) = flight_data_from_arrow_batch(batch, options);
    Box::new(
        flight_dictionaries
            .into_iter()
            .chain(std::iter::once(flight_batch))
            .map(Ok),
    )
}

pub async fn stream_flight_data(
    mut batch_stream: SendableRecordBatchStream,
    tx: FlightDataSender,
) -> std::result::Result<(), Status> {
    let options = IpcWriteOptions::default();
    let schema_flight_data = SchemaAsIpc::new(batch_stream.schema().as_ref(), &options).into();
    send_response(&tx, Ok(schema_flight_data)).await?;

    let mut row_count = 0;

    while let Some(batch) = batch_stream.next().await {
        if let Ok(x) = &batch {
            row_count += x.num_rows();
        }
        let batch_flight_data: Vec<_> = batch
            .map(|b| create_flight_iter(&b, &options).collect())
            .map_err(|e| from_arrow_err(&e))?;
        for batch in batch_flight_data.into_iter() {
            send_response(&tx, batch).await?;
        }
    }

    info!("FetchPartition streamed {} rows", row_count);
    Ok(())
}

async fn send_response(
    tx: &FlightDataSender,
    data: std::result::Result<FlightData, Status>,
) -> std::result::Result<(), Status> {
    tx.send(data)
        .await
        .map_err(|e| Status::internal(format!("{:?}", e)))
}

fn from_arrow_err(e: &ArrowError) -> Status {
    Status::internal(format!("ArrowError: {:?}", e))
}

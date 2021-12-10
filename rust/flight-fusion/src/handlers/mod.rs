use arrow_azure_core::storage::AzureBlobFileSystem;
use arrow_flight::{FlightData, PutResult};
use async_trait::async_trait;
use datafusion::{
    catalog::{catalog::MemoryCatalogProvider, schema::MemorySchemaProvider},
    parquet::{
        arrow::ParquetFileArrowReader,
        file::serialized_reader::{SerializedFileReader, SliceableCursor},
    },
};
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction,
    flight_do_get_request::Operation as DoGetOperation,
    flight_do_put_request::Operation as DoPutOperation, serialize_message, FlightActionRequest,
    FlightDoGetRequest, FlightDoPutRequest, FlightFusionError, RequestFor, Result as FusionResult,
};
use futures::Stream;
use std::env;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Status, Streaming};

pub mod actions;
pub mod do_get;
pub mod do_put;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

fn to_flight_fusion_err(e: datafusion::error::DataFusionError) -> FlightFusionError {
    FlightFusionError::ExternalError(format!("{:?}", e))
}

#[async_trait]
pub trait ActionHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_action(&self, req: T) -> FusionResult<T::Reply>;
}

#[async_trait]
pub trait DoGetHandler<T>: Sync + Send
where
    T: prost::Message,
{
    async fn handle_do_get(&self, req: T) -> FusionResult<BoxedFlightStream<FlightData>>;
}

#[async_trait]
pub trait DoPutHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_put(
        &self,
        req: T,
        flight_data: FlightData,
        mut stream: Streaming<FlightData>,
    ) -> FusionResult<T::Reply>;
}

pub struct FusionActionHandler {
    fs: Arc<AzureBlobFileSystem>,
    catalog: Arc<MemoryCatalogProvider>,
}

impl FusionActionHandler {
    pub fn new() -> Self {
        let conn_str = env::var("AZURE_STORAGE_CONNECTION_STRING").unwrap();
        let fs = Arc::new(AzureBlobFileSystem::new_with_connection_string(conn_str));
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema".to_string(), Arc::new(schema_provider));
        Self { fs, catalog }
    }

    pub async fn execute_action(
        &self,
        request_data: FlightActionRequest,
    ) -> FusionResult<BoxedFlightStream<arrow_flight::Result>> {
        let body = match request_data.action {
            Some(action) => {
                let result_body = match action {
                    FusionAction::Register(register) => {
                        serialize_message(self.handle_do_action(register).await?)
                    }
                    FusionAction::Drop(drop) => {
                        serialize_message(self.handle_do_action(drop).await?)
                    }
                };

                Ok(result_body)
            }
            None => Err(FlightFusionError::UnknownAction(
                "No action data passed".to_string(),
            )),
        }?;

        let result = vec![Ok(arrow_flight::Result { body })];
        Ok(Box::pin(futures::stream::iter(result)) as BoxedFlightStream<arrow_flight::Result>)
    }

    pub async fn execute_do_get(
        &self,
        request_data: FlightDoGetRequest,
    ) -> FusionResult<BoxedFlightStream<FlightData>> {
        match request_data.operation {
            Some(op) => match op {
                DoGetOperation::Sql(sql) => self.handle_do_get(sql).await,
                DoGetOperation::Kql(_kql) => {
                    todo!()
                }
            },
            None => Err(FlightFusionError::UnknownAction(
                "No operation data passed".to_string(),
            )),
        }
    }

    pub async fn execute_do_put(
        &self,
        request_data: FlightDoPutRequest,
        flight_data: FlightData,
        stream: Streaming<FlightData>,
    ) -> FusionResult<BoxedFlightStream<PutResult>> {
        let body = match request_data.operation {
            Some(action) => {
                let result_body = match action {
                    DoPutOperation::Memory(memory) => {
                        serialize_message(self.handle_do_put(memory, flight_data, stream).await?)
                    }
                    DoPutOperation::Remote(_remote) => {
                        todo!()
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

    async fn get_arrow_reader_from_path<T>(&self, path: T) -> ParquetFileArrowReader
    where
        T: Into<String> + Clone,
    {
        let file_contents = self.fs.get_file_bytes(path).await.unwrap();
        let cursor = SliceableCursor::new(file_contents);
        let file_reader = SerializedFileReader::new(cursor).unwrap();
        ParquetFileArrowReader::new(Arc::new(file_reader))
    }
}

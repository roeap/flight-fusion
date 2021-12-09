use arrow_azure_core::storage::AzureBlobFileSystem;
use arrow_flight::{FlightData, PutResult, SchemaAsIpc};
use async_trait::async_trait;
use datafusion::{
    arrow::datatypes::Schema,
    catalog::{
        catalog::{CatalogProvider, MemoryCatalogProvider},
        schema::MemorySchemaProvider,
    },
    datasource::MemTable,
    parquet::{
        arrow::ParquetFileArrowReader,
        file::serialized_reader::{SerializedFileReader, SliceableCursor},
    },
    prelude::{ExecutionConfig, ExecutionContext},
};
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction,
    flight_do_get_request::Operation as DoGetOperation,
    flight_do_put_request::Operation as DoPutOperation, serialize_message, FlightActionRequest,
    FlightDoGetRequest, FlightDoPutRequest, FlightFusionError, PutMemoryTableRequest,
    PutMemoryTableResponse, RequestFor, Result as FusionResult, SqlTicket,
};
use futures::Stream;
use std::convert::TryFrom;
use std::env;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Status, Streaming};

pub mod actions;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

fn to_tonic_err(e: datafusion::error::DataFusionError) -> FlightFusionError {
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

#[async_trait::async_trait]
impl DoGetHandler<SqlTicket> for FusionActionHandler {
    async fn handle_do_get(
        &self,
        ticket: SqlTicket,
    ) -> FusionResult<BoxedFlightStream<FlightData>> {
        let config = ExecutionConfig::new().with_information_schema(true);
        let mut ctx = ExecutionContext::with_config(config);
        ctx.register_catalog("catalog", self.catalog.clone());

        // execute the query
        let df = ctx.sql(ticket.query.as_str()).await.map_err(to_tonic_err)?;
        let results = df.collect().await.map_err(to_tonic_err)?;
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

#[async_trait::async_trait]
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

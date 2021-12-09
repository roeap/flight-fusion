use arrow_azure_core::storage::AzureBlobFileSystem;
use arrow_flight::{FlightData, PutResult, SchemaAsIpc, Ticket};
use datafusion::{
    catalog::{catalog::MemoryCatalogProvider, schema::MemorySchemaProvider},
    parquet::{
        arrow::ParquetFileArrowReader,
        file::serialized_reader::{SerializedFileReader, SliceableCursor},
    },
    prelude::{ExecutionConfig, ExecutionContext},
};
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction,
    flight_do_get_request::Operation as DoGetOperation, serialize_message, FlightActionRequest,
    FlightDoGetRequest, FlightFusionError, RequestFor, Result as FusionResult, SqlTicket,
};
use futures::Stream;
use std::collections::HashMap;
use std::env;
use std::fmt::{self, Debug};
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use tonic::{Status, Streaming};

use async_trait::async_trait;

pub mod actions;
pub mod fusion;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

fn to_tonic_err(e: datafusion::error::DataFusionError) -> FlightFusionError {
    FlightFusionError::ExternalError(format!("{:?}", e))
}

#[async_trait]
pub trait DoPutHandler: Sync + Send + Debug {
    async fn do_put(
        &self,
        flight_data: FlightData,
        mut stream: Streaming<FlightData>,
    ) -> Result<BoxedFlightStream<PutResult>, Status>;

    fn can_handle_descriptor(&self, ticket: Ticket) -> Result<bool, Status>;
}

/// A Registry holds all the flight handlers at runtime.
pub struct FlightHandlerRegistry {
    pub do_put_handlers: RwLock<HashMap<String, Arc<dyn DoPutHandler>>>,
}

impl fmt::Debug for FlightHandlerRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlightHandlerRegistry").finish()
    }
}

impl FlightHandlerRegistry {
    /// Create the registry that flight handlers can registered into.
    pub fn new() -> Self {
        let do_put_map: HashMap<String, Arc<dyn DoPutHandler>> = HashMap::new();

        Self {
            do_put_handlers: RwLock::new(do_put_map),
        }
    }

    pub fn register_do_put_handler(
        &self,
        scheme: String,
        handler: Arc<dyn DoPutHandler>,
    ) -> Option<Arc<dyn DoPutHandler>> {
        let mut stores = self.do_put_handlers.write().unwrap();
        stores.insert(scheme, handler)
    }

    pub fn get_do_put(&self, scheme: &str) -> Option<Arc<dyn DoPutHandler>> {
        let stores = self.do_put_handlers.read().unwrap();
        stores.get(scheme).cloned()
    }
}

// ##################################

#[async_trait]
pub trait ActionHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_action(&self, req: T) -> FusionResult<T::Reply>;
}

#[async_trait]
pub trait DoGetHandlerNew<T>: Sync + Send
where
    T: prost::Message,
{
    async fn handle_do_get(&self, req: T) -> FusionResult<BoxedFlightStream<FlightData>>;
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
impl DoGetHandlerNew<SqlTicket> for FusionActionHandler {
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

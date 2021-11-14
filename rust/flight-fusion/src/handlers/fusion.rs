use super::{ActionHandler, BoxedFlightStream, DoGetHandler, DoPutHandler};
use arrow_azure_core::storage::AzureBlobFileSystem;
use arrow_flight::{
    flight_descriptor::DescriptorType, Action, ActionType, FlightData, PutResult, SchemaAsIpc,
    Ticket,
};
use async_trait::async_trait;
use datafusion::arrow::datatypes::Schema;
use datafusion::catalog::{
    catalog::{CatalogProvider, MemoryCatalogProvider},
    schema::MemorySchemaProvider,
};
use datafusion::datasource::MemTable;
use datafusion::parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use datafusion::prelude::{ExecutionConfig, ExecutionContext};
use std::convert::TryFrom;
use std::env;
use std::fmt;
use std::sync::Arc;
use tonic::{Status, Streaming};
use tracing::debug;

fn to_tonic_err(e: datafusion::error::DataFusionError) -> Status {
    Status::internal(format!("{:?}", e))
}

pub struct FlightFusionHandler {
    fs: Arc<AzureBlobFileSystem>,
    catalog: Arc<MemoryCatalogProvider>,
}

impl FlightFusionHandler {
    pub fn new() -> Self {
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema".to_string(), Arc::new(schema_provider));
        let conn_str = env::var("AZURE_STORAGE_CONNECTION_STRING").unwrap();
        let fs = Arc::new(AzureBlobFileSystem::new_with_connection_string(conn_str));

        Self { fs, catalog }
    }

    async fn get_arrow_reader_from_path(&self, path: String) -> ParquetFileArrowReader {
        let file_contents = self.fs.get_file_bytes(path).await.unwrap();
        debug!("read bytes: {}", file_contents.len());
        let cursor = SliceableCursor::new(file_contents);
        let file_reader = SerializedFileReader::new(cursor).unwrap();
        ParquetFileArrowReader::new(Arc::new(file_reader))
    }
}

impl fmt::Debug for FlightFusionHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlightFusionHandler").finish()
    }
}

#[async_trait]
impl DoGetHandler for FlightFusionHandler {
    fn can_handle_ticket(&self, _ticket: Ticket) -> Result<bool, Status> {
        Ok(true)
    }

    async fn do_get(&self, ticket: Ticket) -> Result<BoxedFlightStream<FlightData>, Status> {
        match std::str::from_utf8(&ticket.ticket) {
            Ok(sql) => {
                println!("do_get: {}", sql);

                // create local execution context
                let config = ExecutionConfig::new().with_information_schema(true);
                let mut ctx = ExecutionContext::with_config(config);
                ctx.register_catalog("catalog", self.catalog.clone());

                // execute the query
                let df = ctx.sql(sql).await.map_err(to_tonic_err)?;
                let results = df.collect().await.map_err(to_tonic_err)?;
                if results.is_empty() {
                    return Err(Status::internal("There were no results from ticket"));
                }

                // add an initial FlightData message that sends schema
                let options = datafusion::arrow::ipc::writer::IpcWriteOptions::default();
                let schema_flight_data =
                    SchemaAsIpc::new(&df.schema().clone().into(), &options).into();

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
            Err(e) => Err(Status::invalid_argument(format!("Invalid ticket: {:?}", e))),
        }
    }
}

#[async_trait]
impl DoPutHandler for FlightFusionHandler {
    fn can_handle_descriptor(&self, _ticket: Ticket) -> Result<bool, Status> {
        Ok(true)
    }

    async fn do_put(
        &self,
        flight_data: FlightData,
        mut stream: Streaming<FlightData>,
    ) -> Result<BoxedFlightStream<PutResult>, Status> {
        let descriptor = flight_data
            .flight_descriptor
            .clone()
            .ok_or_else(|| Status::invalid_argument("Must have a descriptor"))?;

        if descriptor.r#type != DescriptorType::Path as i32 || descriptor.path.is_empty() {
            return Err(Status::invalid_argument("Must specify a path"));
        }
        let key = descriptor.path[0].clone();

        // the schema is the first flight data message
        let schema = Schema::try_from(&flight_data)
            .map_err(|e| Status::invalid_argument(format!("Invalid schema: {:?}", e)))?;
        let schema_ref = Arc::new(schema.clone());

        // TODO handle dictionary batches, also ... find out how dictionary batches work.
        // once we can use arrow2, things might become clearer...
        // https://github.com/jorgecarleitao/arrow2/blob/main/integration-testing/src/flight_server_scenarios/integration_test.rs
        let dictionaries_by_field = vec![None; schema_ref.fields().len()];
        let mut batches = vec![];
        while let Some(flight_data) = stream.message().await? {
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
            .register_table(key, Arc::new(table_provider))
            .unwrap();

        self.catalog
            .register_schema("schema".to_string(), schema_provider);

        // TODO generate messages in channel
        // https://github.com/jorgecarleitao/arrow2/blob/main/integration-testing/src/flight_server_scenarios/integration_test.rs
        let result = Ok(PutResult {
            app_metadata: "created".as_bytes().to_vec(),
        });
        let output = futures::stream::iter(vec![result]);
        Ok(Box::pin(output) as BoxedFlightStream<PutResult>)
    }
}

#[async_trait]
impl ActionHandler for FlightFusionHandler {
    async fn list_actions(&self) -> Result<BoxedFlightStream<ActionType>, Status> {
        let actions = vec![
            Ok(ActionType {
                r#type: "register-table".to_string(),
                description: "Register a new table such that it can be queried.".to_string(),
            }),
            Ok(ActionType {
                r#type: "register-delta-table".to_string(),
                description: "Register a new delta table such that it can be queried.".to_string(),
            }),
        ];
        let output = futures::stream::iter(actions);
        Ok(Box::pin(output) as BoxedFlightStream<ActionType>)
    }

    async fn do_action(
        &self,
        action: Action,
    ) -> Result<BoxedFlightStream<arrow_flight::Result>, Status> {
        let result = match action.r#type.as_str() {
            "register-table" => {
                let body = std::str::from_utf8(&action.body).unwrap();
                let (table_name, path) = body.split_once("::").unwrap();

                let mut reader = self.get_arrow_reader_from_path(path.into()).await;
                let schema = Arc::new(reader.get_schema().unwrap());
                let batch_reader = reader.get_record_reader(1024).unwrap();
                let batches = batch_reader
                    .into_iter()
                    .map(|batch| batch.unwrap())
                    .collect::<Vec<_>>();

                // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
                let table_provider = MemTable::try_new(schema, vec![batches]).unwrap();
                let schema_provider = self.catalog.schema("schema").unwrap();
                schema_provider
                    .register_table(table_name.to_string(), Arc::new(table_provider))
                    .unwrap();

                self.catalog
                    .register_schema("schema".to_string(), schema_provider);

                Ok(arrow_flight::Result {
                    body: "created".as_bytes().to_vec(),
                })
            }
            "register-delta-table" => {
                Err(Status::unimplemented("Delta tables not yet implemented"))
            }
            _ => Err(Status::failed_precondition("Unknown action type")),
        };

        Ok(Box::pin(futures::stream::iter(vec![result]))
            as BoxedFlightStream<arrow_flight::Result>)
    }

    fn can_do_action(&self, _action: Action) -> Result<bool, Status> {
        Ok(true)
    }
}

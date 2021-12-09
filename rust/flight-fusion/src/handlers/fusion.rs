use super::{BoxedFlightStream, DoPutHandler};
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
    arrow::ParquetFileArrowReader,
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

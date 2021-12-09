use super::BoxedFlightStream;
use arrow_azure_core::storage::AzureBlobFileSystem;
use datafusion::catalog::{
    catalog::{CatalogProvider, MemoryCatalogProvider},
    schema::MemorySchemaProvider,
};
use datafusion::datasource::MemTable;
use datafusion::parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction, DropDatasetRequest, DropDatasetResponse,
    FlightActionRequest, FlightFusionError, RegisterDatasetRequest, RegisterDatasetResponse,
    RequestFor, Result as FusionResult,
};
use prost::Message;
use std::env;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait ActionHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle(&self, req: T) -> FusionResult<T::Reply>;
}

fn serialize_message(msg: impl Message) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(msg.encoded_len());
    msg.encode(&mut buf).unwrap();
    buf
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
    pub async fn execute(
        &self,
        request_data: FlightActionRequest,
    ) -> FusionResult<BoxedFlightStream<arrow_flight::Result>> {
        let body = match request_data.action {
            Some(action) => {
                let result_body = match action {
                    FusionAction::Register(register) => {
                        serialize_message(self.handle(register).await?)
                    }
                    FusionAction::Drop(drop) => serialize_message(self.handle(drop).await?),
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
impl ActionHandler<DropDatasetRequest> for FusionActionHandler {
    async fn handle(&self, action: DropDatasetRequest) -> FusionResult<DropDatasetResponse> {
        let response = DropDatasetResponse { name: action.name };
        Ok(response)
    }
}

#[async_trait::async_trait]
impl ActionHandler<RegisterDatasetRequest> for FusionActionHandler {
    async fn handle(
        &self,
        action: RegisterDatasetRequest,
    ) -> FusionResult<RegisterDatasetResponse> {
        let mut reader = self.get_arrow_reader_from_path(action.path).await;
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
            .register_table(action.name.clone(), Arc::new(table_provider))
            .unwrap();

        self.catalog
            .register_schema("schema".to_string(), schema_provider);

        Ok(RegisterDatasetResponse {
            message: action.name,
        })
    }
}

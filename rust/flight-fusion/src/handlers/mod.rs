use crate::{area_store::InMemoryAreaStore, stream::*};
use arrow_deps::datafusion::{
    catalog::{catalog::MemoryCatalogProvider, schema::MemorySchemaProvider},
    datasource::{
        file_format::parquet::ChunkObjectReader,
        object_store::{
            local::{local_unpartitioned_file, LocalFileSystem},
            ObjectStore,
        },
    },
    parquet::{arrow::ParquetFileArrowReader, file::serialized_reader::SerializedFileReader},
    physical_plan::ExecutionPlan,
};
use arrow_flight::{FlightData, PutResult};
use async_trait::async_trait;
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction,
    flight_do_get_request::Operation as DoGetOperation,
    flight_do_put_request::Operation as DoPutOperation, serialize_message, FlightActionRequest,
    FlightDoGetRequest, FlightFusionError, RequestFor, Result as FusionResult,
};
use futures::Stream;
use std::sync::Arc;
use std::{path::PathBuf, pin::Pin};
use tonic::Status;

pub mod actions;
pub mod do_get;
pub mod do_put;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

fn to_flight_fusion_err(e: arrow_deps::datafusion::error::DataFusionError) -> FlightFusionError {
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
    async fn handle_do_put(&self, req: T, input: Arc<dyn ExecutionPlan>) -> FusionResult<T::Reply>;
}

pub struct FusionActionHandler {
    catalog: Arc<MemoryCatalogProvider>,
    object_store: Arc<dyn ObjectStore>,
    area_store: InMemoryAreaStore,
}

impl FusionActionHandler {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let object_store = Arc::new(LocalFileSystem);
        let area_store = InMemoryAreaStore::new(root);
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema".to_string(), Arc::new(schema_provider));
        Self {
            object_store,
            catalog,
            area_store,
        }
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
                DoGetOperation::Kql(_) => {
                    todo!()
                }
                DoGetOperation::Frame(_) => {
                    todo!()
                }
            },
            None => Err(FlightFusionError::UnknownAction(
                "No operation data passed".to_string(),
            )),
        }
    }

    async fn get_arrow_reader_from_path<T>(&self, path: T) -> FusionResult<ParquetFileArrowReader>
    where
        T: Into<String>,
    {
        let file = local_unpartitioned_file(path.into());
        let object_reader = self
            .object_store
            .file_reader(file.file_meta.sized_file)
            .unwrap();
        let obj_reader = ChunkObjectReader(object_reader);
        let file_reader = Arc::new(SerializedFileReader::new(obj_reader).unwrap());
        Ok(ParquetFileArrowReader::new(file_reader))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flight_fusion_ipc::{DatasetFormat, DropDatasetRequest, RegisterDatasetRequest};

    #[tokio::test]
    async fn test_drop_table_action() {
        let handler = crate::test_utils::get_fusion_handler();
        let req = DropDatasetRequest {
            name: "some.table".to_string(),
        };
        let res = handler.handle_do_action(req).await.unwrap();
        assert_eq!(res.name, "some.table")
    }

    #[tokio::test]
    async fn test_put_table() {
        let handler = crate::test_utils::get_fusion_handler();
        let register_table_action = RegisterDatasetRequest {
            path: "./tests/data/file/table.parquet".to_string(),
            name: "table".to_string(),
            format: DatasetFormat::File as i32,
        };
        let res = handler
            .handle_do_action(register_table_action)
            .await
            .unwrap();
        println!("{:?}", res)
    }
}

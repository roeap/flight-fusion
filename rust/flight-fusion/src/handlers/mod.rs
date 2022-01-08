use crate::{area_store::InMemoryAreaStore, stream::*};
use arrow_deps::datafusion::{
    catalog::{catalog::MemoryCatalogProvider, schema::MemorySchemaProvider},
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
pub use object_store::{path::ObjectStorePath, ObjectStoreApi};
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
    area_store: InMemoryAreaStore,
}

impl FusionActionHandler {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let area_store = InMemoryAreaStore::new(root);
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema".to_string(), Arc::new(schema_provider));
        Self {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        DropDatasetRequest, PutTableRequest, SaveMode,
    };

    #[tokio::test]
    async fn test_table_put_drop() {
        let root = crate::test_utils::workspace_test_data_folder();
        let plan = crate::test_utils::get_input_plan(None, false);
        let handler = crate::test_utils::get_fusion_handler(root.clone());
        let table_dir = root.join("data/new_table");

        let table = TableReference::Location(AreaTableLocation {
            name: "new_table".to_string(),
            areas: vec![],
        });
        let put_request = PutTableRequest {
            table: Some(AreaSourceReference { table: Some(table) }),
            save_mode: SaveMode::Overwrite as i32,
        };

        assert!(!table_dir.exists());

        let _put_response = handler.handle_do_put(put_request, plan).await.unwrap();

        assert!(table_dir.is_dir());

        let drop_request = DropDatasetRequest {
            name: "some.table".to_string(),
        };
        let _drop_response = handler.handle_do_action(drop_request).await.unwrap();

        assert!(!table_dir.exists());
    }
}

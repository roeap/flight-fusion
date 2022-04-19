use crate::{
    error::{FusionServiceError, Result},
    stream::*,
};
use area_store::{
    catalog::{AreaCatalog, FileAreaCatalog},
    store::{AreaStore, DefaultAreaStore},
};
use arrow_deps::datafusion::{
    arrow::datatypes::SchemaRef as ArrowSchemaRef,
    catalog::{
        catalog::CatalogProvider, catalog::MemoryCatalogProvider, schema::MemorySchemaProvider,
    },
    datasource::MemTable,
    physical_plan::ExecutionPlan,
    prelude::*,
};
use arrow_flight::{
    flight_descriptor::DescriptorType, FlightData, FlightDescriptor, FlightEndpoint, FlightInfo,
    PutResult,
};
use async_trait::async_trait;
use flight_fusion_ipc::AreaSourceReference;
use flight_fusion_ipc::{
    area_source_reference::Table, flight_action_request::Action as FusionAction,
    flight_do_get_request::Command as DoGetCommand, serialize_message, AreaSourceDetails,
    AreaSourceMetadata, CommandListSources, FlightActionRequest, FlightDoGetRequest,
    FlightGetFlightInfoRequest, FlightGetSchemaRequest, RequestFor,
};
use futures::Stream;
pub use object_store::{path::ObjectStorePath, ObjectStoreApi};
use std::sync::Arc;
use std::{path::PathBuf, pin::Pin};
use tonic::Status;

pub mod actions;
mod delta;
pub mod do_get;
pub mod do_put;
mod utils;

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = std::result::Result<T, Status>> + Send + Sync + 'static>>;

#[async_trait]
pub trait ActionHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_action(&self, req: T) -> Result<T::Reply>;
}

#[async_trait]
pub trait DoGetHandler<T>: Sync + Send
where
    T: prost::Message,
{
    async fn handle_do_get(&self, req: T) -> Result<BoxedFlightStream<FlightData>>;
}

#[async_trait]
pub trait DoPutHandler<T>: Sync + Send
where
    T: RequestFor,
{
    async fn handle_do_put(&self, req: T, input: Arc<dyn ExecutionPlan>) -> Result<T::Reply>;
}

pub struct FusionActionHandler {
    catalog: Arc<MemoryCatalogProvider>,
    area_store: Arc<DefaultAreaStore>,
    area_catalog: Arc<FileAreaCatalog>,
}

impl FusionActionHandler {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema", Arc::new(schema_provider))?;

        let area_store = Arc::new(DefaultAreaStore::new(root));
        let area_catalog = Arc::new(FileAreaCatalog::new(area_store.clone()));

        Ok(Self {
            catalog,
            area_store,
            area_catalog,
        })
    }

    pub fn new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Result<Self> {
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema", Arc::new(schema_provider))?;

        let area_store = Arc::new(DefaultAreaStore::new_azure(
            account,
            access_key,
            container_name,
        )?);
        let area_catalog = Arc::new(FileAreaCatalog::new(area_store.clone()));

        Ok(Self {
            catalog,
            area_store,
            area_catalog,
        })
    }

    pub async fn register_source(
        &self,
        ctx: &mut SessionContext,
        source: &AreaSourceReference,
    ) -> Result<()> {
        let location = self.area_store.get_table_location(&source.clone())?;
        let batches = self.area_store.get_batches(&location).await?;
        let table_provider = Arc::new(MemTable::try_new(batches[0].schema(), vec![batches])?);
        let name = match &source {
            AreaSourceReference {
                table: Some(Table::Location(tbl)),
            } => tbl.name.clone(),
            _ => todo!(),
        };
        ctx.register_table(&*name, table_provider)?;
        Ok(())
    }

    pub async fn list_flights(
        &self,
        _command: CommandListSources,
    ) -> Result<BoxedFlightStream<FlightInfo>> {
        // let _ = Ok(Box::pin(
        //     self.area_catalog
        //         .list_area_sources(command.root)
        //         .await
        //         .map_err(to_fusion_err)?
        //         .map(meta_to_flight_info),
        // ));
        Ok(Box::pin(futures::stream::iter(vec![])) as BoxedFlightStream<FlightInfo>)
        // todo!()
    }

    pub async fn get_schema(&self, request: FlightGetSchemaRequest) -> Result<ArrowSchemaRef> {
        if let Some(source) = request.source {
            let schema = self.area_store.get_schema(&source).await?;
            Ok(schema)
        } else {
            todo!()
        }
    }

    pub async fn get_flight_info(&self, request: FlightGetFlightInfoRequest) -> Result<FlightInfo> {
        if let Some(source) = request.source {
            let details = self.area_catalog.get_source_details(source).await?;

            Ok(details_to_flight_info(details))
        } else {
            todo!()
        }
    }

    pub async fn execute_action(
        &self,
        request_data: FlightActionRequest,
    ) -> Result<BoxedFlightStream<arrow_flight::Result>> {
        let body = match request_data.action {
            Some(action) => {
                let result_body = match action {
                    FusionAction::Register(_register) => {
                        todo!()
                        // serialize_message(self.handle_do_action(register).await?)
                    }
                    FusionAction::Drop(drop) => {
                        serialize_message(self.handle_do_action(drop).await?)
                    }
                    FusionAction::SetMeta(meta) => {
                        serialize_message(self.handle_do_action(meta).await?)
                    }
                };

                Ok(result_body)
            }
            None => Err(FusionServiceError::unknown_action("No action data passed")),
        }?;

        let result = vec![Ok(arrow_flight::Result { body })];
        Ok(Box::pin(futures::stream::iter(result)) as BoxedFlightStream<arrow_flight::Result>)
    }

    pub async fn execute_do_get(
        &self,
        request_data: FlightDoGetRequest,
    ) -> Result<BoxedFlightStream<FlightData>> {
        match request_data.command {
            Some(op) => match op {
                DoGetCommand::Sql(sql) => self.handle_do_get(sql).await,
                DoGetCommand::Kql(_) => {
                    todo!()
                }
                DoGetCommand::Frame(_) => {
                    todo!()
                }
                DoGetCommand::Read(read) => self.handle_do_get(read).await,
                DoGetCommand::Query(query) => self.handle_do_get(query).await,
                DoGetCommand::Delta(operation) => self.handle_do_get(operation).await,
            },
            None => Err(FusionServiceError::unknown_action(
                "No operation data passed",
            )),
        }
    }
}

fn details_to_flight_info(details: AreaSourceDetails) -> FlightInfo {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        CommandDropSource, CommandWriteIntoDataset, SaveMode,
    };

    #[ignore = "currently directories are not deleted when tables are dropped"]
    #[tokio::test]
    async fn test_table_put_drop() {
        let root = crate::test_utils::workspace_test_data_folder();
        let plan = crate::test_utils::get_input_plan(None, false);
        let handler = crate::test_utils::get_fusion_handler(root.clone());
        let table_dir = root.join("_ff_data/new_table");

        let table_ref = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "new_table".to_string(),
                areas: vec![],
            })),
        };
        let put_request = CommandWriteIntoDataset {
            source: Some(table_ref.clone()),
            save_mode: SaveMode::Overwrite.into(),
        };

        assert!(!table_dir.exists());

        let _put_response = handler.handle_do_put(put_request, plan).await.unwrap();

        assert!(table_dir.is_dir());

        let drop_request = CommandDropSource {
            source: Some(table_ref),
        };
        let _drop_response = handler.handle_do_action(drop_request).await.unwrap();

        assert!(!table_dir.exists());
    }
}

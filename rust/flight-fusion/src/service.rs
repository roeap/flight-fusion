use crate::handlers::*;
use crate::stream::FlightReceiverPlan;
use area_store::store::{AreaStore, DefaultAreaStore};
use arrow_deps::datafusion::{
    arrow::ipc::writer::IpcWriteOptions,
    catalog::{
        catalog::{CatalogProvider, MemoryCatalogProvider},
        schema::MemorySchemaProvider,
    },
    datasource::MemTable,
    prelude::SessionContext,
};
use arrow_flight::{
    flight_descriptor::DescriptorType, flight_service_server::FlightService, Action, ActionType,
    Criteria, Empty, FlightData, FlightDescriptor, FlightInfo, HandshakeRequest, HandshakeResponse,
    IpcMessage, PutResult, SchemaAsIpc, SchemaResult, Ticket,
};
use flight_fusion_ipc::{
    area_source_reference::Table, flight_action_request::Action as FusionAction,
    flight_do_get_request::Command as DoGetCommand, flight_do_put_request::Command as DoPutCommand,
    serialize_message, AreaSourceReference, CommandListSources, FlightActionRequest,
    FlightDoGetRequest, FlightGetFlightInfoRequest,
};
use futures::Stream;
use observability_deps::instrument;
use observability_deps::opentelemetry::{global, propagation::Extractor};
use observability_deps::tracing;
use observability_deps::tracing_opentelemetry::OpenTelemetrySpanExt;
use prost::Message;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap.  If the value can't be converted to &str, returns None
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|metadata| metadata.to_str().ok())
    }

    /// Collect all the keys from the MetadataMap.
    fn keys(&self) -> Vec<&str> {
        self.0
            .keys()
            .map(|key| match key {
                tonic::metadata::KeyRef::Ascii(v) => v.as_str(),
                tonic::metadata::KeyRef::Binary(v) => v.as_str(),
            })
            .collect::<Vec<_>>()
    }
}

#[derive(Clone)]
pub struct FlightFusionService {
    pub(crate) catalog: Arc<MemoryCatalogProvider>,
    pub(crate) area_store: Arc<DefaultAreaStore>,
}

impl FlightFusionService {
    pub fn new_default(root: impl Into<PathBuf>) -> crate::error::Result<Self> {
        let root: PathBuf = root.into();

        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema", Arc::new(schema_provider))?;

        let area_store = Arc::new(DefaultAreaStore::try_new(root)?);

        Ok(Self {
            catalog,
            area_store,
        })
    }

    pub fn new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> crate::error::Result<Self> {
        let schema_provider = MemorySchemaProvider::new();
        let catalog = Arc::new(MemoryCatalogProvider::new());
        catalog.register_schema("schema", Arc::new(schema_provider))?;

        let area_store = Arc::new(DefaultAreaStore::try_new_azure(
            account,
            access_key,
            container_name,
        )?);

        Ok(Self {
            catalog,
            area_store,
        })
    }

    pub async fn register_source(
        &self,
        ctx: &mut SessionContext,
        source: &AreaSourceReference,
    ) -> crate::error::Result<()> {
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

    pub async fn build_index(&self) -> crate::error::Result<()> {
        Ok(self.area_store.build_index().await?)
    }
}

impl std::fmt::Debug for FlightFusionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlightFusionService").finish()
    }
}

#[tonic::async_trait]
impl FlightService for FlightFusionService {
    type HandshakeStream = BoxedFlightStream<HandshakeResponse>;
    type ListFlightsStream = BoxedFlightStream<FlightInfo>;
    type DoGetStream = BoxedFlightStream<FlightData>;
    type DoPutStream = BoxedFlightStream<PutResult>;
    type DoActionStream = BoxedFlightStream<arrow_flight::Result>;
    type ListActionsStream = BoxedFlightStream<ActionType>;
    type DoExchangeStream = BoxedFlightStream<FlightData>;

    #[instrument(skip(self, _request))]
    async fn handshake(
        &self,
        _request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    #[instrument(skip(self, request))]
    async fn list_flights(
        &self,
        request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let criteria = request.into_inner();
        let _command = CommandListSources::decode(&mut criteria.expression.as_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(
            Box::pin(futures::stream::iter(vec![])) as BoxedFlightStream<FlightInfo>
        ))
    }

    #[instrument(skip(self, request))]
    async fn get_flight_info(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let command = message_from_descriptor::<FlightGetFlightInfoRequest>(request)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let schema = if let Some(source) = command.source {
            Ok(self
                .area_store
                .get_schema(&source)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?)
        } else {
            Err(crate::error::FusionServiceError::InputError(
                "Expected valid command payload".to_string(),
            ))
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let options = IpcWriteOptions::default();
        let schema_result = SchemaAsIpc::new(&schema, &options);

        let descriptor = FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: vec![],
            ..FlightDescriptor::default()
        };
        let info = FlightInfo::new(
            IpcMessage::try_from(schema_result)
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
            Some(descriptor),
            vec![],
            -1,
            -1,
        );

        Ok(Response::new(info))
    }

    #[instrument(skip(self, request))]
    async fn get_schema(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let command = message_from_descriptor::<FlightGetFlightInfoRequest>(request)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let schema = if let Some(source) = command.source {
            Ok(self
                .area_store
                .get_schema(&source)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?)
        } else {
            Err(crate::error::FusionServiceError::InputError(
                "Expected valid command payload".to_string(),
            ))
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let options = IpcWriteOptions::default();
        let schema_result = SchemaAsIpc::new(&schema, &options);
        let IpcMessage(vals) = IpcMessage::try_from(schema_result).unwrap();

        let result = SchemaResult { schema: vals };

        Ok(Response::new(result))
    }

    #[instrument(skip(self, request))]
    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let request: FlightDoGetRequest =
            FlightDoGetRequest::decode(&mut request.into_inner().ticket.as_ref())
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let result = match request.command {
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
            None => Err(crate::error::FusionServiceError::unknown_action(
                "No operation data passed",
            )),
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(result))
    }

    #[instrument(skip(self, request))]
    async fn do_put(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let rec_plan = Arc::new(
            FlightReceiverPlan::try_new(request.into_inner())
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        );

        let request_data = rec_plan.ticket();
        let body = match &request_data.command {
            Some(action) => {
                let result_body = match action {
                    DoPutCommand::Storage(storage) => serialize_message(
                        self.handle_do_put(storage.clone(), rec_plan)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                    DoPutCommand::Delta(delta) => serialize_message(
                        self.handle_do_put(delta.clone(), rec_plan)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                };

                Ok(result_body)
            }
            None => Err(crate::error::FusionServiceError::unknown_action(
                "No action data passed",
            )),
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let result = vec![Ok(PutResult { app_metadata: body })];

        Ok(Response::new(
            Box::pin(futures::stream::iter(result)) as BoxedFlightStream<PutResult>
        ))
    }

    #[instrument(skip(self, request))]
    async fn do_action(
        &self,
        request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let flight_action = request.into_inner();
        let request_data: FlightActionRequest =
            FlightActionRequest::decode(&mut flight_action.body.as_ref())
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let body = match request_data.action {
            Some(action) => {
                let result_body = match action {
                    FusionAction::Drop(drop) => serialize_message(
                        self.handle_do_action(drop)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                    FusionAction::SetMeta(meta) => serialize_message(
                        self.handle_do_action(meta)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                };

                Ok(result_body)
            }
            None => Err(crate::error::FusionServiceError::unknown_action(
                "No action data passed",
            )),
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let result = vec![Ok(arrow_flight::Result { body })];

        Ok(Response::new(
            Box::pin(futures::stream::iter(result)) as BoxedFlightStream<arrow_flight::Result>
        ))
    }

    #[instrument(skip(self, _request))]
    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }

    #[instrument(skip(self, _request))]
    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Not implemented"))
    }
}

fn message_from_descriptor<M>(
    data: Request<FlightDescriptor>,
) -> Result<M, crate::error::FusionServiceError>
where
    M: prost::Message + Default,
{
    let descriptor = data.into_inner();
    match DescriptorType::from_i32(descriptor.r#type) {
        Some(DescriptorType::Cmd) => {
            let request_data = M::decode(&mut descriptor.cmd.as_ref())
                .map_err(|e| tonic::Status::internal(e.to_string()))?;
            Ok(request_data)
        }
        _ => Err(crate::error::FusionServiceError::InputError(
            "`get_schema` requires command to be defined on flight descriptor".to_string(),
        )),
    }
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

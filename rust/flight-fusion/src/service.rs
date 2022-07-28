use crate::stream::{
    raw_stream_to_flight_data_stream, stream_flight_data, FlightDataReceiver, FlightDataSender,
};
use crate::{error::FusionServiceError, handlers::*};
use area_store::store::{is_delta_location, AreaPath, AreaStore, DefaultAreaStore};
use arrow_deps::arrow_flight::{
    self, flight_descriptor::DescriptorType, flight_service_server::FlightService, Action,
    ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo, HandshakeRequest,
    HandshakeResponse, IpcMessage, PutResult, SchemaAsIpc, SchemaResult, Ticket,
};
use arrow_deps::datafusion::{
    arrow::{datatypes::Schema, ipc::writer::IpcWriteOptions},
    catalog::{
        catalog::{CatalogProvider, MemoryCatalogProvider},
        schema::MemorySchemaProvider,
    },
    datasource::MemTable,
    physical_plan::common::collect,
    prelude::SessionContext,
};
use flight_fusion_ipc::{
    area_source_reference::Table, flight_action_request::Action as FusionAction,
    flight_do_get_request::Command as DoGetCommand, flight_do_put_request::Command as DoPutCommand,
    serialize_message, AreaSourceMetadata, AreaSourceReference, CommandListSources,
    FlightActionRequest, FlightDoGetRequest,
};
use futures::Stream;
use observability_deps::opentelemetry::{global, propagation::Extractor};
use observability_deps::tracing_opentelemetry::OpenTelemetrySpanExt;
use observability_deps::{instrument, tracing};
use prost::Message;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::channel;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub type BoxedFlightStream<T> =
    Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

struct MetadataMap<'a>(&'a tonic::metadata::MetadataMap);

impl<'a> Extractor for MetadataMap<'a> {
    /// Get a value for a key from the MetadataMap. If the value can't be converted to &str, returns None
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
    #[allow(unused)]
    pub(crate) catalog: Arc<MemoryCatalogProvider>,
    /// the area store provides high level access to registered datasets.
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
        let location: AreaPath = source.clone().into();
        let files = self.area_store.get_location_files(&location).await.unwrap();
        let mut batches = Vec::new();
        for file in files {
            let curr = collect(self.area_store.open_file(&file.into(), None).await?).await?;
            batches.extend(curr)
        }
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

        let files = self
            .area_store
            .list_areas(None)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let infos = files.into_iter().map(|a| {
            let source: AreaSourceReference = a.into();
            let descriptor = FlightDescriptor {
                r#type: DescriptorType::Cmd.into(),
                cmd: source.encode_to_vec(),
                ..FlightDescriptor::default()
            };

            let options = IpcWriteOptions::default();
            let schema = Schema::new(vec![]);
            let schema_result = SchemaAsIpc::new(&schema, &options);

            Ok(FlightInfo::new(
                IpcMessage::try_from(schema_result)
                    .map_err(|e| tonic::Status::internal(e.to_string()))?,
                Some(descriptor),
                vec![],
                -1,
                -1,
            ))
        });

        Ok(Response::new(
            Box::pin(futures::stream::iter(infos)) as BoxedFlightStream<FlightInfo>
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

        let source: AreaSourceReference = request
            .into_inner()
            .try_into()
            .map_err(|_| tonic::Status::invalid_argument("failed to decode command".to_string()))?;

        let schema = self
            .area_store
            .get_schema(&source)
            .await
            .map_err(|e| tonic::Status::not_found(e.to_string()))?;

        let options = IpcWriteOptions::default();
        let schema_result = SchemaAsIpc::new(&schema, &options);

        let is_versioned =
            is_delta_location(self.area_store.object_store(), &source.clone().into())
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let area_info = AreaSourceMetadata {
            source: Some(source),
            is_versioned,
            ..AreaSourceMetadata::default()
        };

        let descriptor = FlightDescriptor {
            r#type: DescriptorType::Cmd.into(),
            cmd: area_info.encode_to_vec(),
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

        let command: AreaSourceReference = request
            .into_inner()
            .try_into()
            .map_err(|_| tonic::Status::invalid_argument("failed to decode command".to_string()))?;

        let schema = self
            .area_store
            .get_schema(&command)
            .await
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

        let request = FlightDoGetRequest::decode(&mut request.into_inner().ticket.as_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let result = match request.command {
            Some(op) => match op {
                DoGetCommand::Kql(_) => {
                    todo!()
                }
                DoGetCommand::Read(read) => self.execute_do_get(read).await,
                DoGetCommand::Query(query) => self.execute_do_get(query).await,
                DoGetCommand::Delta(operation) => self.execute_do_get(operation).await,
            },
            None => Err(FusionServiceError::unknown_action(
                "No operation data passed",
            )),
        }
        .map_err(|e| Status::invalid_argument(format!("Error executing operation - {:?}", e)))?;

        let (tx, rx): (FlightDataSender, FlightDataReceiver) = channel(2);

        // Arrow IPC reader does not implement Sync + Send so we need to use a channel to communicate
        tokio::task::spawn(async move {
            if let Err(e) = stream_flight_data(result, tx).await {
                tracing::error!("Error streaming results: {:?}", e);
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::DoGetStream
        ))
    }

    #[instrument(skip(self, request))]
    async fn do_put(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let stream = request.into_inner();

        // the schema should be the first message returned, else client should error
        let (data_stream, request_data) = raw_stream_to_flight_data_stream(stream)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let body = match &request_data.command {
            Some(action) => {
                let result_body = match action {
                    DoPutCommand::Storage(storage) => serialize_message(
                        self.handle_do_put(storage.clone(), data_stream)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                    DoPutCommand::Delta(delta) => serialize_message(
                        self.handle_do_put(delta.clone(), data_stream)
                            .await
                            .map_err(|e| tonic::Status::internal(e.to_string()))?,
                    ),
                };

                Ok(result_body)
            }
            None => Err(FusionServiceError::unknown_action("No action data passed")),
        }
        .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(Box::pin(futures::stream::once(async {
            Ok(PutResult { app_metadata: body })
        })) as BoxedFlightStream<PutResult>))
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
                };

                Ok(result_body)
            }
            None => Err(FusionServiceError::unknown_action("No action data passed")),
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

#[cfg(test)]
mod tests {
    use super::*;
    use flight_fusion_ipc::AreaTableLocation;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, delta_operation_request::Operation,
        flight_do_get_request::Command, DeltaOperationRequest, DeltaReadOperation,
        FlightDoGetRequest,
    };
    use futures::{StreamExt, TryStreamExt};

    #[tokio::test]
    async fn test_list_flights() {
        let handler = crate::test_utils::get_test_data_fusion_handler();

        let command = CommandListSources { recursive: true };
        let criteria = Criteria {
            expression: command.encode_to_vec(),
        };

        let flights = handler
            .list_flights(Request::new(criteria))
            .await
            .unwrap()
            .into_inner()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        assert_eq!(flights.len(), 3)
    }

    #[tokio::test]
    async fn get_schema() {
        let handler = crate::test_utils::get_test_data_fusion_handler();
        let ref_schema = crate::test_utils::get_test_data_schema();

        let location = AreaTableLocation {
            name: "simple".to_string(),
            areas: vec!["delta".to_string()],
        };

        let request: Request<FlightDescriptor> = Request::new(location.into());
        let result = handler.get_schema(request).await.unwrap().into_inner();

        let arrow_schema: Arc<Schema> = Arc::new(IpcMessage(result.schema).try_into().unwrap());
        assert_eq!(ref_schema, arrow_schema)
    }

    #[tokio::test]
    async fn get_flight_info() {
        let handler = crate::test_utils::get_test_data_fusion_handler();
        let ref_schema = crate::test_utils::get_test_data_schema();

        let location = AreaTableLocation {
            name: "simple".to_string(),
            areas: vec!["delta".to_string()],
        };

        let request: Request<FlightDescriptor> = Request::new(location.into());
        let info = handler.get_flight_info(request).await.unwrap().into_inner();

        let arrow_schema: Arc<Schema> = Arc::new(IpcMessage(info.schema).try_into().unwrap());
        assert_eq!(ref_schema, arrow_schema)
    }

    #[tokio::test]
    async fn do_get_delta() {
        let handler = crate::test_utils::get_test_data_fusion_handler();
        let ref_schema = crate::test_utils::get_test_data_schema();

        let op = FlightDoGetRequest {
            command: Some(Command::Delta(DeltaOperationRequest {
                source: Some(AreaSourceReference {
                    table: Some(TableReference::Location(AreaTableLocation {
                        name: "date".to_string(),
                        areas: vec!["delta".to_string(), "partitioned".to_string()],
                    })),
                }),
                operation: Some(Operation::Read(DeltaReadOperation::default())),
            })),
        };
        let ticket = Ticket {
            ticket: op.encode_to_vec(),
        };
        let response = handler
            .do_get(Request::new(ticket))
            .await
            .unwrap()
            .into_inner()
            .next()
            .await
            .unwrap()
            .unwrap();

        let schema = Arc::new(Schema::try_from(&response).unwrap());

        assert_eq!(ref_schema, schema)
    }
}

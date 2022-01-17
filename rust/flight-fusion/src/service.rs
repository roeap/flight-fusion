use crate::{handlers::FusionActionHandler, stream::FlightReceiverPlan};
use arrow_flight::{
    flight_descriptor::DescriptorType, flight_service_server::FlightService, Action, ActionType,
    Criteria, Empty, FlightData, FlightDescriptor, FlightInfo, HandshakeRequest, HandshakeResponse,
    PutResult, SchemaResult, Ticket,
};
use flight_fusion_ipc::{
    CommandListSources, FlightActionRequest, FlightDoGetRequest, FlightGetFlightInfoRequest,
    FlightGetSchemaRequest,
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

pub struct FlightFusionService {
    action_handler: Arc<FusionActionHandler>,
}

impl FlightFusionService {
    pub fn new_default(root: impl Into<PathBuf>) -> Self {
        let action_handler = Arc::new(FusionActionHandler::new(root));
        Self { action_handler }
    }

    pub fn new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Self {
        let action_handler =
            Arc::new(FusionActionHandler::new_azure(account, access_key, container_name).unwrap());
        Self { action_handler }
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
        let command = CommandListSources::decode(&mut criteria.expression.as_ref())
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(
            self.action_handler
                .list_flights(command)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
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

        let descriptor = request.into_inner();
        let command = match DescriptorType::from_i32(descriptor.r#type) {
            Some(DescriptorType::Cmd) => {
                let request_data = FlightGetFlightInfoRequest::decode(&mut descriptor.cmd.as_ref())
                    .map_err(|e| tonic::Status::internal(e.to_string()))?;
                Ok(request_data)
            }
            _ => Err(tonic::Status::internal(
                "`get_schema` requires command to be defined on flight descriptor",
            )),
        }?;

        Ok(Response::new(
            self.action_handler
                .get_flight_info(command)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        ))
    }

    #[instrument(skip(self, request))]
    async fn get_schema(
        &self,
        request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let descriptor = request.into_inner();
        let command = match DescriptorType::from_i32(descriptor.r#type) {
            Some(DescriptorType::Cmd) => {
                let request_data = FlightGetSchemaRequest::decode(&mut descriptor.cmd.as_ref())
                    .map_err(|e| tonic::Status::internal(e.to_string()))?;
                Ok(request_data)
            }
            _ => Err(tonic::Status::internal(
                "`get_schema` requires command to be defined on flight descriptor",
            )),
        }?;

        Ok(Response::new(
            self.action_handler
                .get_schema(command)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
        ))
    }

    #[instrument(skip(self, request))]
    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let parent_cx =
            global::get_text_map_propagator(|prop| prop.extract(&MetadataMap(request.metadata())));
        tracing::Span::current().set_parent(parent_cx);

        let flight_ticket = request.into_inner();
        let request_data: FlightDoGetRequest =
            FlightDoGetRequest::decode(&mut flight_ticket.ticket.as_ref())
                .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(
            self.action_handler
                .execute_do_get(request_data)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
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

        Ok(Response::new(
            self.action_handler
                .execute_do_put(Arc::new(
                    FlightReceiverPlan::try_new(request.into_inner())
                        .await
                        .map_err(|e| tonic::Status::internal(e.to_string()))?,
                ))
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
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

        Ok(Response::new(
            self.action_handler
                .execute_action(request_data)
                .await
                .map_err(|e| tonic::Status::internal(e.to_string()))?,
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

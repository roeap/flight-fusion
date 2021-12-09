use crate::handlers::{fusion::FlightFusionHandler, FlightHandlerRegistry, FusionActionHandler};
use arrow_flight::{
    flight_service_server::FlightService, Action, ActionType, Criteria, Empty, FlightData,
    FlightDescriptor, FlightInfo, HandshakeRequest, HandshakeResponse, PutResult, SchemaResult,
    Ticket,
};

use flight_fusion_ipc::{FlightActionRequest, FlightDoGetRequest};
use futures::Stream;
use prost::Message;
use std::io::Cursor;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

type BoxedFlightStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

pub struct FlightFusionService {
    handlers: Arc<FlightHandlerRegistry>,
    action_handler: Arc<FusionActionHandler>,
}

impl FlightFusionService {
    pub fn new_default() -> Self {
        let handlers = FlightHandlerRegistry::new();
        let fusion_handler = Arc::new(FlightFusionHandler::new());
        handlers.register_do_put_handler("fusion".to_string(), fusion_handler.clone());

        Self {
            handlers: Arc::new(handlers),
            action_handler: Arc::new(FusionActionHandler::new()),
        }
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

    async fn handshake(
        &self,
        _request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn get_schema(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_get(
        &self,
        request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let flight_ticket = request.into_inner();
        let mut buf = Cursor::new(&flight_ticket.ticket);
        let request_data: FlightDoGetRequest = FlightDoGetRequest::decode(&mut buf)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let result = self
            .action_handler
            .execute_do_get(request_data)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(result))
    }

    async fn do_put(
        &self,
        request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        let mut input_stream = request.into_inner();
        let flight_data = input_stream
            .message()
            .await?
            .ok_or_else(|| Status::invalid_argument("Must send some FlightData"))?;
        let handler = self.handlers.get_do_put("fusion").unwrap();
        let result = handler.do_put(flight_data, input_stream).await?;
        Ok(Response::new(result))
    }

    async fn do_action(
        &self,
        request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        // Decode FlightRequest from buffer.
        let flight_action = request.into_inner();
        let mut buf = Cursor::new(&flight_action.body);
        let request_data: FlightActionRequest = FlightActionRequest::decode(&mut buf)
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        let response = self
            .action_handler
            .execute_action(request_data)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(Response::new(response))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }

    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

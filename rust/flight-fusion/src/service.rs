use crate::handlers::{fusion::FlightFusionHandler, FlightHandlerRegistry};
use arrow_flight::{
    flight_service_server::FlightService, Action, ActionType, Criteria, Empty, FlightData,
    FlightDescriptor, FlightInfo, HandshakeRequest, HandshakeResponse, PutResult, SchemaResult,
    Ticket,
};
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status, Streaming};

type BoxedFlightStream<T> = Pin<Box<dyn Stream<Item = Result<T, Status>> + Send + Sync + 'static>>;

pub struct FlightFusionService {
    handlers: Arc<FlightHandlerRegistry>,
}

impl FlightFusionService {
    pub fn new_default() -> Self {
        let handlers = FlightHandlerRegistry::new();
        let fusion_handler = Arc::new(FlightFusionHandler::new());
        handlers.register_do_get_handler("fusion".to_string(), fusion_handler.clone());
        handlers.register_do_put_handler("fusion".to_string(), fusion_handler.clone());
        handlers.register_action_handler("fusion".to_string(), fusion_handler);

        Self {
            handlers: Arc::new(handlers),
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
        let ticket = request.into_inner();
        let handler = self.handlers.get_do_get("fusion").unwrap();
        let result = handler.do_get(ticket).await?;
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
        let action = request.into_inner();
        let handler = self.handlers.get_action("fusion").unwrap();
        let result = handler.do_action(action).await?;
        Ok(Response::new(result))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        let handler = self.handlers.get_action("fusion").unwrap();
        let result = handler.list_actions().await?;
        Ok(Response::new(result))
    }

    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Not yet implemented"))
    }
}

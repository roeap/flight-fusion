use super::BoxedFlightStream;
use flight_fusion_rpc::{
    flight_action_request::Action as FusionAction, DropDatasetRequest, DropDatasetResponse,
    FlightActionRequest, FlightFusionError, RegisterDatasetAction, RegisterDatasetResponse,
    RequestFor, Result as FusionResult,
};
use prost::Message;

#[async_trait::async_trait]
pub trait RequestHandler<T>: Sync + Send
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

pub struct ActionHandler;

impl ActionHandler {
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
                    FusionAction::Drop(drop) => {
                        serialize_message(self.handle(drop).await?)
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
}

#[async_trait::async_trait]
impl RequestHandler<DropDatasetRequest> for ActionHandler {
    async fn handle(&self, action: DropDatasetRequest) -> FusionResult<DropDatasetResponse> {
        let response = DropDatasetResponse {
            name: action.name,
        };
        Ok(response)
    }
}

#[async_trait::async_trait]
impl RequestHandler<RegisterDatasetAction> for ActionHandler {
    async fn handle(&self, _act: RegisterDatasetAction) -> FusionResult<RegisterDatasetResponse> {
        println!("RegisterDatasetAction");
        todo!()
    }
}

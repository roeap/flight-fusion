use arrow::{ipc::writer::IpcWriteOptions, record_batch::RecordBatch};
use arrow_flight::{
    flight_descriptor, flight_service_client::FlightServiceClient, Action, FlightData,
    FlightDescriptor, PutResult, SchemaAsIpc,
};
use error::FusionClientError;
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction, utils::serialize_message, DatasetFormat,
    DropDatasetRequest, DropDatasetResponse, FlightActionRequest, FlightDoPutRequest,
    RegisterDatasetRequest, RegisterDatasetResponse, RequestFor,
};
use prost::Message;
use std::io::Cursor;
use tonic::{metadata::MetadataValue, service::Interceptor, transport::Channel};

pub mod error;
pub use arrow;

pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

const AUTH_TOKEN_KEY: &str = "auth-token-bin";

#[inline]
fn response_message<T: prost::Message + Default>(
    msg: arrow_flight::Result,
) -> Result<T, FusionClientError> {
    let mut buf = Cursor::new(&msg.body);
    Ok(T::decode(&mut buf)?)
}

#[derive(Clone, Debug)]
pub struct FlightFusionClient {
    //     token: Vec<u8>,
    client: FlightServiceClient<Channel>,
}

impl FlightFusionClient {
    pub async fn try_new() -> Result<Self, FusionClientError> {
        let client = FlightServiceClient::connect("http://localhost:50051")
            .await
            .unwrap();

        Ok(Self { client })
    }

    // #[tracing::instrument(level = "debug", skip(self, v))]
    pub async fn drop_table<T>(
        &self,
        table_name: T,
    ) -> Result<DropDatasetResponse, FusionClientError>
    where
        T: Into<String>,
    {
        let action_request = FlightActionRequest {
            action: Some(FusionAction::Drop(DropDatasetRequest {
                name: table_name.into(),
            })),
        };
        let result = self
            .do_action::<DropDatasetRequest, DropDatasetResponse>(action_request)
            .await?;
        Ok(result)
    }

    pub async fn register_batches(
        &self,
        batches: Vec<RecordBatch>,
        ticket: FlightDoPutRequest,
    ) -> Result<Option<PutResult>, FusionClientError> {
        let options = IpcWriteOptions::default();

        // Create initial message with schema and flight descriptor
        let schema = batches[0].schema();
        let descriptor = FlightDescriptor {
            r#type: flight_descriptor::DescriptorType::Cmd as i32,
            cmd: serialize_message(ticket),
            ..FlightDescriptor::default()
        };
        let mut schema_flight_data: FlightData = SchemaAsIpc::new(&schema, &options).into();
        schema_flight_data.flight_descriptor = Some(descriptor);
        let mut flights: Vec<FlightData> = vec![schema_flight_data];

        // Write data to subsequent messages
        let mut flight_batches = batches
            .iter()
            .flat_map(|batch| {
                let (flight_dictionaries, flight_batch) =
                    arrow_flight::utils::flight_data_from_arrow_batch(batch, &options);
                flight_dictionaries
                    .into_iter()
                    .chain(std::iter::once(flight_batch))
            })
            .collect::<Vec<_>>();
        flights.append(&mut flight_batches);

        let request = futures::stream::iter(flights);
        let mut result = self.client.clone().do_put(request).await?.into_inner();
        Ok(result.message().await?)
    }

    pub async fn register_dataset<S, T, P>(
        &self,
        _schema_name: S,
        table_name: T,
        path: P,
    ) -> Result<RegisterDatasetResponse, FusionClientError>
    where
        S: Into<String>,
        T: Into<String>,
        P: Into<String>,
    {
        let action_request = FlightActionRequest {
            action: Some(FusionAction::Register(RegisterDatasetRequest {
                name: table_name.into(),
                format: DatasetFormat::File.into(),
                path: path.into(),
            })),
        };
        let result = self
            .do_action::<RegisterDatasetRequest, RegisterDatasetResponse>(action_request)
            .await?;
        Ok(result)
    }

    // #[tracing::instrument(level = "debug", skip(self, v))]
    pub(crate) async fn do_action<T, R>(
        &self,
        request: FlightActionRequest,
    ) -> Result<R, FusionClientError>
    where
        T: RequestFor<Reply = R>,
        R: prost::Message + Default,
    {
        let mut buf = Vec::new();
        buf.reserve(request.encoded_len());
        request.encode(&mut buf).unwrap();

        let action = Action {
            r#type: String::from(""),
            body: buf,
        };

        let mut stream = self.client.clone().do_action(action).await?.into_inner();
        match stream.message().await? {
            // TODO do something more meaningful here. Should we communicate detailed results in message?
            None => Err(FusionClientError::TableAlreadyExists("asd".to_string())),
            Some(result) => Ok(response_message::<T::Reply>(result)?),
        }
    }
}

#[derive(Clone)]
pub struct AuthInterceptor {
    pub token: Vec<u8>,
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut req: tonic::Request<()>,
    ) -> std::result::Result<tonic::Request<()>, tonic::Status> {
        let metadata = req.metadata_mut();
        metadata.insert_bin(AUTH_TOKEN_KEY, MetadataValue::from_bytes(&self.token));
        Ok(req)
    }
}

use arrow::{ipc::writer::IpcWriteOptions, record_batch::RecordBatch};
use arrow_flight::{
    flight_descriptor, flight_service_client::FlightServiceClient, Action, FlightData,
    FlightDescriptor, SchemaAsIpc,
};
use error::FusionClientError;
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction, flight_do_put_request, utils::serialize_message,
    DatasetFormat, DropDatasetRequest, DropDatasetResponse, FlightActionRequest,
    FlightDoPutRequest, PutMemoryTableRequest, PutMemoryTableResponse, RegisterDatasetRequest,
    RegisterDatasetResponse, RequestFor,
};
use observability_deps::instrument;
use observability_deps::tracing;
use std::io::Cursor;
use tonic::{
    codegen::InterceptedService,
    transport::{Channel, Endpoint},
};

pub mod error;
mod interceptor;
pub use arrow;

pub fn crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[inline]
fn response_message<T: prost::Message + Default>(
    msg: arrow_flight::Result,
) -> Result<T, FusionClientError> {
    let mut buf = Cursor::new(&msg.body);
    Ok(T::decode(&mut buf)?)
}

#[derive(Clone, Debug)]
pub struct FlightFusionClient {
    client: FlightServiceClient<InterceptedService<Channel, interceptor::TracingInterceptor>>,
    // client: FlightServiceClient<Channel>,
}

impl FlightFusionClient {
    pub async fn try_new() -> Result<Self, FusionClientError> {
        let channel = Endpoint::from_static("http://localhost:50051")
            .connect()
            .await?;
        let interceptor = interceptor::TracingInterceptor {};
        let intercepted_client = FlightServiceClient::with_interceptor(channel, interceptor);
        // let intercepted_client = FlightServiceClient::new(channel);
        Ok(Self {
            client: intercepted_client,
        })
    }

    #[instrument(skip(self, batches))]
    pub async fn put_memory_table<T>(
        &self,
        table_name: T,
        batches: Vec<RecordBatch>,
    ) -> Result<PutMemoryTableResponse, FusionClientError>
    where
        T: Into<String> + std::fmt::Debug,
    {
        let operation = flight_do_put_request::Operation::Memory(PutMemoryTableRequest {
            name: table_name.into(),
        });
        Ok(self
            .do_put::<PutMemoryTableResponse>(batches, operation)
            .await
            .unwrap()
            .unwrap())
    }

    #[instrument(skip(self))]
    pub async fn drop_table<T>(
        &self,
        table_name: T,
    ) -> Result<DropDatasetResponse, FusionClientError>
    where
        T: Into<String> + std::fmt::Debug,
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

    pub async fn do_put<R>(
        &self,
        batches: Vec<RecordBatch>,
        operation: flight_do_put_request::Operation,
    ) -> Result<Option<R>, FusionClientError>
    where
        R: prost::Message + Default,
    {
        let options = IpcWriteOptions::default();

        // Create initial message with schema and flight descriptor
        let schema = batches[0].schema();
        let descriptor = FlightDescriptor {
            r#type: flight_descriptor::DescriptorType::Cmd as i32,
            cmd: serialize_message(FlightDoPutRequest {
                operation: Some(operation),
            }),
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
        match result.message().await? {
            Some(msg) => Ok(Some(R::decode(msg.app_metadata.as_ref())?)),
            _ => Ok(None),
        }
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
        let action = Action {
            r#type: String::from(""),
            body: serialize_message(request),
        };
        let mut stream = self.client.clone().do_action(action).await?.into_inner();
        match stream.message().await? {
            // TODO do something more meaningful here. Should we communicate detailed results in message?
            None => Err(FusionClientError::TableAlreadyExists("asd".to_string())),
            Some(result) => Ok(response_message::<T::Reply>(result)?),
        }
    }
}

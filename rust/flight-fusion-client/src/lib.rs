use arrow::{
    datatypes::Schema as ArrowSchema, ipc::writer::IpcWriteOptions, record_batch::RecordBatch,
};
use arrow_flight::{
    flight_descriptor, flight_service_client::FlightServiceClient, Action, FlightData,
    FlightDescriptor, SchemaAsIpc, Ticket,
};
use error::FusionClientError;
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction, flight_do_get_request::Command as DoGetCommand,
    flight_do_put_request::Command as DoPutCommand, utils::serialize_message, CommandDropSource,
    CommandReadDataset, CommandRegisterSource, CommandWriteIntoDataset, DatasetFormat,
    FlightActionRequest, FlightDoGetRequest, FlightDoPutRequest, FlightFusionError,
    PutMemoryTableRequest, RequestFor, ResultActionStatus, ResultDoPutUpdate,
};
use observability_deps::instrument;
use observability_deps::tracing;
use std::io::Cursor;
use std::str::FromStr;
use tonic::{
    codegen::InterceptedService,
    transport::{Channel, Endpoint},
};
pub mod error;
pub use arrow;
pub use flight_fusion_ipc;
use futures::TryStreamExt;
use std::sync::Arc;

mod interceptor;

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
    pub async fn try_new<H>(host: H, port: i32) -> Result<Self, FusionClientError>
    where
        H: Into<String>,
    {
        let address = format!("http://{}:{}", host.into(), port);
        let channel = Endpoint::from_str(&address).unwrap().connect().await?;
        let interceptor = interceptor::TracingInterceptor {};
        let intercepted_client = FlightServiceClient::with_interceptor(channel, interceptor);
        // let intercepted_client = FlightServiceClient::new(channel);
        Ok(Self {
            client: intercepted_client,
        })
    }

    #[instrument(skip(self, batches))]
    pub async fn write_into_table(
        &self,
        command: CommandWriteIntoDataset,
        batches: Vec<RecordBatch>,
    ) -> Result<ResultDoPutUpdate, FusionClientError> {
        let operation = DoPutCommand::Storage(command);
        Ok(self
            .do_put::<ResultDoPutUpdate>(batches, operation)
            .await?
            .unwrap())
    }

    #[instrument(skip(self))]
    pub async fn read_table(
        &self,
        command: CommandReadDataset,
    ) -> Result<Vec<RecordBatch>, FusionClientError> {
        let ticket = Ticket {
            ticket: serialize_message(FlightDoGetRequest {
                command: Some(DoGetCommand::Read(command)),
            }),
        };
        // TODO Don't panic
        let response = self.client.clone().do_get(ticket).await?;
        collect_response_stream(response.into_inner()).await
    }

    #[instrument(skip(self, batches))]
    pub async fn put_memory_table<T>(
        &self,
        table_ref: T,
        batches: Vec<RecordBatch>,
    ) -> Result<ResultDoPutUpdate, FusionClientError>
    where
        T: Into<String> + std::fmt::Debug,
    {
        let operation = DoPutCommand::Memory(PutMemoryTableRequest {
            name: table_ref.into(),
        });
        Ok(self
            .do_put::<ResultDoPutUpdate>(batches, operation)
            .await?
            .unwrap())
    }

    #[instrument(skip(self))]
    pub async fn drop_table(
        &self,
        command: CommandDropSource,
    ) -> Result<ResultActionStatus, FusionClientError> {
        let action_request = FlightActionRequest {
            action: Some(FusionAction::Drop(command)),
        };
        let result = self
            .do_action::<CommandDropSource, ResultActionStatus>(action_request)
            .await?;
        Ok(result)
    }

    #[instrument(skip(self))]
    pub async fn execute_query(
        &self,
        query: String,
    ) -> Result<Vec<RecordBatch>, FusionClientError> {
        todo!()
    }

    pub async fn register_dataset<S, T, P>(
        &self,
        _schema_name: S,
        table_name: T,
        path: P,
    ) -> Result<ResultActionStatus, FusionClientError>
    where
        S: Into<String>,
        T: Into<String>,
        P: Into<String>,
    {
        let action_request = FlightActionRequest {
            action: Some(FusionAction::Register(CommandRegisterSource {
                name: table_name.into(),
                format: DatasetFormat::File.into(),
                path: path.into(),
            })),
        };
        let result = self
            .do_action::<CommandRegisterSource, ResultActionStatus>(action_request)
            .await?;
        Ok(result)
    }

    pub async fn do_put<R>(
        &self,
        batches: Vec<RecordBatch>,
        operation: DoPutCommand,
    ) -> Result<Option<R>, FusionClientError>
    where
        R: prost::Message + Default,
    {
        let options = IpcWriteOptions::default();

        // Create initial message with schema and flight descriptor
        let schema = batches[0].schema();
        let descriptor = FlightDescriptor {
            r#type: flight_descriptor::DescriptorType::Cmd.into(),
            cmd: serialize_message(FlightDoPutRequest {
                command: Some(operation),
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

async fn collect_response_stream(
    mut stream: tonic::Streaming<FlightData>,
) -> Result<Vec<RecordBatch>, FusionClientError> {
    // TODO get rid of all the panics
    let flight_data = stream.message().await.unwrap().unwrap();
    let schema = Arc::new(
        ArrowSchema::try_from(&flight_data)
            .map_err(|e| FlightFusionError::ExternalError(format!("Invalid schema: {:?}", e)))
            .unwrap(),
    );

    let to_batch = |flight_data| {
        let dictionaries_by_field = vec![None; schema.fields().len()];
        arrow_flight::utils::flight_data_to_arrow_batch(
            &flight_data,
            schema.clone(),
            &dictionaries_by_field,
        )
        .unwrap()
    };

    Ok(stream
        .try_collect::<Vec<_>>()
        .await
        .unwrap()
        .into_iter()
        .map(to_batch)
        .collect::<Vec<_>>())
}

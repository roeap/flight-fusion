use arrow_flight::{flight_service_client::FlightServiceClient, Action};
use flight_fusion_ipc::{
    flight_action_request::Action as FusionAction, DatasetFormat, DropDatasetRequest,
    DropDatasetResponse, FlightActionRequest, RegisterDatasetRequest, RegisterDatasetResponse,
    RequestFor,
};
use prost::{DecodeError, Message};
use std::io::Cursor;
use tonic::{metadata::MetadataValue, service::Interceptor, transport::Channel};

const AUTH_TOKEN_KEY: &str = "auth-token-bin";

#[derive(thiserror::Error, Debug)]
pub enum FusionClientError {
    /// Error returned when the table to be created already exists
    #[error("Table: '{0}' already exists")]
    TableAlreadyExists(String),

    /// Errors during communication with flight server
    #[error("Table: already exists")]
    TransportError {
        #[from]
        source: tonic::transport::Error,
    },

    /// Errors during communication with flight server
    #[error("Unexpected return status: {source}")]
    ReturnCodeError {
        #[from]
        source: tonic::Status,
    },

    #[error("Unexpected return status: {source}")]
    CorruptReturnMessage {
        #[from]
        source: DecodeError,
    },
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
        let mut action = DropDatasetRequest::default();
        action.name = table_name.into();
        let mut action_request = FlightActionRequest::default();
        action_request.action = Some(FusionAction::Drop(action));

        let result = self
            .do_action::<DropDatasetRequest, DropDatasetResponse>(action_request)
            .await?;
        Ok(result)
    }

    pub async fn register_dataset<T>(
        &self,
        _schema_name: T,
        table_name: T,
        path: T,
    ) -> Result<RegisterDatasetResponse, FusionClientError>
    where
        T: Into<String>,
    {
        let action = RegisterDatasetRequest {
            name: table_name.into(),
            format: DatasetFormat::File.into(),
            path: path.into(),
        };

        let action_request = FlightActionRequest {
            action: Some(FusionAction::Register(action)),
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

        match stream.message().await.unwrap() {
            None => Err(FusionClientError::TableAlreadyExists("asd".to_string())),
            Some(result) => Ok(response_message::<T::Reply>(result).unwrap()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = FlightFusionClient::try_new().await.unwrap();
        let response = client.drop_table("table_name").await.unwrap();
        println!("{:?}", response)
    }
}

use crate::{
    clients::{OpenMetadataClient, PagedReturn},
    generated::{Database, EntityReference},
    operations::{CreateDatabaseBuilder, ListDatabasesBuilder},
};
use reqwest_pipeline::{collect_pinned_stream, Context, Response, Result as RPResult};

impl PagedReturn<Database> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

impl Database {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

#[derive(Debug, Clone)]
pub struct DatabasesCollectionClient {
    client: OpenMetadataClient,
}

impl DatabasesCollectionClient {
    pub(crate) fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn create_database(
        &self,
        database_name: String,
        service: EntityReference,
    ) -> CreateDatabaseBuilder {
        CreateDatabaseBuilder::new(self.client.clone(), database_name, service)
    }

    pub fn list_databases(&self) -> ListDatabasesBuilder {
        ListDatabasesBuilder::new(self.client.clone())
    }

    pub fn into_database_client<S: Into<String>>(&self, database_id: S) -> DatabaseClient {
        DatabaseClient::new(self.client.clone(), database_id)
    }
}

pub struct DatabaseClient {
    client: OpenMetadataClient,
    entity_id: String,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<String>>(client: OpenMetadataClient, database_id: S) -> Self {
        Self {
            client,
            entity_id: database_id.into(),
        }
    }

    pub async fn get_database(&self) -> crate::Result<Database> {
        // TODO add optional fields query parameter
        let url = self.client.api_routes().databases().join(&self.entity_id)?;
        let mut request = self.client.prepare_request(url.as_str(), http::Method::GET);
        let response = self
            .client
            .pipeline()
            .send(&mut Context::new(), &mut request)
            .await?;
        Ok(Database::try_from(response).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::clients::{OpenMetadataClient, OpenMetadataOptions};
    use futures_util::StreamExt;

    #[tokio::test]
    async fn test_list_databases() {
        let client = initialize().into_databases_collection_client();

        let databases = Box::pin(client.list_databases().into_stream())
            .next()
            .await
            .unwrap()
            .unwrap();

        assert!(databases.data.len() >= 2)
    }

    #[cfg(not(feature = "mock_transport_framework"))]
    pub fn initialize() -> OpenMetadataClient {
        let client =
            OpenMetadataClient::new("http://localhost:8585", OpenMetadataOptions::default());

        client
    }
}

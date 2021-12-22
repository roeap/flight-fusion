use crate::{
    clients::{OpenMetadataClient, PagedReturn},
    models::Database,
    operations::ListDatabases,
};
use reqwest_pipeline::{collect_pinned_stream, Response, Result as RPResult, Context};

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
    open_meta_client: OpenMetadataClient,
}

impl DatabasesCollectionClient {
    pub(crate) fn new(open_meta_client: OpenMetadataClient) -> Self {
        Self { open_meta_client }
    }

    pub fn list_databases(&self) -> ListDatabases {
        ListDatabases::new(self.open_meta_client.clone())
    }

    pub fn into_database_client<S: Into<String>>(&self, database_id: S) -> DatabaseClient {
        DatabaseClient::new(self.open_meta_client.clone(), database_id)
    }
}

pub struct DatabaseClient {
    open_meta_client: OpenMetadataClient,
    entity_id: String,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<String>>(
        open_meta_client: OpenMetadataClient,
        database_id: S,
    ) -> Self {
        Self {
            open_meta_client,
            entity_id: database_id.into(),
        }
    }

    pub async fn get_database(&self) -> crate::Result<Database> {
        // TODO add optional fields query parameter
        let url = self
            .open_meta_client
            .databases_url()
            .join(&self.entity_id)?;
        let mut request = self
            .open_meta_client
            .prepare_request(url.as_str(), http::Method::GET);
        let response = self
            .open_meta_client
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

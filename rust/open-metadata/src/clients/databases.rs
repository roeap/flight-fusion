use crate::{
    clients::{OpenMetadataClient, PagedReturn},
    models::Database,
    operations::ListDatabases,
};
use reqwest_pipeline::{collect_pinned_stream, Response, Result as RPResult};

impl PagedReturn<Database> {
    pub(crate) async fn try_from(response: Response) -> RPResult<Self> {
        let (_status_code, _headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        Ok(serde_json::from_slice(&body)?)
    }
}

#[derive(Debug, Clone)]
pub struct DatabasesCollectionClient {
    open_meta_client: OpenMetadataClient,
    fully_qualified_name: Option<String>,
    entity_id: Option<String>,
}

impl DatabasesCollectionClient {
    pub(crate) fn new(open_meta_client: OpenMetadataClient) -> Self {
        Self {
            open_meta_client,
            fully_qualified_name: None,
            entity_id: None,
        }
    }

    #[allow(dead_code)]
    pub fn open_meta_client(&self) -> &OpenMetadataClient {
        &self.open_meta_client
    }

    pub fn list_databases(&self) -> ListDatabases {
        ListDatabases::new(self.open_meta_client.clone())
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

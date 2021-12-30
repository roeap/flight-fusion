use super::OpenMetadataClient;
use crate::operations::{CreateStorageServiceBuilder, ListServicesBuilder};

pub struct ServicesClient {
    client: OpenMetadataClient,
}

impl ServicesClient {
    pub(crate) fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn list_services(&self) -> ListServicesBuilder {
        todo!()
    }

    pub fn create_storage_service<T>(&self, name: T) -> CreateStorageServiceBuilder
    where
        T: Into<String>,
    {
        CreateStorageServiceBuilder::new(self.client.clone(), name.into())
    }
}

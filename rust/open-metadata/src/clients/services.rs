use super::OpenMetadataClient;
use crate::{
    generated::{DatabaseServiceType, JdbcInfo},
    operations::{CreateDatabaseServiceBuilder, CreateStorageServiceBuilder, ListServicesBuilder},
};

pub struct ServicesClient {
    client: OpenMetadataClient,
}

impl ServicesClient {
    pub(crate) fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn list_services(&self) -> ListServicesBuilder {
        ListServicesBuilder::new(self.client.clone())
    }

    pub fn create_database_service<T>(
        &self,
        name: T,
        service_type: DatabaseServiceType,
        jdbc: JdbcInfo,
    ) -> CreateDatabaseServiceBuilder
    where
        T: Into<String>,
    {
        CreateDatabaseServiceBuilder::new(self.client.clone(), name.into(), service_type, jdbc)
    }

    pub fn create_storage_service<T>(&self, name: T) -> CreateStorageServiceBuilder
    where
        T: Into<String>,
    {
        CreateStorageServiceBuilder::new(self.client.clone(), name.into())
    }
}

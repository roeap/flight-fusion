use crate::{
    clients::OpenMetadataClient,
    generated::{Database, EntityReference},
    operations::ListTablesBuilder,
};

#[derive(Debug, Clone)]
pub struct TablesCollectionClient {
    client: OpenMetadataClient,
}

impl TablesCollectionClient {
    pub(crate) fn new(client: OpenMetadataClient) -> Self {
        Self { client }
    }

    pub fn list_tables(&self) -> ListTablesBuilder {
        ListTablesBuilder::new(self.client.clone())
    }
}

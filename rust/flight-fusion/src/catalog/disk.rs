//! File based AreaCatalog implementation stored on disk.
use super::AreaCatalog;
use crate::{
    area_store::AreaStore,
    error::{Result, ResultStream},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    AreaReference, AreaSourceDetails, AreaSourceMetadata, AreaSourceReference,
};
use std::sync::Arc;

pub struct FileAreaCatalog {
    store: Arc<dyn AreaStore>,
}

impl FileAreaCatalog {
    pub fn new(store: Arc<dyn AreaStore>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl AreaCatalog for FileAreaCatalog {
    async fn get_source_metadata(
        &self,
        _reference: AreaSourceReference,
    ) -> Result<AreaSourceMetadata> {
        todo!()
    }

    async fn set_source_metadata(
        &self,
        _reference: AreaSourceReference,
        metadata: AreaSourceMetadata,
    ) -> Result<()> {
        todo!()
    }

    async fn register_source(&self, _reference: AreaSourceReference) -> Result<()> {
        todo!()
    }

    async fn list_area_sources(
        &self,
        _root: Option<AreaReference>,
    ) -> ResultStream<AreaSourceMetadata> {
        todo!()
    }

    async fn get_source_details(
        &self,
        _reference: AreaSourceReference,
    ) -> Result<AreaSourceDetails> {
        todo!()
    }
}

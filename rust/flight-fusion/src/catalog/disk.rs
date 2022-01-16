//! File based AreaCatalog implementation stored on disk.
use super::AreaCatalog;
use crate::{
    area_store::AreaStore,
    error::{Result, ResultStream},
};
use async_trait::async_trait;
use flight_fusion_ipc::{AreaReference, AreaSourceMetadata, AreaSourceReference};
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
    async fn get_source_meta(&self, _reference: AreaSourceReference) -> Result<AreaSourceMetadata> {
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
}

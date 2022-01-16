//! File based AreaCatalog implementation stored on disk.
use super::AreaCatalog;
use crate::error::{Result, ResultStream};
use async_trait::async_trait;
use flight_fusion_ipc::{AreaSourceMetadata, AreaSourceReference};

pub struct DiskAreaCatalog {}

#[async_trait]
impl AreaCatalog for DiskAreaCatalog {
    async fn get_source_meta(&self, _reference: AreaSourceReference) -> Result<AreaSourceMetadata> {
        todo!()
    }

    async fn register_source(&self, _reference: AreaSourceReference) -> Result<()> {
        todo!()
    }

    async fn list_area_sources(
        &self,
        _reference: AreaSourceReference,
    ) -> ResultStream<AreaSourceMetadata> {
        todo!()
    }
}

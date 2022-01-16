//! The area catalog provides meta data for all data sources registered in the service.
mod disk;

use crate::error::{Result, ResultStream};
use async_trait::async_trait;
use flight_fusion_ipc::{AreaSourceMetadata, AreaSourceReference};

#[async_trait]
pub trait AreaCatalog {
    /// Fetch metadata for a registered data source
    async fn get_source_meta(&self, reference: AreaSourceReference) -> Result<AreaSourceMetadata>;

    /// Register a new data source in the catalog
    async fn register_source(&self, reference: AreaSourceReference) -> Result<()>;

    async fn list_area_sources(
        &self,
        reference: AreaSourceReference,
    ) -> ResultStream<AreaSourceMetadata>;
}

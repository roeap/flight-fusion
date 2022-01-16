//! The area catalog provides meta data for all data sources registered in the service.
mod disk;

use crate::error::{Result, ResultStream};
use async_trait::async_trait;
pub use disk::*;
use flight_fusion_ipc::{
    AreaReference, AreaSourceDetails, AreaSourceMetadata, AreaSourceReference,
};

#[async_trait]
pub trait AreaCatalog: Send + Sync {
    /// Fetch metadata for a registered data source
    async fn get_source_metadata(
        &self,
        reference: AreaSourceReference,
    ) -> Result<AreaSourceMetadata>;

    /// Set metadata for a registered data source
    async fn set_source_metadata(
        &self,
        reference: AreaSourceReference,
        metadata: AreaSourceMetadata,
    ) -> Result<()>;

    /// Register a new data source in the catalog
    async fn register_source(&self, reference: AreaSourceReference) -> Result<()>;

    /// List all data sources registered in teh catalog
    async fn list_area_sources(
        &self,
        root: Option<AreaReference>,
    ) -> ResultStream<AreaSourceMetadata>;

    /// Get detailed metadata and statistics about data source
    async fn get_source_details(
        &self,
        _reference: AreaSourceReference,
    ) -> Result<AreaSourceDetails>;
}

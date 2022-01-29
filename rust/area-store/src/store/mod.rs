//! Abstractions and implementations for writing data to delta tables
mod cache;
pub mod error;
mod stats;
mod store;
pub mod utils;
pub mod writer;

use std::sync::Arc;

use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::{arrow::ParquetFileArrowReader, basic::LogicalType};
use async_trait::async_trait;
pub use error::*;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
pub use store::DefaultAreaStore;
pub use utils::*;
pub use writer::*;

#[async_trait]
pub trait AreaStore: Send + Sync {
    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<object_store::ObjectStore>;

    // TODO use a more structured reference for table location
    /// Write batches into table location
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &object_store::path::Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>>;

    /// Read batches from location
    async fn get_batches(&self, location: &object_store::path::Path) -> Result<Vec<RecordBatch>>;

    async fn get_arrow_reader(
        &self,
        location: &object_store::path::Path,
    ) -> Result<ParquetFileArrowReader>;

    /// Resolve an [`AreaSourceReference`] to a storage location
    fn get_table_location(&self, source: &AreaSourceReference) -> Result<object_store::path::Path>;
}

//! Abstractions and implementations for writing data to delta tables
mod basic;
mod cache;
pub mod file_index;
mod stats;
pub mod utils;
pub mod writer;

use crate::error::Result;
use arrow_deps::arrow::{datatypes::SchemaRef as ArrowSchemaRef, record_batch::RecordBatch};
use arrow_deps::datafusion::parquet::arrow::ArrowReader;
use arrow_deps::datafusion::parquet::{arrow::ParquetFileArrowReader, basic::LogicalType};
use arrow_deps::datafusion::physical_plan::SendableRecordBatchStream;
use async_trait::async_trait;
pub use basic::DefaultAreaStore;
pub use cache::CachedAreaStore;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use object_store::path::{parsed::DirsAndFileName, Path};
use std::collections::HashSet;
use std::sync::Arc;
pub use utils::*;
pub use writer::*;

const DATA_FOLDER_NAME: &str = "_ff_data";
const DEFAULT_READ_BATCH_SIZE: usize = 1024;

#[async_trait]
pub trait AreaStore: Send + Sync {
    // path manipulation

    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<object_store::ObjectStore>;

    fn get_path_from_raw(&self, raw: String) -> Path;

    /// Resolve an [`AreaSourceReference`] to a storage location
    fn get_table_location(&self, source: &AreaSourceReference) -> Result<Path>;

    async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef>;

    /// Write batches into table location
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>>;

    /// Read batches from location
    async fn get_batches(&self, location: &Path) -> Result<Vec<RecordBatch>>;

    async fn get_arrow_reader(&self, location: &Path) -> Result<ParquetFileArrowReader>;

    async fn read_file(&self, file: &Path) -> Result<Vec<RecordBatch>> {
        let mut reader = self.get_arrow_reader(file).await?;
        let batch_reader = reader.get_record_reader(DEFAULT_READ_BATCH_SIZE)?;
        Ok(batch_reader
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// Resolve an [`AreaSourceReference`] to the files relevant for source reference
    async fn get_source_files(&self, source: &AreaSourceReference) -> Result<Vec<Path>> {
        let location = self.get_table_location(source)?;
        self.get_location_files(&location).await
    }

    async fn get_location_files(&self, location: &Path) -> Result<Vec<Path>> {
        flatten_list_stream(&self.object_store(), Some(location)).await
    }

    async fn list_table_locations(&self) -> Result<Vec<AreaSourceReference>> {
        Ok(flatten_list_stream(&self.object_store(), None)
            .await?
            .into_iter()
            .map(|p| DirsAndFileName::from(p).directories)
            .collect::<HashSet<_>>()
            .into_iter()
            .filter_map(path_to_source)
            .collect::<Vec<_>>())
    }
}

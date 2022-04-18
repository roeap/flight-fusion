//! Abstractions and implementations for writing data to delta tables
mod basic;
mod cache;
pub mod error;
mod stats;
pub mod utils;
pub mod writer;

use arrow_deps::arrow::{datatypes::*, record_batch::RecordBatch};
use arrow_deps::datafusion::parquet::arrow::ArrowReader;
use arrow_deps::datafusion::parquet::{arrow::ParquetFileArrowReader, basic::LogicalType};
use async_trait::async_trait;
pub use basic::DefaultAreaStore;
pub use cache::CachedAreaStore;
pub use error::*;
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
    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<object_store::ObjectStore>;

    fn get_path_from_raw(&self, raw: String) -> Path;

    // TODO use a more structured reference for table location
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

    /// Resolve an [`AreaSourceReference`] to a storage location
    fn get_table_location(&self, source: &AreaSourceReference) -> Result<Path>;

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

#[cfg(test)]
mod tests {
    use arrow_deps::datafusion::parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder;
    use futures::TryStreamExt;
    use object_store::path::ObjectStorePath;
    use object_store::{ObjectStore, ObjectStoreApi};

    #[tokio::test]
    async fn test_async_read_2() {
        let integration =
            ObjectStore::new_file("/home/robstar/github/flight-fusion/.flight-fusion/.fusion");

        let mut location = integration.new_path();
        location.push_dir("demo");
        location.push_dir("hacker");
        location.push_dir("_ff_data");
        location.push_dir("comments");
        location
            .set_file_name("part-00000-3ae45d64-1c6f-40ea-a641-36d8e2f76dc4-c000.snappy.parquet");

        let file = integration.open_file(&location).await.unwrap();

        let builder = ParquetRecordBatchStreamBuilder::new(file)
            .await
            .unwrap()
            .with_batch_size(3);
        let metadata = builder.schema();
        println!("{:?}", metadata)
    }

    #[tokio::test]
    async fn test_async_read_3() {
        let integration = ObjectStore::new_file("/home/robstar/github/flight-fusion/.tmp");

        let mut location = integration.new_path();
        location.push_dir("file");
        // location.push_dir("hacker");
        // location.push_dir("_ff_data");
        // location.push_dir("comments");
        location.set_file_name("table.parquet");

        let file = integration.open_file(&location).await.unwrap();
        // let path = "/home/robstar/github/flight-fusion/.flight-fusion/.fusion/demo/hacker/_ff_data/comments/part-00000-3ae45d64-1c6f-40ea-a641-36d8e2f76dc4-c000.snappy.parquet";
        // let file = tokio::fs::File::open(path).await.unwrap();
        let builder = ParquetRecordBatchStreamBuilder::new(file)
            .await
            .unwrap()
            .with_batch_size(3);
        // let metadata = builder.schema();

        let stream = builder.build().unwrap();
        let results = stream.try_collect::<Vec<_>>().await.unwrap();

        println!("{:?}", results)
    }
}

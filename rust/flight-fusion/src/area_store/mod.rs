//! Abstractions and implementations for writing data to delta tables
pub mod error;
mod stats;
pub mod utils;
pub mod writer;

use std::path::PathBuf;
use std::sync::Arc;

use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    basic::LogicalType,
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use async_trait::async_trait;
pub use error::*;
use flight_fusion_ipc::{
    area_source_reference::Table as TableReference, AreaSourceReference, SaveMode,
};
use futures::{stream, StreamExt, TryStreamExt};
use object_store::{path::ObjectStorePath, ObjectStoreApi};
pub use utils::*;
pub use writer::*;

const DATA_FOLDER_NAME: &str = "data";
const DEFAULT_READ_BATCH_SIZE: usize = 1024;

type AuxError = Box<dyn std::error::Error + Send + Sync + 'static>;
type AuxResult<T, E = AuxError> = std::result::Result<T, E>;

pub async fn flatten_list_stream(
    storage: &object_store::ObjectStore,
    prefix: Option<&object_store::path::Path>,
) -> AuxResult<Vec<object_store::path::Path>> {
    storage
        .list(prefix)
        .await?
        .map_ok(|v| stream::iter(v).map(Ok))
        .try_flatten()
        .try_collect()
        .await
}

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

pub struct InMemoryAreaStore {
    object_store: Arc<object_store::ObjectStore>,
}

impl InMemoryAreaStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let object_store = Arc::new(object_store::ObjectStore::new_file(root));
        Self { object_store }
    }

    pub fn new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Result<Self> {
        let object_store = Arc::new(object_store::ObjectStore::new_microsoft_azure(
            account,
            access_key,
            container_name,
            false,
        )?);
        Ok(Self { object_store })
    }
}

#[async_trait]
impl AreaStore for InMemoryAreaStore {
    fn object_store(&self) -> Arc<object_store::ObjectStore> {
        self.object_store.clone()
    }

    fn get_table_location(&self, source: &AreaSourceReference) -> Result<object_store::path::Path> {
        match source {
            AreaSourceReference { table: Some(tbl) } => match tbl {
                TableReference::Location(loc) => {
                    let mut location = self.object_store().path_from_raw(&loc.areas.join("/"));
                    location.push_dir(DATA_FOLDER_NAME);
                    location.push_dir(&loc.name);
                    Ok(location)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }

    // TODO use some sort of borrowed reference
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &object_store::path::Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>> {
        let schema = batches[0].schema();
        let partition_cols = vec![];
        let mut writer = DeltaWriter::new(self.object_store(), schema, Some(partition_cols));
        batches.iter().for_each(|b| writer.write(b).unwrap());

        match save_mode {
            SaveMode::Overwrite => {
                let files = flatten_list_stream(&self.object_store(), Some(location))
                    .await
                    .unwrap();
                for file in files {
                    // TODO remove panic
                    self.object_store().delete(&file).await.unwrap();
                }
                writer.flush(location).await
            }
            // TODO actually check if exists
            SaveMode::ErrorIfExists => Err(AreaStoreError::TableAlreadyExists(location.to_raw())),
            _ => writer.flush(location).await,
        }
    }

    /// Read batches from location
    async fn get_batches(&self, location: &object_store::path::Path) -> Result<Vec<RecordBatch>> {
        let files = flatten_list_stream(&self.object_store(), Some(location))
            .await
            .unwrap();
        let mut batches = Vec::new();
        for file in files {
            // TODO remove panic
            let mut reader = self.get_arrow_reader(&file).await?;
            let batch_reader = reader.get_record_reader(DEFAULT_READ_BATCH_SIZE).unwrap();
            let mut file_batches = batch_reader
                .into_iter()
                .map(|batch| batch.unwrap())
                .collect::<Vec<_>>();
            batches.append(&mut file_batches);
        }
        Ok(batches)
    }

    async fn get_arrow_reader(
        &self,
        location: &object_store::path::Path,
    ) -> Result<ParquetFileArrowReader> {
        let bytes = self
            .object_store()
            .get(location)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let cursor = SliceableCursor::new(Arc::new(bytes));
        // TODO remove panic and return result
        let file_reader = Arc::new(SerializedFileReader::new(cursor)?);
        Ok(ParquetFileArrowReader::new(file_reader))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = InMemoryAreaStore::new(root.path());
        let location = area_store.object_store().new_path();

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(vec![batch.clone()], &location, SaveMode::Append)
            .await
            .unwrap();

        let read_batch = area_store.get_batches(&location).await.unwrap();

        assert_eq!(batch, read_batch[0])
    }
}

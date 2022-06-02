//! Abstractions and implementations for writing data to delta tables
mod basic;
mod cache;
// mod file_index;
mod area_path;
mod stats;
pub mod writer;

use crate::error::Result;
pub use area_path::*;
use arrow_deps::arrow::{datatypes::SchemaRef as ArrowSchemaRef, record_batch::RecordBatch};
use arrow_deps::datafusion::arrow::record_batch::RecordBatchReader;
use arrow_deps::datafusion::parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use arrow_deps::datafusion::parquet::{
    basic::LogicalType,
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use arrow_deps::datafusion::physical_plan::{
    stream::RecordBatchStreamAdapter, SendableRecordBatchStream,
};
use async_trait::async_trait;
pub use basic::DefaultAreaStore;
pub use cache::CachedAreaStore;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use futures::TryStreamExt;
use object_store::{path::Path, DynObjectStore, Error as ObjectStoreError};
use std::sync::Arc;
pub use writer::*;

const DATA_FOLDER_NAME: &str = "_ff_data";
const DELTA_LOG_FOLDER_NAME: &str = "_delta_log";
const DEFAULT_BATCH_SIZE: usize = 2048;

#[async_trait]
pub trait AreaStore: Send + Sync {
    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<DynObjectStore>;

    fn get_path_from_raw(&self, raw: String) -> Path;

    async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef>;

    /// Write batches into table location
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>>;

    /// Stream RecordBatches from a parquet file
    async fn open_file(
        &self,
        file: &AreaPath,
        column_indices: Option<Vec<usize>>,
    ) -> Result<SendableRecordBatchStream> {
        open_file(self.object_store(), file, column_indices).await
    }

    async fn get_location_files(&self, location: &AreaPath) -> Result<Vec<Path>> {
        Ok(self
            .object_store()
            .list(Some(&location.into()))
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|f| f.location)
            .collect::<Vec<_>>())
    }

    async fn delete_location(&self, _location: &Path) -> Result<()> {
        todo!()
    }

    async fn list_areas(&self, prefix: Option<&Path>) -> Result<Vec<AreaPath>> {
        let areas = self.object_store().list_with_delimiter(prefix).await?;
        let folders = areas.common_prefixes.into_iter().map(AreaPath::from);

        let mut data_roots = Vec::new();
        for folder in folders {
            if folder.is_table_root() {
                data_roots.push(folder);
            } else {
                let nested = self.list_areas(Some(&folder.into())).await?;
                data_roots.extend(nested);
            };
        }

        Ok(data_roots)
    }
}

async fn is_delta_location(store: Arc<DynObjectStore>, location: &AreaPath) -> Result<bool> {
    let path: Path = location.into();
    let path = path.child(DELTA_LOG_FOLDER_NAME);
    let res = match store.head(&path).await {
        Ok(_) => Ok(true),
        Err(ObjectStoreError::NotFound { .. }) => Ok(false),
        Err(other) => Err(other),
    }?;
    Ok(res)
}

async fn open_file(
    store: Arc<DynObjectStore>,
    file: &AreaPath,
    column_indices: Option<Vec<usize>>,
) -> Result<SendableRecordBatchStream> {
    let bytes = store.get(&file.into()).await?.bytes().await?;
    let cursor = SliceableCursor::new(Arc::new(bytes.to_vec()));
    let file_reader = Arc::new(SerializedFileReader::new(cursor)?);
    let mut arrow_reader = ParquetFileArrowReader::new(file_reader);
    let record_batch_reader = match column_indices {
        Some(indices) => arrow_reader.get_record_reader_by_columns(indices, DEFAULT_BATCH_SIZE),
        None => arrow_reader.get_record_reader(DEFAULT_BATCH_SIZE),
    }?;

    Ok(Box::pin(RecordBatchStreamAdapter::new(
        record_batch_reader.schema(),
        futures::stream::iter(record_batch_reader),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
    };

    #[tokio::test]
    async fn is_delta() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path();
        let area_store = Arc::new(DefaultAreaStore::try_new(area_root).unwrap());

        let path = Path::parse("_ff_data/foo/_delta_log/00000000000.json").unwrap();
        let data = Bytes::from("arbitrary data");
        area_store
            .object_store()
            .put(&path, data.clone())
            .await
            .unwrap();

        let table = TableReference::Location(AreaTableLocation {
            name: "foo".to_string(),
            areas: vec![],
        });
        let source = AreaSourceReference { table: Some(table) };

        let is_delta = is_delta_location(area_store.object_store().clone(), &source.into())
            .await
            .unwrap();
        assert!(is_delta);

        let path = Path::parse("_ff_data/bar/00000000000.parquet").unwrap();
        let data = Bytes::from("arbitrary data");
        area_store
            .object_store()
            .put(&path, data.clone())
            .await
            .unwrap();

        let table = TableReference::Location(AreaTableLocation {
            name: "bar".to_string(),
            areas: vec![],
        });
        let source = AreaSourceReference { table: Some(table) };

        let is_delta = is_delta_location(area_store.object_store().clone(), &source.into())
            .await
            .unwrap();
        assert!(!is_delta)
    }
}

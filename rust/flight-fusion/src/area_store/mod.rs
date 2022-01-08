//! Abstractions and implementations for writing data to delta tables
use std::path::PathBuf;
use std::sync::Arc;

use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::{
    arrow::ParquetFileArrowReader, basic::LogicalType,
    file::serialized_reader::SerializedFileReader,
};
use async_trait::async_trait;
pub use error::*;
use flight_fusion_ipc::{area_source_reference::Table as TableReference, AreaSourceReference};
use futures::{stream, StreamExt, TryStreamExt};
use object_store::{path::ObjectStorePath, ObjectStoreApi};
pub use utils::*;
pub use writer::*;

pub mod error;
mod stats;
pub mod utils;
pub mod writer;

const DATA_FOLDER_NAME: &str = "data";

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
pub trait AreaStore {
    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<object_store::ObjectStore>;

    // TODO use a more structured reference for table location
    async fn put_batches(&self, batches: Vec<RecordBatch>, path: &str) -> Result<Vec<stats::Add>>;
    async fn get_arrow_reader(&self, path: &str) -> ParquetFileArrowReader;
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
    async fn put_batches(&self, batches: Vec<RecordBatch>, path: &str) -> Result<Vec<stats::Add>> {
        let schema = batches[0].schema();
        let partition_cols = vec![];
        let mut writer =
            DeltaWriter::new(self.object_store().clone(), schema, Some(partition_cols));
        batches.iter().for_each(|b| writer.write(b).unwrap());
        let location = self.object_store().path_from_raw(&path);
        writer.flush(&location).await
    }

    async fn get_arrow_reader(&self, path: &str) -> ParquetFileArrowReader {
        let location = self.object_store().path_from_raw(path);
        let obj_reader = BytesReader(bytes::Bytes::from(
            self.object_store()
                .get(&location)
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap(),
        ));
        // TODO remove panic and return result
        let file_reader = Arc::new(SerializedFileReader::new(obj_reader).unwrap());
        ParquetFileArrowReader::new(file_reader)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow_deps::datafusion::parquet::arrow::ArrowReader;

    #[tokio::test]
    async fn test_arrow_reader() {
        let ws_root = crate::test_utils::workspace_root().unwrap();
        let root = format!("{}/test", ws_root);
        let area_store = InMemoryAreaStore::new(root);
        let file_path = "data/P1.parquet";
        let mut arrow_reader = area_store.get_arrow_reader(file_path).await;
        let schema = Arc::new(arrow_reader.get_schema().unwrap());
        assert!(schema.fields().len() > 1)
    }
}

//! Abstractions and implementations for writing data to delta tables
use std::path::PathBuf;
use std::sync::Arc;

use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    basic::LogicalType,
    file::serialized_reader::SerializedFileReader,
};
use async_trait::async_trait;
pub use error::*;
use object_store::{path::parsed::DirsAndFileName, ObjectStoreApi};
pub use utils::*;
pub use writer::*;

pub mod error;
pub mod json;
mod stats;
pub mod utils;
pub mod writer;

#[async_trait]
pub trait AreaStore {
    fn store_batches(&self);
    fn object_store(&self) -> &object_store::ObjectStore;
    async fn get_arrow_reader(&self, path: &str) -> ParquetFileArrowReader;
}

pub struct InMemoryAreaStore {
    object_store: object_store::ObjectStore,
}

impl InMemoryAreaStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let object_store = object_store::ObjectStore::new_file(root);
        Self { object_store }
    }
}

#[async_trait]
impl AreaStore for InMemoryAreaStore {
    fn object_store(&self) -> &object_store::ObjectStore {
        &self.object_store
    }

    fn store_batches(&self) {
        println!("hello")
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

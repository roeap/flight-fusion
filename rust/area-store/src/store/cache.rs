use super::{error::Result, stats, AreaStore};
use arrow_deps::arrow::record_batch::RecordBatch;
use arrow_deps::datafusion::parquet::arrow::ParquetFileArrowReader;
use async_trait::async_trait;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use std::sync::Arc;

pub struct CachedAreaStore {
    store: Arc<dyn AreaStore>,
}

#[async_trait]
impl AreaStore for CachedAreaStore {
    fn object_store(&self) -> Arc<object_store::ObjectStore> {
        self.store.object_store()
    }
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &object_store::path::Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>> {
        self.store.put_batches(batches, location, save_mode).await
    }

    async fn get_batches(&self, location: &object_store::path::Path) -> Result<Vec<RecordBatch>> {
        self.store.get_batches(location).await
    }

    fn get_table_location(&self, source: &AreaSourceReference) -> Result<object_store::path::Path> {
        self.store.get_table_location(source)
    }

    async fn get_arrow_reader(
        &self,
        location: &object_store::path::Path,
    ) -> Result<ParquetFileArrowReader> {
        self.store.get_arrow_reader(location).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn batch_to_ipc() {
        todo!()
    }
}

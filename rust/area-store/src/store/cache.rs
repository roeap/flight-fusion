use super::{error::Result, stats, AreaStore, DEFAULT_READ_BATCH_SIZE};
use arrow_deps::datafusion::parquet::arrow::ParquetFileArrowReader;
use arrow_deps::{
    arrow::{
        datatypes::SchemaRef as ArrowSchemaRef,
        ipc::{reader::StreamReader, writer::StreamWriter},
        record_batch::RecordBatch,
    },
    datafusion::parquet::arrow::{ArrowReader, ParquetRecordBatchStreamBuilder},
};
use async_trait::async_trait;
use file_cache::LruDiskCache;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use object_store::{
    path::{ObjectStorePath, Path},
    ObjectStoreApi,
};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

pub struct CachedAreaStore {
    store: Arc<dyn AreaStore>,
    cache: Arc<RwLock<LruDiskCache>>,
}

impl CachedAreaStore {
    pub fn try_new<P>(store: Arc<dyn AreaStore>, cache_root: P, max_cache_size: u64) -> Result<Self>
    where
        PathBuf: From<P>,
    {
        let cache = Arc::new(RwLock::new(LruDiskCache::new(cache_root, max_cache_size)?));
        Ok(Self { store, cache })
    }

    async fn file_in_cache(&self, location: &Path) -> bool {
        let cache = self.cache.read().unwrap();
        cache.contains_key(location.to_raw())
    }

    async fn load_or_cache(&self, location: &Path) -> Result<Vec<RecordBatch>> {
        // check if file can be loaded from cache
        if self.file_in_cache(location).await {
            // TODO remove panic
            let mut cache = self.cache.write().unwrap();
            let file_reader = StreamReader::try_new(cache.get(location.to_raw())?, None)?;
            let mut batches = Vec::new();
            for batch in file_reader.into_iter() {
                batches.push(batch?);
            }
            return Ok(batches);
        }

        // read record patches form location (file)
        let mut batches = Vec::new();
        let mut reader = self.get_arrow_reader(location).await?;
        let batch_reader = reader.get_record_reader(DEFAULT_READ_BATCH_SIZE)?;
        let mut file_batches = batch_reader
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?;
        batches.append(&mut file_batches);

        // Write record batches to cache in IPC format
        if !batches.is_empty() {
            let mut buffer = Vec::new();
            let mut stream_writer = StreamWriter::try_new(&mut buffer, &batches[0].schema())?;
            for batch in batches.iter() {
                stream_writer.write(batch)?
            }
            let batch_bytes = stream_writer.into_inner()?;
            let mut cache = self.cache.write().unwrap();
            cache.insert_bytes(location.to_raw(), batch_bytes)?;
        }

        Ok(batches)
    }
}

#[async_trait]
impl AreaStore for CachedAreaStore {
    fn object_store(&self) -> Arc<object_store::ObjectStore> {
        self.store.object_store()
    }

    fn get_path_from_raw(&self, raw: String) -> Path {
        self.store.get_path_from_raw(raw)
    }

    fn get_table_location(&self, source: &AreaSourceReference) -> Result<Path> {
        self.store.get_table_location(source)
    }

    async fn get_schema(&self, location: &Path) -> Result<ArrowSchemaRef> {
        if self.file_in_cache(location).await {
            // TODO remove panic
            // TODO use async ArrowReader as well
            let mut cache = self.cache.write().unwrap();
            let file_reader = StreamReader::try_new(cache.get(location.to_raw())?, None)?;
            Ok(file_reader.schema())
        } else {
            let reader = self.store.object_store().open_file(&location).await?;
            let builder = ParquetRecordBatchStreamBuilder::new(reader).await?;
            Ok(builder.schema().clone())
        }
    }

    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>> {
        self.store.put_batches(batches, location, save_mode).await
    }

    async fn get_batches(&self, location: &Path) -> Result<Vec<RecordBatch>> {
        let files = self.get_location_files(location).await?;
        let mut batches = Vec::new();
        for file in files {
            let mut file_batches = self.load_or_cache(&file).await?;
            batches.append(&mut file_batches);
        }
        Ok(batches)
    }

    async fn get_arrow_reader(&self, location: &Path) -> Result<ParquetFileArrowReader> {
        self.store.get_arrow_reader(location).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::basic::DefaultAreaStore;
    use crate::test_utils::get_record_batch;
    use object_store::ObjectStoreApi;

    #[tokio::test]
    async fn put_cache_on_read() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path().join(".tmp");
        let cache_root = root.path().join(".tmp/_ff_cache");

        let area_store = Arc::new(DefaultAreaStore::new(area_root));
        let cached_store = CachedAreaStore::try_new(area_store, cache_root, 10000).unwrap();

        let mut path = cached_store.object_store().new_path();
        path.push_dir("asd");

        let batch = get_record_batch(None, false);
        cached_store
            .put_batches(vec![batch.clone()], &path, SaveMode::Overwrite)
            .await
            .unwrap();

        // On first read, the file contents should not yet be cached
        let batches = cached_store.get_batches(&path).await.unwrap();
        assert_eq!(batches[0], batch);

        // After loading the batches from file, file contents should be cached.
        let batches = cached_store.get_batches(&path).await.unwrap();
        assert_eq!(batches[0], batch)
    }

    #[tokio::test]
    async fn read_schema() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path().join(".tmp");
        let cache_root = root.path().join(".tmp/_ff_cache");

        let area_store = Arc::new(DefaultAreaStore::new(area_root));
        let cached_store = CachedAreaStore::try_new(area_store, cache_root, 10000).unwrap();

        let mut path = cached_store.object_store().new_path();
        path.push_dir("asd");

        let batch = get_record_batch(None, false);
        cached_store
            .put_batches(vec![batch.clone()], &path, SaveMode::Overwrite)
            .await
            .unwrap();

        let files = cached_store.get_location_files(&path).await.unwrap();
        // In this check schema should be read from file
        let schema = cached_store.get_schema(&files[0]).await.unwrap();
        assert_eq!(schema, batch.schema());

        let batches = cached_store.get_batches(&path).await.unwrap();
        assert_eq!(batches[0], batch);

        // After loading the batches from file, file contents should be cached.
        let schema = cached_store.get_schema(&files[0]).await.unwrap();
        assert_eq!(schema, batch.schema())
    }
}

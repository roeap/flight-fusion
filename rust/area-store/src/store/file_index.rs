use crate::store::flatten_list_stream;
use arrow_deps::datafusion::arrow::datatypes::SchemaRef as ArrowSchemaRef;
use arrow_deps::datafusion::parquet::arrow::{
    async_reader::ParquetRecordBatchStreamBuilder, parquet_to_arrow_schema,
};
use arrow_deps::datafusion::parquet::file::metadata::ParquetMetaData;
use dashmap::DashMap;
use futures::future::try_join_all;
use object_store::ObjectStoreApi;
use object_store::{
    path::{parsed::DirsAndFileName, parts::PathPart, Path},
    ObjectStore,
};
use observability_deps::tracing::info;
use std::sync::Arc;
use tokio::task::{spawn, JoinHandle};

pub type IndexEntry = (Path, Arc<ParquetMetaData>);

pub trait ToCacheKey {
    fn to_key(&self) -> Vec<PathPart>;
}

impl ToCacheKey for DirsAndFileName {
    fn to_key(&self) -> Vec<PathPart> {
        self.directories.clone()
    }
}

impl ToCacheKey for Vec<PathPart> {
    fn to_key(&self) -> Vec<PathPart> {
        self.clone()
    }
}

impl ToCacheKey for Path {
    fn to_key(&self) -> Vec<PathPart> {
        let dirs = DirsAndFileName::from(self.clone());
        dirs.directories
    }
}

pub struct FileIndex {
    store: Arc<ObjectStore>,
    index: Arc<DashMap<Vec<PathPart>, Vec<IndexEntry>>>,
}

impl FileIndex {
    pub fn new(store: Arc<ObjectStore>) -> Self {
        Self {
            store,
            index: Arc::new(DashMap::new()),
        }
    }

    pub async fn build_index(&self) -> crate::error::Result<()> {
        info!("Building file index");
        let stats: Vec<JoinHandle<crate::error::Result<IndexEntry>>> =
            flatten_list_stream(&self.store, None)
                .await?
                .into_iter()
                .map(|f| {
                    let object_store = self.store.clone();
                    spawn(async move {
                        let file = object_store.open_file(&f).await?;
                        let builder = ParquetRecordBatchStreamBuilder::new(file).await?;
                        Ok((f, builder.metadata().clone()))
                    })
                })
                .collect();

        let outputs = try_join_all(stats).await.unwrap();

        for (file, schema) in outputs.into_iter().flatten() {
            let parsed = DirsAndFileName::from(file.clone());
            let mut area = self.index.entry(parsed.directories).or_insert(Vec::new());
            area.push((file, schema))
        }

        Ok(())
    }

    pub async fn scan_area(&self) {
        todo!()
    }

    pub async fn scan_files(&self, _files: Vec<Path>) {
        todo!()
    }

    pub fn get_area_files(&self, area_ref: &impl ToCacheKey) -> Option<Vec<IndexEntry>> {
        Some(self.index.get(&area_ref.to_key())?.value().to_vec())
    }

    pub fn get_schema(&self, area_ref: &impl ToCacheKey) -> Option<ArrowSchemaRef> {
        let reference = self.index.get(&area_ref.to_key()).unwrap();
        let (_, metadata) = reference.value().first()?;
        let schema = Arc::new(
            parquet_to_arrow_schema(
                metadata.file_metadata().schema_descr(),
                metadata.file_metadata().key_value_metadata(),
            )
            .unwrap(),
        );
        Some(schema)
    }

    pub fn get_area_counts(&self, area_ref: &impl ToCacheKey) -> Option<(usize, usize)> {
        Some(self.index.get(&area_ref.to_key())?.value().iter().fold(
            (0, 0),
            |(n_rows, size), f| {
                let (n, s) =
                    f.1.row_groups()
                        .iter()
                        .fold((0, 0), |(i_n_rows, i_size), meta| {
                            (i_n_rows + meta.num_rows(), i_size + meta.total_byte_size())
                        });
                (n_rows + n as usize, size + s as usize)
            },
        ))
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn contains_key(&self, key: &impl ToCacheKey) -> bool {
        self.index.contains_key(&key.to_key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::AreaStore;
    use crate::store::DefaultAreaStore;
    use flight_fusion_ipc::SaveMode;
    use object_store::path::ObjectStorePath;

    #[tokio::test]
    async fn create_index() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path().join(".tmp");
        let area_store = Arc::new(DefaultAreaStore::try_new(area_root).unwrap());

        let mut location = area_store.object_store().new_path();
        location.push_dir("_ff_data");
        location.push_dir("asd");

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(vec![batch.clone()], &location, SaveMode::Append)
            .await
            .unwrap();

        assert!(!area_store.file_index.contains_key(&location));

        area_store.build_index().await.unwrap();
        assert_eq!(area_store.file_index.len(), 1);
        assert!(area_store.file_index.contains_key(&location));
        assert_eq!(
            batch.schema(),
            area_store.file_index.get_schema(&location).unwrap()
        );

        let (n_rows, _) = area_store.file_index.get_area_counts(&location).unwrap();
        assert_eq!(n_rows, batch.num_rows())
    }
}

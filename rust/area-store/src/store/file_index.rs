use arrow_deps::datafusion::arrow::datatypes::SchemaRef as ArrowSchemaRef;
use arrow_deps::datafusion::parquet::arrow::{
    async_reader::ParquetRecordBatchStreamBuilder, parquet_to_arrow_schema,
};
use arrow_deps::datafusion::parquet::file::metadata::ParquetMetaData;
use dashmap::DashMap;
use futures::future::try_join_all;
use object_store::{
    path::{Path, PathPart},
    DynObjectStore, ObjectStore,
};
use observability_deps::tracing::info;
use std::sync::Arc;
use tokio::task::{spawn, JoinHandle};

pub type IndexEntry = (Path, Arc<ParquetMetaData>);

pub trait ToCacheKey {
    fn to_key(&self) -> Vec<PathPart>;
}

impl ToCacheKey for Vec<PathPart<'_>> {
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
    store: Arc<DynObjectStore>,
    index: Arc<DashMap<String, Vec<IndexEntry>>>,
}

impl FileIndex {
    pub fn new(store: Arc<dyn ObjectStore>) -> Self {
        Self {
            store,
            index: Arc::new(DashMap::new()),
        }
    }

    pub async fn build_index(&self) -> crate::error::Result<()> {
        info!("Building file index");
        todo!()
        // let stats: Vec<JoinHandle<crate::error::Result<IndexEntry>>> =
        //     flatten_list_stream(&self.store, None)
        //         .await?
        //         .into_iter()
        //         .map(|f| {
        //             let object_store = self.store.clone();
        //             spawn(async move {
        //                 let file = object_store.open_file(&f).await?;
        //                 let builder = ParquetRecordBatchStreamBuilder::new(file).await?;
        //                 Ok((f, builder.metadata().clone()))
        //             })
        //         })
        //         .collect();
        //
        // let outputs = try_join_all(stats).await.unwrap();
        //
        // for (file, schema) in outputs.into_iter().flatten() {
        //     let parsed = DirsAndFileName::from(file.clone());
        //     let mut area = self.index.entry(parsed.directories).or_insert(Vec::new());
        //     area.push((file, schema))
        // }
        // Ok(())
    }

    pub async fn scan_area(&self) {
        todo!()
    }

    pub async fn scan_files(&self, _files: Vec<Path>) {
        todo!()
    }

    pub fn get_area_files(&self, area_ref: Path) -> Option<Vec<IndexEntry>> {
        todo!()
        // Some(self.index.get(&area_ref.to_key())?.value().to_vec())
    }

    pub fn get_schema(&self, area_ref: Path) -> Option<ArrowSchemaRef> {
        let reference = self.index.get(area_ref.as_ref()).unwrap();
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

    pub fn get_area_counts(&self, area_ref: Path) -> Option<(usize, usize)> {
        todo!()
        // Some(self.index.get(&area_ref.to_key())?.value().iter().fold(
        //     (0, 0),
        //     |(n_rows, size), f| {
        //         let (n, s) =
        //             f.1.row_groups()
        //                 .iter()
        //                 .fold((0, 0), |(i_n_rows, i_size), meta| {
        //                     (i_n_rows + meta.num_rows(), i_size + meta.total_byte_size())
        //                 });
        //         (n_rows + n as usize, size + s as usize)
        //     },
        // ))
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    pub fn contains_key(&self, key: Path) -> bool {
        self.index.contains_key(key.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::AreaStore;
    use flight_fusion_ipc::SaveMode;

    #[tokio::test]
    async fn create_index() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path();
        let area_store = Arc::new(AreaStore::try_new(area_root).unwrap());
    }
}

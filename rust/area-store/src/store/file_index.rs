use crate::store::{flatten_list_stream, AreaStore};
use arrow_deps::datafusion::parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder;
use arrow_deps::datafusion::{arrow::datatypes::Schema as ArrowSchema, logical_plan::plan::Join};
use dashmap::DashMap;
use futures::future::try_join_all;
use object_store::path::ObjectStorePath;
use object_store::ObjectStoreApi;
use object_store::{
    path::{parsed::DirsAndFileName, parts::PathPart, Path},
    ObjectStore,
};
use std::sync::Arc;
use tokio::task::{spawn, JoinHandle};

pub struct FileIndex {
    store: Arc<ObjectStore>,
    index: Arc<DashMap<Vec<PathPart>, Vec<(Path, Arc<ArrowSchema>)>>>,
}

impl FileIndex {
    pub fn new(store: Arc<ObjectStore>) -> Self {
        Self {
            store,
            index: Arc::new(DashMap::new()),
        }
    }

    pub async fn build_index(&self) -> crate::error::Result<()> {
        let stats: Vec<JoinHandle<crate::error::Result<(Path, Arc<ArrowSchema>)>>> =
            flatten_list_stream(&self.store, None)
                .await?
                .into_iter()
                .map(|f| {
                    let object_store = self.store.clone();
                    spawn(async move {
                        let file = object_store.open_file(&f).await?;
                        let builder = ParquetRecordBatchStreamBuilder::new(file).await?;
                        Ok((f, builder.schema().clone()))
                    })
                })
                .collect();

        let outputs = try_join_all(stats).await.unwrap();

        for result in outputs {
            if let Ok((file, schema)) = result {
                let parsed = DirsAndFileName::from(file.clone());
                let mut area = self.index.entry(parsed.directories).or_insert(Vec::new());
                area.push((file, schema))
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::DefaultAreaStore;

    #[tokio::test]
    async fn create_index() {
        let area_store = Arc::new(
            DefaultAreaStore::try_new("/home/robstar/github/flight-fusion/.flight-fusion/.fusion")
                .unwrap(),
        );
        // let file_index = FileIndex::try_new(area_store).await.unwrap();
    }
}

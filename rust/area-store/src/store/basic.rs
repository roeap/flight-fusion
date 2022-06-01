use super::{stats, writer::*, AreaPath, AreaStore};
use crate::error::{AreaStoreError, Result};
use arrow_deps::arrow::record_batch::*;
use arrow_deps::datafusion::{
    arrow::datatypes::SchemaRef as ArrowSchemaRef, physical_plan::common::collect,
};
use async_trait::async_trait;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use futures::TryStreamExt;
use object_store::path::Path;
use object_store::DynObjectStore;
use std::path::PathBuf;
use std::sync::Arc;

pub struct DefaultAreaStore {
    object_store: Arc<DynObjectStore>,
    root_path: String,
    // only visible for testing purposes
    // pub(crate) file_index: Arc<FileIndex>,
}

impl DefaultAreaStore {
    pub fn try_new(root: impl Into<PathBuf>) -> Result<Self> {
        let buf: PathBuf = root.into();
        let object_store =
            Arc::new(object_store::local::LocalFileSystem::new_with_prefix(buf.clone()).unwrap());
        // let file_index = Arc::new(FileIndex::new(object_store.clone()));

        Ok(Self {
            object_store,
            root_path: buf.to_str().unwrap().to_string(),
            // file_index,
        })
    }

    pub fn try_new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Result<Self> {
        let container: String = container_name.into();
        let object_store = Arc::new(object_store::azure::new_azure(
            account,
            access_key,
            container.clone(),
            false,
        )?);
        // let file_index = Arc::new(FileIndex::new(object_store.clone()));

        Ok(Self {
            object_store,
            root_path: format!("adls2://{}", container),
            // file_index,
        })
    }

    pub fn get_full_table_path(&self, source: &AreaSourceReference) -> Result<String> {
        let location: AreaPath = source.into();
        Ok(format!("{}/{}", self.root_path, location.as_ref()))
    }

    pub async fn build_index(&self) -> Result<()> {
        // self.file_index.build_index().await
        Ok(())
    }
}

#[async_trait]
impl AreaStore for DefaultAreaStore {
    fn object_store(&self) -> Arc<DynObjectStore> {
        self.object_store.clone()
    }

    fn get_path_from_raw(&self, raw: String) -> Path {
        let trimmed_raw = raw
            .trim_start_matches(&self.root_path)
            .trim_start_matches('/');
        // TODO remove panic
        Path::parse(trimmed_raw).unwrap()
    }

    async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef> {
        // TODO only actually load first file and also make this work for delta
        let area_path = AreaPath::from(source);
        let files = self.get_location_files(&area_path.into()).await?;
        let reader = self.open_file(&files[0], None).await?;
        Ok(reader.schema())
    }

    // TODO use SendableRecordBatchStream as input
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>> {
        let schema = batches[0].schema();
        let partition_cols = vec![];
        let mut writer = DeltaWriter::new(self.object_store(), schema, Some(partition_cols));
        for batch in batches.iter() {
            writer.write(batch)?
        }
        match save_mode {
            SaveMode::Overwrite => {
                let files = &self
                    .object_store()
                    .list(Some(location))
                    .await?
                    .try_collect::<Vec<_>>()
                    .await?;
                for file in files {
                    self.object_store().delete(&file.location).await?;
                }
                writer.flush(location).await
            }
            // TODO actually check if exists
            SaveMode::ErrorIfExists => Err(AreaStoreError::TableAlreadyExists(
                location.as_ref().to_string(),
            )),
            _ => writer.flush(location).await,
        }
    }

    /// Read batches from location
    async fn get_batches(&self, location: &AreaPath) -> Result<Vec<RecordBatch>> {
        let files = self.get_location_files(location).await?;
        let mut batches = Vec::new();
        for file in files {
            let mut batch = collect(self.open_file(&file, None).await?).await?;
            batches.append(&mut batch);
        }
        Ok(batches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::get_record_batch;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        SaveMode,
    };

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = DefaultAreaStore::try_new(root.path()).unwrap();
        let location = AreaPath::default();

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(
                vec![batch.clone()],
                &location.clone().into(),
                SaveMode::Append,
            )
            .await
            .unwrap();

        let read_batch = area_store.get_batches(&location).await.unwrap();
        assert_eq!(batch, read_batch[0])
    }

    #[tokio::test]
    async fn read_schema() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path();
        let area_store = Arc::new(DefaultAreaStore::try_new(area_root).unwrap());

        let path = Path::parse("_ff_data/asd").unwrap();

        let batch = get_record_batch(None, false);
        area_store
            .put_batches(vec![batch.clone()], &path, SaveMode::Overwrite)
            .await
            .unwrap();

        let table = TableReference::Location(AreaTableLocation {
            name: "asd".to_string(),
            areas: vec![],
        });
        let source = AreaSourceReference { table: Some(table) };

        let schema = area_store.get_schema(&source).await.unwrap();
        assert_eq!(schema, batch.schema());
    }
}

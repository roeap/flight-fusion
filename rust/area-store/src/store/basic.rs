use super::{stats, utils::*, writer::*, AreaStore, DATA_FOLDER_NAME};
use crate::error::{AreaStoreError, Result};
use crate::store::file_index::FileIndex;
use arrow_deps::arrow::record_batch::*;
use arrow_deps::datafusion::parquet::arrow::async_reader::ParquetRecordBatchStreamBuilder;
use arrow_deps::datafusion::{
    arrow::datatypes::SchemaRef as ArrowSchemaRef,
    parquet::{
        arrow::ParquetFileArrowReader,
        file::serialized_reader::{SerializedFileReader, SliceableCursor},
    },
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    area_source_reference::Table as TableReference, AreaSourceReference, SaveMode,
};
use object_store::path::Path;
use object_store::{path::ObjectStorePath, ObjectStoreApi};
use std::path::PathBuf;
use std::sync::Arc;

const DEFAULT_PARQUET_SUFFIX: &str = ".parquet";

pub struct DefaultAreaStore {
    object_store: Arc<object_store::ObjectStore>,
    root_path: String,
    // only visible for testing purposes
    pub(crate) file_index: Arc<FileIndex>,
}

impl DefaultAreaStore {
    pub fn try_new(root: impl Into<PathBuf>) -> Result<Self> {
        let buf: PathBuf = root.into();
        let object_store = Arc::new(object_store::ObjectStore::new_file(buf.clone()));
        let file_index = Arc::new(FileIndex::new(object_store.clone()));

        Ok(Self {
            object_store,
            root_path: buf.to_str().unwrap().to_string(),
            file_index,
        })
    }

    pub fn try_new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Result<Self> {
        let container: String = container_name.into();
        let object_store = Arc::new(object_store::ObjectStore::new_microsoft_azure(
            account,
            access_key,
            container.clone(),
            false,
        )?);
        let file_index = Arc::new(FileIndex::new(object_store.clone()));

        Ok(Self {
            object_store,
            root_path: format!("adls2://{}", container),
            file_index,
        })
    }

    pub fn get_full_table_path(&self, source: &AreaSourceReference) -> Result<String> {
        let location = self.get_table_location(source)?;
        Ok(format!("{}/{}", self.root_path, location.to_raw()))
    }

    pub async fn build_index(&self) -> Result<()> {
        self.file_index.build_index().await
    }
}

#[async_trait]
impl AreaStore for DefaultAreaStore {
    fn object_store(&self) -> Arc<object_store::ObjectStore> {
        self.object_store.clone()
    }

    fn get_path_from_raw(&self, raw: String) -> Path {
        let trimmed_raw = raw
            .trim_start_matches(&self.root_path)
            .trim_start_matches('/');
        if let Some((first, last)) = trimmed_raw.rsplit_once('/') {
            if last.ends_with(DEFAULT_PARQUET_SUFFIX) {
                let mut path = self.object_store.path_from_raw(first);
                path.set_file_name(last);
                return path;
            }
        }
        self.object_store.path_from_raw(trimmed_raw)
    }

    fn get_table_location(&self, source: &AreaSourceReference) -> Result<object_store::path::Path> {
        match source {
            AreaSourceReference { table: Some(tbl) } => match tbl {
                TableReference::Location(loc) => {
                    let mut location = self.object_store().new_path();
                    loc.areas.iter().for_each(|p| location.push_dir(p));
                    location.push_dir(DATA_FOLDER_NAME);
                    location.push_dir(&loc.name);
                    Ok(location)
                }
                TableReference::Uri(_uri) => {
                    todo!()
                }
                TableReference::Id(_id) => {
                    todo!()
                }
            },
            _ => todo!(),
        }
    }

    async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef> {
        // TODO only actually load first file and also make this work for delta
        let files = self.get_source_files(source).await?;
        let reader = self.object_store.open_file(&files[0]).await?;
        let builder = ParquetRecordBatchStreamBuilder::new(reader).await?;
        Ok(builder.schema().clone())
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
        for batch in batches.iter() {
            writer.write(batch)?
        }
        match save_mode {
            SaveMode::Overwrite => {
                let files = flatten_list_stream(&self.object_store(), Some(location)).await?;
                for file in files {
                    self.object_store().delete(&file).await?;
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
        let files = self.get_location_files(location).await?;
        let mut batches = Vec::new();
        for file in files {
            batches.append(&mut self.read_file(&file).await?);
        }
        Ok(batches)
    }

    async fn get_arrow_reader(
        &self,
        location: &object_store::path::Path,
    ) -> Result<ParquetFileArrowReader> {
        let bytes = self.object_store().get(location).await?.bytes().await?;
        let cursor = SliceableCursor::new(Arc::new(bytes));
        let file_reader = Arc::new(SerializedFileReader::new(cursor)?);
        Ok(ParquetFileArrowReader::new(file_reader))
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
    use object_store::path::ObjectStorePath;

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = DefaultAreaStore::try_new(root.path()).unwrap();
        let location = area_store.object_store().new_path();

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(vec![batch.clone()], &location, SaveMode::Append)
            .await
            .unwrap();

        let read_batch = area_store.get_batches(&location).await.unwrap();
        assert_eq!(batch, read_batch[0])
    }

    #[tokio::test]
    async fn read_schema() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path().join(".tmp");
        let area_store = Arc::new(DefaultAreaStore::try_new(area_root).unwrap());

        let mut path = area_store.object_store().new_path();
        path.push_dir("_ff_data");
        path.push_dir("asd");

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

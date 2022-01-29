use std::path::PathBuf;
use std::sync::Arc;

use super::{error::*, stats, utils::*, writer::*, AreaStore};
use arrow_deps::arrow::record_batch::*;
use arrow_deps::datafusion::parquet::{
    arrow::{ArrowReader, ParquetFileArrowReader},
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use async_trait::async_trait;
use flight_fusion_ipc::{
    area_source_reference::Table as TableReference, AreaSourceReference, SaveMode,
};
use object_store::{path::ObjectStorePath, ObjectStoreApi};

const DATA_FOLDER_NAME: &str = "_ff_data";
const DEFAULT_READ_BATCH_SIZE: usize = 1024;

pub struct DefaultAreaStore {
    object_store: Arc<object_store::ObjectStore>,
}

impl DefaultAreaStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let object_store = Arc::new(object_store::ObjectStore::new_file(root));
        Self { object_store }
    }

    pub fn new_azure(
        account: impl Into<String>,
        access_key: impl Into<String>,
        container_name: impl Into<String>,
    ) -> Result<Self> {
        let object_store = Arc::new(object_store::ObjectStore::new_microsoft_azure(
            account,
            access_key,
            container_name,
            false,
        )?);
        Ok(Self { object_store })
    }
}

#[async_trait]
impl AreaStore for DefaultAreaStore {
    fn object_store(&self) -> Arc<object_store::ObjectStore> {
        self.object_store.clone()
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
                _ => todo!(),
            },
            _ => todo!(),
        }
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
        // TODO Don't panic
        batches.iter().for_each(|b| writer.write(b).unwrap());

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
        let files = flatten_list_stream(&self.object_store(), Some(location))
            .await
            .unwrap();
        let mut batches = Vec::new();
        for file in files {
            let mut reader = self.get_arrow_reader(&file).await?;
            let batch_reader = reader.get_record_reader(DEFAULT_READ_BATCH_SIZE).unwrap();
            let mut file_batches = batch_reader
                .into_iter()
                .collect::<std::result::Result<Vec<_>, _>>()?;
            batches.append(&mut file_batches);
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

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = DefaultAreaStore::new(root.path());
        let location = area_store.object_store().new_path();

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(vec![batch.clone()], &location, SaveMode::Append)
            .await
            .unwrap();

        let read_batch = area_store.get_batches(&location).await.unwrap();

        assert_eq!(batch, read_batch[0])
    }
}

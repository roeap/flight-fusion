use super::{is_delta_location, stats, writer::*, AreaPath, AreaStore};
use crate::error::{AreaStoreError, Result};
use arrow_deps::arrow::record_batch::*;
use arrow_deps::datafusion::arrow::datatypes::SchemaRef as ArrowSchemaRef;
use arrow_deps::deltalake::open_table;
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

    pub fn get_full_table_path(&self, source: &AreaPath) -> Result<String> {
        Ok(format!("{}/{}", self.root_path, source.as_ref()))
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
        let area_path = AreaPath::from(source);
        let is_delta = is_delta_location(self.object_store().clone(), &area_path).await?;

        if is_delta {
            let full_path = self.get_full_table_path(&area_path)?;
            let table = open_table(&full_path).await?;
            return Ok(Arc::new(table.get_schema()?.try_into()?));
        }

        let files = self.get_location_files(&area_path).await?;
        if files.is_empty() {
            return Err(AreaStoreError::TableDoesNotExists(
                "does not exist".to_string(),
            ));
        }
        let reader = self.open_file(&files[0].clone().into(), None).await?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_record_batch, workspace_test_data_folder};
    use arrow_deps::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
    use arrow_deps::datafusion::physical_plan::common::collect;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        SaveMode,
    };

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = DefaultAreaStore::try_new(root.path()).unwrap();
        let location: AreaPath = Path::from("_ff_data/test").into();

        let batch = crate::test_utils::get_record_batch(None, false);
        area_store
            .put_batches(
                vec![batch.clone()],
                &location.clone().into(),
                SaveMode::Append,
            )
            .await
            .unwrap();

        let files = area_store.get_location_files(&location).await.unwrap();

        let read_batch = collect(
            area_store
                .open_file(&files[0].clone().into(), None)
                .await
                .unwrap(),
        )
        .await
        .unwrap();

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

    #[tokio::test]
    async fn read_delta_schema() {
        let area_root = workspace_test_data_folder();
        let area_store = Arc::new(DefaultAreaStore::try_new(area_root).unwrap());

        let ref_schema = Arc::new(Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                true,
            ),
            Field::new("date", DataType::Date32, true),
            Field::new("string", DataType::Utf8, true),
            Field::new("double", DataType::Float64, true),
            Field::new("real", DataType::Float64, true),
            Field::new("float", DataType::Float64, true),
        ]));

        let source = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "date".to_string(),
                areas: vec!["delta".to_string(), "partitioned".to_string()],
            })),
        };
        let schema = area_store.get_schema(&source).await.unwrap();
        assert_eq!(ref_schema, schema);

        let source = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "string".to_string(),
                areas: vec!["delta".to_string(), "partitioned".to_string()],
            })),
        };
        let schema = area_store.get_schema(&source).await.unwrap();
        assert_eq!(ref_schema, schema);

        let source = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "simple".to_string(),
                areas: vec!["delta".to_string()],
            })),
        };
        let schema = area_store.get_schema(&source).await.unwrap();
        assert_eq!(ref_schema, schema);
    }
}

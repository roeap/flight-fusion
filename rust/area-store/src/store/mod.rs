//! Abstractions and implementations for writing data to delta tables
mod area_path;
// mod stats;
pub mod writer;

use crate::error::{AreaStoreError, Result};
use crate::storage_location::StorageLocation;
pub use area_path::*;
use arrow_deps::arrow::{datatypes::SchemaRef as ArrowSchemaRef, record_batch::RecordBatch};
use arrow_deps::datafusion::arrow::record_batch::RecordBatchReader;
use arrow_deps::datafusion::datasource::{object_store::ObjectStoreRegistry, TableProvider};
use arrow_deps::datafusion::parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use arrow_deps::datafusion::parquet::{
    arrow::{arrow_to_parquet_schema, ProjectionMask},
    file::serialized_reader::SerializedFileReader,
};
use arrow_deps::datafusion::physical_plan::{
    stream::RecordBatchStreamAdapter, SendableRecordBatchStream,
};
use arrow_deps::deltalake::{
    get_backend_for_uri, get_backend_for_uri_with_options, DeltaTable, DeltaTableBuilder,
    DeltaTableConfig, DeltaTableError,
};
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use futures::TryStreamExt;
use object_store::{path::Path, DynObjectStore, Error as ObjectStoreError};
use std::collections::HashMap;
use std::sync::Arc;
pub use writer::*;

const DATA_FOLDER_NAME: &str = "_ff_data";
const DELTA_LOG_FOLDER_NAME: &str = "_delta_log";
const DEFAULT_BATCH_SIZE: usize = 2048;

#[derive(Debug, Clone)]
pub struct AreaStore {
    object_store: Arc<DynObjectStore>,
    object_store_registry: Arc<ObjectStoreRegistry>,
    root_path: String,
    pub root: StorageLocation,
    pub(crate) storage_options: Option<HashMap<String, String>>,
}

impl AreaStore {
    pub fn try_new(root: impl Into<std::path::PathBuf>) -> Result<Self> {
        let buf: std::path::PathBuf = root.into();
        std::fs::create_dir_all(&buf).map_err(|err| AreaStoreError::Generic {
            source: Box::new(err),
        })?;
        let object_store =
            Arc::new(object_store::local::LocalFileSystem::new_with_prefix(buf.clone()).unwrap());
        let buf = buf.canonicalize().unwrap();
        let root = StorageLocation::try_from(buf.to_str().unwrap())?;

        Ok(Self {
            object_store,
            object_store_registry: Arc::new(ObjectStoreRegistry::new()),
            root_path: buf.to_str().unwrap().to_string(),
            root,
            storage_options: None,
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
        let root = StorageLocation::try_from(format!("azure://{}", &container))?;

        Ok(Self {
            object_store,
            object_store_registry: Arc::new(ObjectStoreRegistry::new()),
            root_path: format!("adls2://{}", container),
            root,
            storage_options: None,
        })
    }

    pub fn object_store(&self) -> Arc<DynObjectStore> {
        self.object_store.clone()
    }

    pub fn get_path_from_raw(&self, raw: String) -> Path {
        let trimmed_raw = raw
            .trim_start_matches(&self.root_path)
            .trim_start_matches('/');
        // TODO remove panic
        Path::parse(trimmed_raw).unwrap()
    }

    pub async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef> {
        let area_path = AreaPath::from(source);
        let is_delta = is_delta_location(self.object_store().clone(), &area_path).await?;

        if is_delta {
            let mut table = self.open_delta(&area_path).await?;
            table.load().await?;
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
    pub async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<String>> {
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

    /// Stream RecordBatches from a parquet file
    pub async fn open_file(
        &self,
        file: &AreaPath,
        column_indices: Option<Vec<usize>>,
    ) -> Result<SendableRecordBatchStream> {
        open_file(self.object_store(), file, column_indices).await
    }

    pub async fn get_location_files(&self, location: &AreaPath) -> Result<Vec<Path>> {
        let is_delta = is_delta_location(self.object_store().clone(), location).await?;

        if is_delta {
            let table = self.open_delta(location).await?;
            return Ok(table.get_file_uris().map(Path::from).collect());
        }

        Ok(self
            .object_store()
            .list(Some(&location.into()))
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|f| f.location)
            .collect())
    }

    pub async fn delete_location(&self, _location: &AreaPath) -> Result<()> {
        todo!()
    }

    pub async fn list_areas(&self, prefix: Option<&Path>) -> Result<Vec<AreaPath>> {
        let areas = self
            .clone()
            .object_store()
            .list(prefix)
            .await?
            .map_err(|err| AreaStoreError::from(err))
            .map_ok(|meta| AreaPath::from(meta.location))
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .filter(|area| area.is_table_root())
            .collect();

        println!("{:?}", areas);

        Ok(areas)
    }

    pub async fn table_provider(&self, location: &AreaPath) -> Result<Arc<dyn TableProvider>> {
        Ok(Arc::new(self.open_delta(location).await?))
    }

    pub async fn open_delta_uninitialized(&self, location: &AreaPath) -> Result<DeltaTable> {
        let full_path = format!("{}/{}", self.root_path, location.as_ref());
        let storage = if let Some(options) = &self.storage_options {
            get_backend_for_uri_with_options(&full_path, options.clone())?
        } else {
            get_backend_for_uri(&full_path)?
        };
        Ok(DeltaTable::new(
            &full_path,
            storage,
            DeltaTableConfig::default(),
        )?)
    }

    pub async fn open_delta(&self, location: &AreaPath) -> Result<DeltaTable> {
        let full_path = format!("{}/{}", self.root_path, location.as_ref());
        if let StorageLocation::Local(store_loc) = StorageLocation::try_from(full_path.as_ref())? {
            std::fs::create_dir_all(store_loc.as_path()).map_err(|err| {
                AreaStoreError::Generic {
                    source: Box::new(err),
                }
            })?;
        }
        let mut builder = DeltaTableBuilder::from_uri(&full_path)?;
        if let Some(options) = &self.storage_options {
            let storage = get_backend_for_uri_with_options(&full_path, options.clone())?;
            builder = builder.with_storage_backend(storage);
        };
        match builder.load().await {
            Ok(table) => Ok(table),
            Err(DeltaTableError::NotATable(_)) => {
                let storage = if let Some(options) = &self.storage_options {
                    get_backend_for_uri_with_options(&full_path, options.clone())?
                } else {
                    get_backend_for_uri(&full_path)?
                };
                Ok(DeltaTable::new(
                    full_path,
                    storage,
                    DeltaTableConfig::default(),
                )?)
            }
            Err(err) => Err(AreaStoreError::Generic {
                source: Box::new(err),
            }),
        }
    }
}

pub async fn is_delta_location(store: Arc<DynObjectStore>, location: &AreaPath) -> Result<bool> {
    let path: Path = location.into();
    let path = path.child(DELTA_LOG_FOLDER_NAME);
    let res = match store.head(&path).await {
        Ok(_) => Ok(true),
        Err(ObjectStoreError::NotFound { .. }) => Ok(false),
        Err(other) => Err(other),
    }?;
    Ok(res)
}

pub async fn open_file(
    store: Arc<DynObjectStore>,
    file: &AreaPath,
    column_indices: Option<Vec<usize>>,
) -> Result<SendableRecordBatchStream> {
    let bytes = store.get(&file.into()).await?.bytes().await?;
    let file_reader = Arc::new(SerializedFileReader::new(bytes)?);
    let mut arrow_reader = ParquetFileArrowReader::new(file_reader);
    let record_batch_reader = match column_indices {
        Some(indices) => {
            let mask = ProjectionMask::roots(
                &arrow_to_parquet_schema(&arrow_reader.get_schema()?)?,
                indices,
            );
            arrow_reader.get_record_reader_by_columns(mask, DEFAULT_BATCH_SIZE)
        }
        None => arrow_reader.get_record_reader(DEFAULT_BATCH_SIZE),
    }?;

    Ok(Box::pin(RecordBatchStreamAdapter::new(
        record_batch_reader.schema(),
        futures::stream::iter(record_batch_reader),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{get_record_batch, workspace_test_data_folder};
    use arrow_deps::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
    use arrow_deps::datafusion::physical_plan::common::collect;
    use bytes::Bytes;
    use flight_fusion_ipc::{
        area_source_reference::Table as TableReference, AreaSourceReference, AreaTableLocation,
        SaveMode,
    };

    #[tokio::test]
    async fn is_delta() {
        let root = tempfile::tempdir().unwrap();
        let area_root = root.path();
        let area_store = Arc::new(AreaStore::try_new(area_root).unwrap());

        let path = Path::parse("_ff_data/foo/_delta_log/00000000000.json").unwrap();
        let data = Bytes::from("arbitrary data");
        area_store
            .object_store()
            .put(&path, data.clone())
            .await
            .unwrap();

        let table = TableReference::Location(AreaTableLocation {
            name: "foo".to_string(),
            areas: vec![],
        });
        let source = AreaSourceReference { table: Some(table) };

        let is_delta = is_delta_location(area_store.object_store().clone(), &source.into())
            .await
            .unwrap();
        assert!(is_delta);

        let path = Path::parse("_ff_data/bar/00000000000.parquet").unwrap();
        let data = Bytes::from("arbitrary data");
        area_store
            .object_store()
            .put(&path, data.clone())
            .await
            .unwrap();

        let table = TableReference::Location(AreaTableLocation {
            name: "bar".to_string(),
            areas: vec![],
        });
        let source = AreaSourceReference { table: Some(table) };

        let is_delta = is_delta_location(area_store.object_store().clone(), &source.into())
            .await
            .unwrap();
        assert!(!is_delta)
    }

    #[tokio::test]
    async fn test_put_get_batches() {
        let root = tempfile::tempdir().unwrap();
        let area_store = AreaStore::try_new(root.path()).unwrap();
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
        let area_store = Arc::new(AreaStore::try_new(area_root).unwrap());

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
        let area_store = Arc::new(AreaStore::try_new(area_root).unwrap());

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

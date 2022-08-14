//! Abstractions and implementations for writing data to delta tables
mod area_path;
use crate::error::{AreaStoreError, Result};
use crate::storage_url::{StorageService, StorageUrl};
pub use area_path::*;
use arrow_deps::arrow::{datatypes::SchemaRef as ArrowSchemaRef, record_batch::RecordBatchReader};
use arrow_deps::datafusion::{
    datasource::{
        file_format::parquet::ParquetFormat,
        listing::{ListingOptions, ListingTable, ListingTableConfig, ListingTableUrl},
        TableProvider,
    },
    parquet::{
        arrow::{arrow_to_parquet_schema, ArrowReader, ParquetFileArrowReader, ProjectionMask},
        file::serialized_reader::SerializedFileReader,
    },
    physical_plan::{stream::RecordBatchStreamAdapter, SendableRecordBatchStream},
    prelude::SessionContext,
};
use arrow_deps::deltalake::{
    get_backend_for_uri, get_backend_for_uri_with_options,
    writer::{DeltaWriter, RecordBatchWriter},
    DeltaTable, DeltaTableBuilder, DeltaTableConfig, DeltaTableError,
};
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use futures::StreamExt;
use futures::TryStreamExt;
use object_store::{path::Path, DynObjectStore, Error as ObjectStoreError};
use std::collections::HashMap;
use std::sync::Arc;

const DATA_FOLDER_NAME: &str = "_ff_data";
const DELTA_LOG_FOLDER_NAME: &str = "_delta_log";
const DEFAULT_BATCH_SIZE: usize = 2048;

#[derive(Debug, Clone)]
pub struct AreaStore {
    object_store: Arc<DynObjectStore>,
    root_path: String,
    pub root: StorageUrl,
    pub(crate) storage_options: Option<HashMap<String, String>>,
}

impl AreaStore {
    pub fn try_new(root: impl Into<std::path::PathBuf>) -> Result<Self> {
        let buf: std::path::PathBuf = root.into();
        std::fs::create_dir_all(&buf).map_err(|err| AreaStoreError::Generic {
            source: Box::new(err),
        })?;

        let root = StorageUrl::parse(
            buf.to_str()
                .ok_or_else(|| AreaStoreError::Parsing("failed to convert path".to_string()))?,
        )?;

        let object_store =
            Arc::new(object_store::local::LocalFileSystem::new_with_prefix(buf.clone()).unwrap());

        Ok(Self {
            object_store,
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

        let root = StorageUrl::parse(format!("azure://{}", &container))?;
        let object_store = Arc::new(object_store::azure::new_azure(
            account,
            access_key,
            container.clone(),
            false,
        )?);

        Ok(Self {
            object_store,
            root_path: format!("adls2://{}", container),
            root,
            storage_options: None,
        })
    }

    pub fn object_store(&self) -> Arc<DynObjectStore> {
        self.object_store.clone()
    }

    pub async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef> {
        let table = self.table_provider(&source.into()).await?;
        Ok(table.schema())
    }

    #[allow(clippy::manual_async_fn)]
    #[fix_hidden_lifetime_bug::fix_hidden_lifetime_bug]
    pub async fn put_batches(
        &self,
        #[allow(unused_mut)] mut input: SendableRecordBatchStream,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<()> {
        let files = match save_mode {
            SaveMode::Overwrite | SaveMode::ErrorIfExists => {
                self.object_store()
                    .list(Some(location))
                    .await?
                    .try_collect::<Vec<_>>()
                    .await?
            }
            _ => vec![],
        };

        match save_mode {
            SaveMode::Overwrite => {
                for file in files {
                    self.object_store().delete(&file.location).await?;
                }
                Ok(())
            }
            SaveMode::ErrorIfExists if !files.is_empty() => Err(
                AreaStoreError::TableAlreadyExists(location.as_ref().to_string()),
            ),
            _ => Ok(()),
        }?;

        let schema = input.schema();
        let full_path = self.full_path_delta(location)?;
        let mut writer = RecordBatchWriter::try_new(full_path, schema.clone(), None, None).unwrap();

        while let Some(maybe_batch) = input.next().await {
            let batch = maybe_batch.unwrap();
            writer.write(batch).await.unwrap();
        }
        let _adds = writer.flush().await.unwrap();

        Ok(())
    }

    /// Stream RecordBatches from a parquet file
    pub async fn open_file(
        &self,
        file: &AreaPath,
        column_indices: Option<Vec<usize>>,
    ) -> Result<SendableRecordBatchStream> {
        open_file(self.object_store(), file, column_indices).await
    }

    pub async fn delete_location(&self, _location: &AreaPath) -> Result<()> {
        todo!()
    }

    pub async fn list_areas(&self, prefix: Option<&Path>) -> Result<Vec<AreaPath>> {
        Ok(self
            .clone()
            .object_store()
            .list(prefix)
            .await?
            .filter_map(|maybe_path| async {
                match maybe_path {
                    Err(_) => None,
                    Ok(meta) => AreaPath::from(meta.location).table_root(),
                }
            })
            .collect::<std::collections::HashSet<_>>()
            .await
            .into_iter()
            .collect())
    }

    pub async fn table_provider(&self, location: &AreaPath) -> Result<Arc<dyn TableProvider>> {
        if is_delta_location(self.object_store().clone(), location).await? {
            Ok(Arc::new(self.open_delta(location).await?))
        } else {
            Ok(Arc::new(self.open_listing_table(location).await?))
        }
    }

    pub async fn open_listing_table(&self, location: &AreaPath) -> Result<ListingTable> {
        let store_url = self.root.object_store();
        let url: &url::Url = store_url.as_ref();
        let files = self
            .object_store()
            .list(Some(&location.into()))
            .await?
            .try_filter_map(|meta| async move {
                Ok(ListingTableUrl::parse(format!(
                    "{}://{}/{}",
                    url.scheme(),
                    url.host_str().unwrap_or(""),
                    meta.location.as_ref()
                ))
                .ok())
            })
            .try_collect::<Vec<_>>()
            .await?;
        // TODO handle session context more formally
        let ctx = SessionContext::new();
        ctx.runtime_env().register_object_store(
            url.scheme(),
            url.host_str().unwrap_or(""),
            self.object_store(),
        );
        let opt = ListingOptions::new(Arc::new(ParquetFormat::default()));
        let schema = opt.infer_schema(&ctx.state(), &files[0]).await?;
        let config = ListingTableConfig::new_with_multi_paths(files)
            .with_listing_options(opt)
            .with_schema(schema);

        Ok(ListingTable::try_new(config)?)
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
        let full_path = self.full_path_delta(&location.into())?;

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

    fn full_path_delta(&self, location: &Path) -> Result<String> {
        match StorageService::from(&self.root) {
            StorageService::Local(path) => {
                std::fs::create_dir_all(path.as_path()).map_err(|err| AreaStoreError::Generic {
                    source: Box::new(err),
                })?;

                Ok(format!("{}{}", self.root.as_str(), location.as_ref()))
            }
            StorageService::Azure(config) => {
                let full = self.root.add_prefix(location);
                Ok(format!("adls2://{}/{}", config.container, full.as_ref()))
            }
            _ => todo!(),
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
    use crate::test_utils::{get_input_stream, workspace_test_data_folder};
    use arrow_deps::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
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

        let stream = get_input_stream(None, false);
        let schema = stream.schema();
        area_store
            .put_batches(stream, &location.clone().into(), SaveMode::Append)
            .await
            .unwrap();

        let table = area_store.table_provider(&location).await.unwrap();

        assert_eq!(table.schema(), schema)
    }

    #[tokio::test]
    async fn read_schema() {
        let area_root = workspace_test_data_folder();
        let area_store = Arc::new(AreaStore::try_new(area_root).unwrap());

        let ref_schema = Arc::new(Schema::new(vec![
            Field::new(
                "timestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                true,
            ),
            Field::new("S2", DataType::Float64, true),
            Field::new("S3", DataType::Float64, true),
            Field::new("S5", DataType::Float64, true),
        ]));

        let source = AreaSourceReference {
            table: Some(TableReference::Location(AreaTableLocation {
                name: "simple".to_string(),
                areas: vec!["listing".to_string()],
            })),
        };
        let schema = area_store.get_schema(&source).await.unwrap();
        assert_eq!(schema, ref_schema);
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

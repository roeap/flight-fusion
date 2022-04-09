// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! ObjectStore implementation for the Azure Datalake Gen2 API
use std::io::{self, Read};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use async_trait::async_trait;
use azure_core::new_http_client;
use azure_core::prelude::Range;
use azure_storage::core::clients::{AsStorageClient, StorageAccountClient, StorageClient};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use azure_storage_blobs::prelude::*;
use azure_storage_datalake::clients::{DataLakeClient, FileClient};
use datafusion_data_access::{
    object_store::{FileMetaStream, ListEntryStream, ObjectReader, ObjectStore},
    FileMeta, Result as DFAccessResult, SizedFile,
};
use futures::AsyncRead;

async fn new_client(account_name: String, account_key: String) -> DataLakeClient {
    let credential = StorageSharedKeyCredential::new(account_name, account_key);
    DataLakeClient::new(credential, None)
}

async fn new_blob_client(account_name: String, account_key: String) -> Arc<StorageClient> {
    let http_client = new_http_client();
    let storage_account_client =
        StorageAccountClient::new_access_key(http_client.clone(), account_name, account_key);
    storage_account_client.as_storage_client()
    // let container_client = storage_client.as_container_client(file_system_name.to_owned());
}

/// `ObjectStore` implementation for the Azure Datalake Gen2 API
#[derive(Debug)]
pub struct AzureFileSystem {
    client: DataLakeClient,
    storage_client: Arc<StorageClient>,
}

impl AzureFileSystem {
    /// Create new `ObjectStore`
    pub async fn new<A>(account_name: A, account_key: String) -> Self
    where
        A: Into<String>,
    {
        let account: String = account_name.into();
        Self {
            client: new_client(account.clone(), account_key.clone()).await,
            storage_client: new_blob_client(account, account_key).await,
        }
    }
}

#[async_trait]
impl ObjectStore for AzureFileSystem {
    async fn list_file(&self, prefix: &str) -> DFAccessResult<FileMetaStream> {
        let (file_system, prefix) = match prefix.split_once("/") {
            Some((file_system, prefix)) => (file_system.to_owned(), prefix),
            None => (prefix.to_owned(), ""),
        };

        // TODO we need to use this function once https://github.com/Azure/azure-sdk-for-rust/issues/720
        // is resolved, or implement pagination in the current implementation.
        // let this = self.client.clone();
        // let stream = this
        //     .into_file_system_client(file_system)
        //     .list_paths()
        //     .directory(prefix)
        //     .into_stream()
        //     .flat_map(|f| convert(f.unwrap()));
        // Ok(Box::pin(stream))

        let container_client = self.storage_client.as_container_client(file_system);
        let objs = container_client
            .list_blobs()
            .prefix(prefix)
            .execute()
            .await
            .unwrap()
            .blobs
            .blobs
            .into_iter()
            .map(|blob| {
                let sized_file = SizedFile {
                    size: blob.properties.content_length,
                    path: blob.name,
                };
                Ok(FileMeta {
                    sized_file,
                    last_modified: Some(blob.properties.last_modified),
                })
            })
            .collect::<Vec<Result<FileMeta, _>>>();

        let output = futures::stream::iter(objs);
        Ok(Box::pin(output))
    }

    async fn list_dir(
        &self,
        _prefix: &str,
        _delimiter: Option<String>,
    ) -> DFAccessResult<ListEntryStream> {
        todo!()
    }

    fn file_reader(&self, file: SizedFile) -> DFAccessResult<Arc<dyn ObjectReader>> {
        let client = self
            .client
            .clone()
            .into_file_system_client("file_system_name")
            .into_file_client("path");
        Ok(Arc::new(AzureFileReader::new(client, file)?))
    }
}

struct AzureFileReader {
    client: FileClient,
    file: SizedFile,
}

impl AzureFileReader {
    #[allow(clippy::too_many_arguments)]
    fn new(client: FileClient, file: SizedFile) -> DFAccessResult<Self> {
        Ok(Self { client, file })
    }
}

#[async_trait]
impl ObjectReader for AzureFileReader {
    async fn chunk_reader(
        &self,
        _start: u64,
        _length: usize,
    ) -> DFAccessResult<Box<dyn AsyncRead>> {
        todo!("implement once async file readers are available (arrow-rs#78, arrow-rs#111)")
    }

    fn sync_chunk_reader(
        &self,
        start: u64,
        length: usize,
    ) -> DFAccessResult<Box<dyn Read + Send + Sync>> {
        let file_path = self.file.path.clone();
        let client = self.client.clone();

        // once the async chunk file readers have been implemented this complexity can be removed
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();

            rt.block_on(async move {
                let (file_system, prefix) = match file_path.split_once("/") {
                    Some((file_system, prefix)) => (file_system.to_owned(), prefix),
                    None => (file_path.to_owned(), ""),
                };

                let get_object = client.read();
                let resp = if length > 0 {
                    // range bytes requests are inclusive
                    get_object
                        .range(Range::new(start, start + (length - 1) as u64))
                        .into_future()
                        .await
                } else {
                    get_object.into_future().await
                };

                let bytes = match resp {
                    Ok(res) => Ok(res.data),
                    // TODO more precise error
                    Err(err) => Err(io::Error::from(io::ErrorKind::ConnectionRefused)),
                };

                tx.send(bytes).unwrap();
            })
        });

        let bytes = rx
            .recv_timeout(Duration::from_secs(10))
            // TODO more precise error
            .map_err(|err| io::Error::from(io::ErrorKind::ConnectionRefused))??;

        Ok(Box::new(bytes.reader()))
    }

    fn length(&self) -> u64 {
        self.file.size
    }
}

#[cfg(test)]
mod tests {
    use crate::object_store::azure::*;
    use datafusion::assert_batches_eq;
    use datafusion::datasource::listing::*;
    use datafusion::datasource::TableProvider;
    use datafusion::prelude::ExecutionContext;
    use futures::StreamExt;
    use http::Uri;

    const ACCESS_KEY_ID: &str = "AKIAIOSFODNN7EXAMPLE";
    const SECRET_ACCESS_KEY: &str = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
    const PROVIDER_NAME: &str = "Static";
    const MINIO_ENDPOINT: &str = "http://localhost:9000";

    // Test that `AzureFileSystem` can read files
    #[tokio::test]
    async fn test_read_files() -> Result<()> {
        let azure_file_system = AzureFileSystem::new(
            Some(SharedCredentialsProvider::new(Credentials::new(
                ACCESS_KEY_ID,
                SECRET_ACCESS_KEY,
                None,
                None,
                PROVIDER_NAME,
            ))),
            None,
            Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
            None,
            None,
            None,
        )
        .await;

        let mut files = azure_file_system.list_file("data").await?;

        while let Some(file) = files.next().await {
            let sized_file = file.unwrap().sized_file;
            let mut reader = azure_file_system
                .file_reader(sized_file.clone())
                .unwrap()
                .sync_chunk_reader(0, sized_file.size as usize)
                .unwrap();

            let mut bytes = Vec::new();
            let size = reader.read_to_end(&mut bytes)?;

            assert_eq!(size as u64, sized_file.size);
        }

        Ok(())
    }

    // Test that reading files with `AzureFileSystem` produces the expected results
    #[tokio::test]
    async fn test_read_range() -> Result<()> {
        let start = 10;
        let length = 128;

        let mut file = std::fs::File::open("parquet-testing/data/alltypes_plain.snappy.parquet")?;
        let mut raw_bytes = Vec::new();
        file.read_to_end(&mut raw_bytes)?;
        let raw_slice = &raw_bytes[start..start + length];
        assert_eq!(raw_slice.len(), length);

        let azure_file_system = AzureFileSystem::new(
            Some(SharedCredentialsProvider::new(Credentials::new(
                ACCESS_KEY_ID,
                SECRET_ACCESS_KEY,
                None,
                None,
                PROVIDER_NAME,
            ))),
            None,
            Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
            None,
            None,
            None,
        )
        .await;
        let mut files = azure_file_system
            .list_file("data/alltypes_plain.snappy.parquet")
            .await?;

        if let Some(file) = files.next().await {
            let sized_file = file.unwrap().sized_file;
            let mut reader = azure_file_system
                .file_reader(sized_file)
                .unwrap()
                .sync_chunk_reader(start as u64, length)
                .unwrap();

            let mut reader_bytes = Vec::new();
            let size = reader.read_to_end(&mut reader_bytes)?;

            assert_eq!(size, length);
            assert_eq!(&reader_bytes, raw_slice);
        }

        Ok(())
    }

    // Test that reading Parquet file with `AzureFileSystem` can create a `ListingTable`
    #[tokio::test]
    async fn test_read_parquet() -> Result<()> {
        let azure_file_system = Arc::new(
            AzureFileSystem::new(
                Some(SharedCredentialsProvider::new(Credentials::new(
                    ACCESS_KEY_ID,
                    SECRET_ACCESS_KEY,
                    None,
                    None,
                    PROVIDER_NAME,
                ))),
                None,
                Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
                None,
                None,
                None,
            )
            .await,
        );

        let filename = "data/alltypes_plain.snappy.parquet";

        let config = ListingTableConfig::new(azure_file_system, filename)
            .infer()
            .await?;

        let table = ListingTable::try_new(config)?;

        let exec = table.scan(&None, &[], Some(1024)).await?;
        assert_eq!(exec.statistics().num_rows, Some(2));

        Ok(())
    }

    // Test that a SQL query can be executed on a Parquet file that was read from `AzureFileSystem`
    #[tokio::test]
    async fn test_sql_query() -> Result<()> {
        let azure_file_system = Arc::new(
            AzureFileSystem::new(
                Some(SharedCredentialsProvider::new(Credentials::new(
                    ACCESS_KEY_ID,
                    SECRET_ACCESS_KEY,
                    None,
                    None,
                    PROVIDER_NAME,
                ))),
                None,
                Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
                None,
                None,
                None,
            )
            .await,
        );

        let filename = "data/alltypes_plain.snappy.parquet";

        let config = ListingTableConfig::new(azure_file_system, filename)
            .infer()
            .await?;

        let table = ListingTable::try_new(config)?;

        let mut ctx = ExecutionContext::new();

        ctx.register_table("tbl", Arc::new(table)).unwrap();

        let batches = ctx.sql("SELECT * FROM tbl").await?.collect().await?;
        let expected = vec![
        "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+",
        "| id | bool_col | tinyint_col | smallint_col | int_col | bigint_col | float_col | double_col | date_string_col  | string_col | timestamp_col       |",
        "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+",
        "| 6  | true     | 0           | 0            | 0       | 0          | 0         | 0          | 30342f30312f3039 | 30         | 2009-04-01 00:00:00 |",
        "| 7  | false    | 1           | 1            | 1       | 10         | 1.1       | 10.1       | 30342f30312f3039 | 31         | 2009-04-01 00:01:00 |",
        "+----+----------+-------------+--------------+---------+------------+-----------+------------+------------------+------------+---------------------+"
        ];
        assert_batches_eq!(expected, &batches);
        Ok(())
    }

    // Test that the AzureFileSystem allows reading from different buckets
    #[tokio::test]
    #[should_panic(expected = "Could not parse metadata: bad data")]
    async fn test_read_alternative_bucket() {
        let azure_file_system = Arc::new(
            AzureFileSystem::new(
                Some(SharedCredentialsProvider::new(Credentials::new(
                    ACCESS_KEY_ID,
                    SECRET_ACCESS_KEY,
                    None,
                    None,
                    PROVIDER_NAME,
                ))),
                None,
                Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
                None,
                None,
                None,
            )
            .await,
        );

        let filename = "bad_data/PARQUET-1481.parquet";

        let config = ListingTableConfig::new(azure_file_system, filename)
            .infer()
            .await
            .unwrap();

        let table = ListingTable::try_new(config).unwrap();

        table.scan(&None, &[], Some(1024)).await.unwrap();
    }

    // Test that `AzureFileSystem` can be registered as object store on a DataFusion `ExecutionContext`
    #[tokio::test]
    async fn test_ctx_register_object_store() -> Result<()> {
        let azure_file_system = Arc::new(
            AzureFileSystem::new(
                Some(SharedCredentialsProvider::new(Credentials::new(
                    ACCESS_KEY_ID,
                    SECRET_ACCESS_KEY,
                    None,
                    None,
                    PROVIDER_NAME,
                ))),
                None,
                Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
                None,
                None,
                None,
            )
            .await,
        );

        let ctx = ExecutionContext::new();

        ctx.register_object_store("s3", azure_file_system);

        let (_, name) = ctx.object_store("s3").unwrap();
        assert_eq!(name, "s3");

        Ok(())
    }

    // Test that an appropriate error message is produced for a non existent bucket
    #[tokio::test]
    #[should_panic(expected = "NoSuchBucket")]
    async fn test_read_nonexistent_bucket() {
        let azure_file_system = AzureFileSystem::new(
            Some(SharedCredentialsProvider::new(Credentials::new(
                ACCESS_KEY_ID,
                SECRET_ACCESS_KEY,
                None,
                None,
                PROVIDER_NAME,
            ))),
            None,
            Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
            None,
            None,
            None,
        )
        .await;

        let mut files = azure_file_system
            .list_file("nonexistent_data")
            .await
            .unwrap();

        while let Some(file) = files.next().await {
            let sized_file = file.unwrap().sized_file;
            let mut reader = azure_file_system
                .file_reader(sized_file.clone())
                .unwrap()
                .sync_chunk_reader(0, sized_file.size as usize)
                .unwrap();

            let mut bytes = Vec::new();
            let size = reader.read_to_end(&mut bytes).unwrap();

            assert_eq!(size as u64, sized_file.size);
        }
    }

    // Test that no files are returned if a non existent file URI is provided
    #[tokio::test]
    async fn test_read_nonexistent_file() {
        let azure_file_system = AzureFileSystem::new(
            Some(SharedCredentialsProvider::new(Credentials::new(
                ACCESS_KEY_ID,
                SECRET_ACCESS_KEY,
                None,
                None,
                PROVIDER_NAME,
            ))),
            None,
            Some(Endpoint::immutable(Uri::from_static(MINIO_ENDPOINT))),
            None,
            None,
            None,
        )
        .await;
        let mut files = azure_file_system
            .list_file("data/nonexistent_file.txt")
            .await
            .unwrap();

        assert!(files.next().await.is_none())
    }
}

//! This module contains the IOx implementation for using Azure Blob storage as
//! the object store.
use crate::{
    path::{cloud::CloudPath, DELIMITER},
    GetResult, ListResult, ObjectMeta, ObjectStoreApi, ObjectStorePath,
};
use async_trait::async_trait;
use azure_core::prelude::*;
use azure_storage::core::clients::*;
use azure_storage::core::prelude::{StorageAccountClient, StorageAccountOptions};
use azure_storage_blobs::prelude::{
    AsBlobClient, AsContainerClient, ContainerClient, DeleteSnapshotsMethod,
};
use bytes::Bytes;
use futures::{
    stream::{self, BoxStream},
    FutureExt, StreamExt,
};
use std::{convert::TryInto, sync::Arc};

/// A specialized `Result` for Azure object store-related errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// A specialized `Error` for Azure object store-related errors
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Unable to DELETE data. Location: {}, Error: {}", location, source)]
    Delete {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: String,
    },

    #[error("Unable to GET data. Location: {}, Error: {}", location, source)]
    Get {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: String,
    },

    #[error("Unable to PUT data. Location: {}, Error: {}", location, source)]
    Put {
        source: Box<dyn std::error::Error + Send + Sync>,
        location: String,
    },

    #[error("Unable to list data. Error: {}", source)]
    List {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[cfg(not(feature = "azure_test"))]
    #[error("Azurite (azure emulator) support not compiled in, please add `azure_test` feature")]
    NoEmulatorFeature,
}

/// Configuration for connecting to [Microsoft Azure Blob Storage](https://azure.microsoft.com/en-us/services/storage/blobs/).
#[derive(Debug)]
pub struct MicrosoftAzure {
    container_client: Arc<ContainerClient>,
    #[allow(dead_code)]
    container_name: String,
}

#[async_trait]
impl ObjectStoreApi for MicrosoftAzure {
    type Path = CloudPath;
    type Error = Error;

    fn new_path(&self) -> Self::Path {
        CloudPath::default()
    }

    fn path_from_raw(&self, raw: &str) -> Self::Path {
        CloudPath::raw(raw)
    }

    async fn put(&self, location: &Self::Path, bytes: Bytes) -> Result<()> {
        let location = location.to_raw();

        let bytes = bytes::BytesMut::from(&*bytes);

        self.container_client
            .as_blob_client(&location)
            .put_block_blob(bytes)
            .execute()
            .await
            .map_err(|err| Error::Put {
                source: err,
                location: location.to_owned(),
            })?;

        Ok(())
    }

    async fn get(&self, location: &Self::Path) -> Result<GetResult<Error>> {
        let container_client = Arc::clone(&self.container_client);
        let location = location.to_raw();
        let s = async move {
            container_client
                .as_blob_client(&location)
                .get()
                .execute()
                .await
                .map_err(|err| Error::Get {
                    source: err,
                    location: location.to_owned(),
                })
                .map(|blob| blob.data)
        }
        .into_stream()
        .boxed();

        Ok(GetResult::Stream(s))
    }

    async fn delete(&self, location: &Self::Path) -> Result<()> {
        let location = location.to_raw();
        self.container_client
            .as_blob_client(&location)
            .delete()
            .delete_snapshots_method(DeleteSnapshotsMethod::Include)
            .execute()
            .await
            .map_err(|err| Error::Delete {
                source: err,
                location: location.to_owned(),
            })?;

        Ok(())
    }

    async fn delete_dir(&self, location: &Self::Path) -> Result<()> {
        todo!()
    }

    async fn list<'a>(
        &'a self,
        prefix: Option<&'a Self::Path>,
    ) -> Result<BoxStream<'a, Result<Vec<Self::Path>>>> {
        let prefix_is_dir = prefix.map(|path| path.is_dir()).unwrap_or(true);

        #[derive(Clone)]
        enum ListState {
            Start,
            HasMore(String),
            Done,
        }

        Ok(stream::unfold(ListState::Start, move |state| async move {
            let mut request = self.container_client.list_blobs();

            let prefix_raw = prefix.map(|p| p.to_raw());
            if let Some(ref p) = prefix_raw {
                request = request.prefix(p as &str);
            }

            match state {
                ListState::HasMore(ref marker) => {
                    request = request.next_marker(marker as &str);
                }
                ListState::Done => {
                    return None;
                }
                ListState::Start => {}
            }

            let resp = match request
                .execute()
                .await
                .map_err(|err| Error::List { source: err })
            {
                Ok(resp) => resp,
                Err(err) => return Some((Err(err), state)),
            };

            let next_state = if let Some(marker) = resp.next_marker {
                ListState::HasMore(marker.as_str().to_string())
            } else {
                ListState::Done
            };

            let names = resp
                .blobs
                .blobs
                .into_iter()
                .map(|blob| CloudPath::raw(blob.name))
                .filter(move |path| {
                    prefix_is_dir || prefix.map(|prefix| prefix == path).unwrap_or_default()
                })
                .collect();

            Some((Ok(names), next_state))
        })
        .boxed())
    }

    async fn list_with_delimiter(&self, prefix: &Self::Path) -> Result<ListResult<Self::Path>> {
        let prefix_is_dir = prefix.is_dir();
        let mut request = self.container_client.list_blobs();

        let prefix_raw = prefix.to_raw();

        request = request.delimiter(Delimiter::new(DELIMITER));
        request = request.prefix(&*prefix_raw);

        let resp = request
            .execute()
            .await
            .map_err(|err| Error::List { source: err })?;

        let next_token = resp.next_marker.as_ref().map(|m| m.as_str().to_string());

        let common_prefixes = resp
            .blobs
            .blob_prefix
            .map(|prefixes| {
                prefixes
                    .iter()
                    .map(|prefix| CloudPath::raw(&prefix.name))
                    .collect()
            })
            .unwrap_or_else(Vec::new);

        let objects = resp
            .blobs
            .blobs
            .into_iter()
            .map(|blob| {
                let location = CloudPath::raw(blob.name);
                let last_modified = blob.properties.last_modified;
                let size = blob
                    .properties
                    .content_length
                    .try_into()
                    .expect("unsupported size on this platform");

                ObjectMeta {
                    location,
                    last_modified,
                    size,
                }
            })
            .filter(move |object| prefix_is_dir || prefix == &object.location)
            .collect();

        Ok(ListResult {
            next_token,
            common_prefixes,
            objects,
        })
    }
}

#[cfg(feature = "azure_test")]
fn check_if_emulator_works() -> Result<()> {
    Ok(())
}

#[cfg(not(feature = "azure_test"))]
fn check_if_emulator_works() -> Result<()> {
    Err(Error::NoEmulatorFeature)
}

/// Configure a connection to container with given name on Microsoft Azure
/// Blob store.
///
/// The credentials `account` and `access_key` must provide access to the
/// store.
pub fn new_azure(
    account: impl Into<String>,
    access_key: impl Into<String>,
    container_name: impl Into<String>,
    _use_emulator: bool,
) -> Result<MicrosoftAzure> {
    let account = account.into();
    let access_key = access_key.into();

    // let storage_account_client = if use_emulator {
    //     check_if_emulator_works()?;
    //     StorageAccountClient::new_emulator_default()
    // } else {
    //     let options = StorageAccountOptions::default();
    //     Arc::new(StorageAccountClient::new_access_key(
    //         &account,
    //         &access_key,
    //         options,
    //     ))
    // };

    let options = StorageAccountOptions::default();
    let storage_account_client =
        StorageAccountClient::new_access_key(&account, &access_key, options);

    let storage_client = storage_account_client.clone().into_storage_client();

    let container_name = container_name.into();

    let container_client = storage_client.as_container_client(&container_name);

    Ok(MicrosoftAzure {
        container_client,
        container_name,
    })
}

#[cfg(test)]
mod tests {
    use crate::tests::{list_uses_directories_correctly, list_with_delimiter, put_get_delete_list};
    use crate::ObjectStore;
    use std::env;

    #[derive(Debug)]
    struct AzureConfig {
        storage_account: String,
        access_key: String,
        bucket: String,
        use_emulator: bool,
    }

    // Helper macro to skip tests if TEST_INTEGRATION and the Azure environment
    // variables are not set.
    macro_rules! maybe_skip_integration {
        () => {{
            dotenv::dotenv().ok();

            let use_emulator = std::env::var("AZURE_USE_EMULATOR").is_ok();

            let mut required_vars = vec!["FLIGHT_FUSION_BUCKET"];
            if !use_emulator {
                required_vars.push("AZURE_STORAGE_ACCOUNT");
                required_vars.push("AZURE_STORAGE_ACCESS_KEY");
            }
            let unset_vars: Vec<_> = required_vars
                .iter()
                .filter_map(|&name| match env::var(name) {
                    Ok(_) => None,
                    Err(_) => Some(name),
                })
                .collect();
            let unset_var_names = unset_vars.join(", ");

            let force = std::env::var("TEST_INTEGRATION");

            if force.is_ok() && !unset_var_names.is_empty() {
                panic!(
                    "TEST_INTEGRATION is set, \
                        but variable(s) {} need to be set",
                    unset_var_names
                )
            } else if force.is_err() {
                eprintln!(
                    "skipping Azure integration test - set {}TEST_INTEGRATION to run",
                    if unset_var_names.is_empty() {
                        String::new()
                    } else {
                        format!("{} and ", unset_var_names)
                    }
                );
                return;
            } else {
                AzureConfig {
                    storage_account: env::var("AZURE_STORAGE_ACCOUNT").unwrap_or_default(),
                    access_key: env::var("AZURE_STORAGE_ACCESS_KEY").unwrap_or_default(),
                    bucket: env::var("FLIGHT_FUSION_BUCKET")
                        .expect("already checked FLIGHT_FUSION_BUCKET"),
                    use_emulator,
                }
            }
        }};
    }

    #[tokio::test]
    async fn azure_blob_test() {
        let config = maybe_skip_integration!();
        let integration = ObjectStore::new_microsoft_azure(
            config.storage_account,
            config.access_key,
            config.bucket,
            config.use_emulator,
        )
        .unwrap();

        put_get_delete_list(&integration).await.unwrap();
        list_uses_directories_correctly(&integration).await.unwrap();
        list_with_delimiter(&integration).await.unwrap();
    }
}

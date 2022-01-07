//! Crate that mimics the interface of the the various object stores
//! but does nothing if they are not enabled.
use std::num::NonZeroUsize;

use async_trait::async_trait;
use bytes::Bytes;

use crate::{path::cloud::CloudPath, GetResult, ObjectStoreApi};

/// A specialized `Error` for Azure object store-related errors
#[derive(Debug, thiserror::Error, Clone)]
#[allow(missing_docs)]
pub enum Error {
    #[error("'{name}' not supported with this build. Hint: recompile with appropriate features")]
    NotSupported { name: String },
}
/// Result for the dummy object store
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Clone)]
/// An object store that always generates an error
pub struct DummyObjectStore {
    name: String,
}

/// If aws feature not available, use DummyObjectStore
// pub type AmazonS3 = DummyObjectStore;

/// If azure feature not available, use DummyObjectStore
pub type MicrosoftAzure = DummyObjectStore;

/// If gcp feature not available, use DummyObjectStore
// pub type GoogleCloudStorage = DummyObjectStore;

#[async_trait]
impl ObjectStoreApi for DummyObjectStore {
    type Path = CloudPath;
    type Error = Error;

    fn new_path(&self) -> Self::Path {
        CloudPath::default()
    }

    fn path_from_raw(&self, raw: &str) -> Self::Path {
        CloudPath::raw(raw)
    }

    async fn put(&self, _location: &Self::Path, _bytes: Bytes) -> crate::Result<(), Self::Error> {
        Err(Error::NotSupported {
            name: self.name.clone(),
        })
    }

    async fn get(
        &self,
        _location: &Self::Path,
    ) -> crate::Result<GetResult<Self::Error>, Self::Error> {
        Err(Error::NotSupported {
            name: self.name.clone(),
        })
    }

    async fn delete(&self, _location: &Self::Path) -> crate::Result<(), Self::Error> {
        Err(Error::NotSupported {
            name: self.name.clone(),
        })
    }

    async fn list<'a>(
        &'a self,
        _prefix: Option<&'a Self::Path>,
    ) -> crate::Result<
        futures::stream::BoxStream<'a, crate::Result<Vec<Self::Path>, Self::Error>>,
        Self::Error,
    > {
        Err(Error::NotSupported {
            name: self.name.clone(),
        })
    }

    async fn list_with_delimiter(
        &self,
        _prefix: &Self::Path,
    ) -> crate::Result<crate::ListResult<Self::Path>, Self::Error> {
        Err(Error::NotSupported {
            name: self.name.clone(),
        })
    }
}

/// Stub when s3 is not configured
// #[allow(dead_code)]
// pub(crate) fn new_s3(
//     _access_key_id: Option<impl Into<String>>,
//     _secret_access_key: Option<impl Into<String>>,
//     _region: impl Into<String>,
//     _bucket_name: impl Into<String>,
//     _endpoint: Option<impl Into<String>>,
//     _session_token: Option<impl Into<String>>,
//     _max_connections: NonZeroUsize,
// ) -> Result<DummyObjectStore> {
//     Err(Error::NotSupported {
//         name: "aws".to_string(),
//     })
// }

// #[allow(dead_code)]
// pub(crate) fn new_failing_s3() -> Result<AmazonS3> {
//     Ok(DummyObjectStore { name: "aws".into() })
// }
//
// /// Stub when gcs is not configured
// #[allow(dead_code)]
// pub(crate) fn new_gcs(
//     _service_account_path: impl AsRef<std::ffi::OsStr>,
//     _bucket_name: impl Into<String>,
// ) -> Result<DummyObjectStore> {
//     Err(Error::NotSupported {
//         name: "gcs".to_string(),
//     })
// }

/// Stub when azure is not configured
#[allow(dead_code)]
pub(crate) fn new_azure(
    _account: impl Into<String>,
    _access_key: impl Into<String>,
    _container_name: impl Into<String>,
    _use_emulator: bool,
) -> Result<DummyObjectStore> {
    Err(Error::NotSupported {
        name: "azure".to_string(),
    })
}

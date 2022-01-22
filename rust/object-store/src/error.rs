use crate::{azure, disk, memory};

/// A specialized `Error` for object store-related errors
#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum Error {
    #[error("File-based Object Store error: {}", source)]
    FileObjectStoreError {
        #[from]
        source: disk::Error,
    },

    // #[error("Google Cloud Storage-based Object Store error: {}", source)]
    // GcsObjectStoreError {
    //     #[from]
    //     source: gcp::Error,
    // },
    //
    // #[error("AWS S3-based Object Store error: {}", source)]
    // AwsObjectStoreError {
    //     #[from]
    //     source: aws::Error,
    // },
    #[error("Azure Blob storage-based Object Store error: {}", source)]
    AzureObjectStoreError {
        #[from]
        source: azure::Error,
    },

    #[error("In-memory-based Object Store error: {}", source)]
    InMemoryObjectStoreError {
        #[from]
        source: memory::Error,
    },

    // #[error("{}", source)]
    // DummyObjectStoreError { source: dummy::Error },
    #[error("Object at location {} not found: {}", location, source)]
    NotFound {
        location: String,
        source: Box<dyn std::error::Error + Send + Sync + 'static>,
    },
}

/// A specialized `Result` for object store-related errors
pub type Result<T, E = Error> = std::result::Result<T, E>;

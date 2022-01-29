use crate::store::error::AreaStoreError;
use futures::Stream;
use std::pin::Pin;

#[derive(thiserror::Error, Debug)]
pub enum AreaCatalogError {
    #[error(transparent)]
    StorageError(#[from] AreaStoreError),
}

pub type AreaCatalogResult<T> = std::result::Result<T, AreaCatalogError>;

/// Result type for fallible streaming operations defined in this crate
pub type AreaCatalogResultStream<T> =
    AreaCatalogResult<Pin<Box<dyn Stream<Item = AreaCatalogResult<T>> + Send + Sync + 'static>>>;

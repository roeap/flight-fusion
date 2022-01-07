//! Abstractions and implementations for writing data to delta tables
use std::path::PathBuf;

use arrow_deps::arrow::{datatypes::*, record_batch::*};
use arrow_deps::datafusion::parquet::basic::LogicalType;
pub use error::*;
pub use utils::*;
pub use writer::*;

pub mod error;
pub mod json;
mod stats;
pub mod utils;
pub mod writer;

pub trait AreaStore {
    fn store_batches(&self);
    fn object_store(&self) -> &object_store::ObjectStore;
}

pub struct InMemoryAreaStore {
    object_store: object_store::ObjectStore,
}

impl InMemoryAreaStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let object_store = object_store::ObjectStore::new_file(root);
        Self { object_store }
    }
}

impl AreaStore for InMemoryAreaStore {
    fn object_store(&self) -> &object_store::ObjectStore {
        &self.object_store
    }

    fn store_batches(&self) {
        println!("hello")
    }
}

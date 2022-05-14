//! Abstractions and implementations for writing data to delta tables
mod basic;
mod cache;
mod stats;
pub mod utils;
pub mod writer;

use crate::error::Result;
use arrow_deps::arrow::{
    datatypes::SchemaRef as ArrowSchemaRef,
    error::{ArrowError, Result as ArrowResult},
    record_batch::RecordBatch,
};
use arrow_deps::datafusion::arrow::record_batch::RecordBatchReader;
use arrow_deps::datafusion::parquet::arrow::async_reader::ParquetRecordBatchStream;
use arrow_deps::datafusion::parquet::arrow::{ArrowReader, ParquetFileArrowReader};
use arrow_deps::datafusion::parquet::{
    basic::LogicalType,
    file::serialized_reader::{SerializedFileReader, SliceableCursor},
};
use arrow_deps::datafusion::physical_plan::{
    stream::RecordBatchStreamAdapter, RecordBatchStream, SendableRecordBatchStream,
};
use async_trait::async_trait;
pub use basic::DefaultAreaStore;
pub use cache::CachedAreaStore;
use flight_fusion_ipc::{AreaSourceReference, SaveMode};
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use object_store::{path::Path, DynObjectStore};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncSeek};
pub use utils::*;
pub use writer::*;

const DATA_FOLDER_NAME: &str = "_ff_data";
const DEFAULT_READ_BATCH_SIZE: usize = 1024;

pub trait AsyncReader: AsyncRead + AsyncSeek + Send {}

pub struct FileReaderStream(ParquetRecordBatchStream<Box<dyn AsyncReader + Unpin>>);

impl Stream for FileReaderStream {
    type Item = ArrowResult<RecordBatch>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.0.poll_next_unpin(cx) {
            Poll::Ready(opt) => Poll::Ready(match opt {
                Some(data) => Some(data.map_err(|err| ArrowError::ParquetError(err.to_string()))),
                None => None,
            }),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl RecordBatchStream for FileReaderStream {
    fn schema(&self) -> ArrowSchemaRef {
        self.0.schema().clone()
    }
}

#[async_trait]
pub trait AreaStore: Send + Sync {
    // path manipulation

    /// Get a reference to the underlying object store
    fn object_store(&self) -> Arc<DynObjectStore>;

    fn get_path_from_raw(&self, raw: String) -> Path;

    /// Resolve an [`AreaSourceReference`] to a storage location
    fn get_table_location(&self, source: &AreaSourceReference) -> Result<Path>;

    async fn get_schema(&self, source: &AreaSourceReference) -> Result<ArrowSchemaRef>;

    /// Write batches into table location
    async fn put_batches(
        &self,
        batches: Vec<RecordBatch>,
        location: &Path,
        save_mode: SaveMode,
    ) -> Result<Vec<stats::Add>>;

    /// Read batches from location
    async fn get_batches(&self, location: &Path) -> Result<Vec<RecordBatch>>;

    async fn get_arrow_reader(&self, location: &Path) -> Result<ParquetFileArrowReader>;

    async fn read_file(&self, file: &Path) -> Result<Vec<RecordBatch>> {
        let mut reader = self.get_arrow_reader(file).await?;
        let batch_reader = reader.get_record_reader(DEFAULT_READ_BATCH_SIZE)?;
        Ok(batch_reader
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()?)
    }

    async fn open_file(&self, file: &Path) -> Result<SendableRecordBatchStream> {
        let bytes = self.object_store().get(file).await?.bytes().await?;
        let cursor = SliceableCursor::new(Arc::new(bytes));
        let file_reader = Arc::new(SerializedFileReader::new(cursor)?);
        let mut arrow_reader = ParquetFileArrowReader::new(file_reader);
        let record_batch_reader = arrow_reader.get_record_reader(2048)?;

        Ok(Box::pin(RecordBatchStreamAdapter::new(
            record_batch_reader.schema(),
            futures::stream::iter(record_batch_reader),
        )))
    }

    /// Resolve an [`AreaSourceReference`] to the files relevant for source reference
    async fn get_source_files(&self, source: &AreaSourceReference) -> Result<Vec<Path>> {
        let location = self.get_table_location(source)?;
        self.get_location_files(&location).await
    }

    async fn get_location_files(&self, location: &Path) -> Result<Vec<Path>> {
        Ok(self
            .object_store()
            .list(Some(location))
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .into_iter()
            .map(|f| f.location)
            .collect::<Vec<_>>())
    }

    async fn delete_location(&self, _location: &Path) -> Result<()> {
        todo!()
    }
}

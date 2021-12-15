use datafusion::datasource::{
    object_store::{
        FileMeta, FileMetaStream, ListEntryStream, ObjectReader, ObjectReaderStream, ObjectStore,
        SizedFile,
    },
    PartitionedFile,
};
use datafusion::parquet::arrow::ArrowReader;
use datafusion::parquet::arrow::ParquetFileArrowReader;
use datafusion::parquet::errors::ParquetError;
use datafusion::parquet::errors::Result as ParquetResult;
use datafusion::parquet::file::reader::ChunkReader;
use datafusion::parquet::file::reader::Length;
use datafusion::parquet::file::serialized_reader::SerializedFileReader;
use datafusion::parquet::file::statistics::Statistics as ParquetStatistics;
pub use local::LocalFileSystem;
use std::io::Read;
use std::sync::Arc;

pub mod local;

pub struct ChunkObjectReader(pub Arc<dyn ObjectReader>);

impl Length for ChunkObjectReader {
    fn len(&self) -> u64 {
        self.0.length()
    }
}

impl ChunkReader for ChunkObjectReader {
    type T = Box<dyn Read + Send + Sync>;

    fn get_read(&self, start: u64, length: usize) -> ParquetResult<Self::T> {
        self.0
            .sync_chunk_reader(start, length)
            .map_err(|e| ParquetError::ArrowError(e.to_string()))
    }
}

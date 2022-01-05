use arrow_deps::datafusion::{
    datasource::object_store::ObjectReader,
    parquet::{
        errors::{ParquetError, Result as ParquetResult},
        file::reader::{ChunkReader, Length},
    },
};
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

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

#[derive(Debug)]
pub struct BytesReader(pub bytes::Bytes);

impl Length for BytesReader {
    fn len(&self) -> u64 {
        self.0.len() as u64
    }
}

impl Read for BytesReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(Read::read(&mut self.0.as_ref(), buf)?)
    }
}

impl ChunkReader for BytesReader {
    type T = Box<dyn Read + Send + Sync>;

    fn get_read(&self, start: u64, length: usize) -> ParquetResult<Self::T> {
        let start = start as usize;
        Ok(Box::new(BytesReader(self.0.slice(start..start + length))))
    }
}

// pub struct SliceableCursor {
//     inner: Arc<Vec<u8>>,
//     start: u64,
//     length: usize,
//     pos: u64,
// }
//
// impl SliceableCursor {
//     pub fn new(content: impl Into<Arc<Vec<u8>>>) -> Self {
//         let inner = content.into();
//         let size = inner.len();
//         SliceableCursor {
//             inner,
//             start: 0,
//             pos: 0,
//             length: size,
//         }
//     }
//
//     /// Create a slice cursor using the same data as a current one.
//     pub fn slice(&self, start: u64, length: usize) -> io::Result<Self> {
//         let new_start = self.start + start;
//         if new_start >= self.inner.len() as u64 || new_start as usize + length > self.inner.len() {
//             return Err(Error::new(ErrorKind::InvalidInput, "out of bound"));
//         }
//         Ok(SliceableCursor {
//             inner: Arc::clone(&self.inner),
//             start: new_start,
//             pos: new_start,
//             length,
//         })
//     }
//
//     fn remaining_slice(&self) -> &[u8] {
//         let end = self.start as usize + self.length;
//         let offset = cmp::min(self.pos, end as u64) as usize;
//         &self.inner[offset..end]
//     }
//
//     /// Get the length of the current cursor slice
//     pub fn len(&self) -> u64 {
//         self.length as u64
//     }
//
//     /// return true if the cursor is empty (self.len() == 0)
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }
// }
//
// impl Seek for SliceableCursor {
//     fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
//         let new_pos = match pos {
//             SeekFrom::Start(pos) => pos as i64,
//             SeekFrom::End(pos) => self.inner.len() as i64 + pos as i64,
//             SeekFrom::Current(pos) => self.pos as i64 + pos as i64,
//         };
//
//         if new_pos < 0 {
//             Err(Error::new(
//                 ErrorKind::InvalidInput,
//                 format!(
//                     "Request out of bounds: cur position {} + seek {:?} < 0: {}",
//                     self.pos, pos, new_pos
//                 ),
//             ))
//         } else if new_pos >= self.inner.len() as i64 {
//             Err(Error::new(
//                 ErrorKind::InvalidInput,
//                 format!(
//                     "Request out of bounds: cur position {} + seek {:?} >= length {}: {}",
//                     self.pos,
//                     pos,
//                     self.inner.len(),
//                     new_pos
//                 ),
//             ))
//         } else {
//             self.pos = new_pos as u64;
//             Ok(self.start)
//         }
//     }
// }

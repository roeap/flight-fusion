use std::fs::Metadata;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::Arc;
use tokio::{
    fs::{self, metadata, File},
    io::AsyncSeekExt,
};

use async_trait::async_trait;
use futures::{stream, AsyncRead, StreamExt};

use datafusion::datasource::{
    object_store::{
        FileMeta, FileMetaStream, ListEntryStream, ObjectReader, ObjectReaderStream, ObjectStore,
        SizedFile,
    },
    PartitionedFile,
};
use datafusion::error::DataFusionError;
use datafusion::error::Result;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[derive(Debug)]
/// Local File System as Object Store.
pub struct LocalFileSystem;

#[async_trait]
impl ObjectStore for LocalFileSystem {
    async fn list_file(&self, prefix: &str) -> Result<FileMetaStream> {
        list_all(prefix.to_owned()).await
    }

    async fn list_dir(&self, _prefix: &str, _delimiter: Option<String>) -> Result<ListEntryStream> {
        todo!()
    }

    fn file_reader(&self, file: SizedFile) -> Result<Arc<dyn ObjectReader>> {
        Ok(Arc::new(LocalFileReader::new(file)?))
    }
}

struct LocalFileReader {
    file: SizedFile,
}

impl LocalFileReader {
    fn new(file: SizedFile) -> Result<Self> {
        Ok(Self { file })
    }
}

#[async_trait]
impl ObjectReader for LocalFileReader {
    async fn chunk_reader(&self, start: u64, _length: usize) -> Result<Box<dyn AsyncRead>> {
        let mut file = fs::File::open(&self.file.path).await?;
        file.seek(SeekFrom::Start(start)).await?;
        Ok(Box::new(file.compat()))
    }

    fn sync_chunk_reader(&self, start: u64, length: usize) -> Result<Box<dyn Read + Send + Sync>> {
        // A new file descriptor is opened for each chunk reader.
        // This okay because chunks are usually fairly large.
        let mut file = std::fs::File::open(&self.file.path)?;
        file.seek(SeekFrom::Start(start))?;

        let file = BufReader::new(file.take(length as u64));

        Ok(Box::new(file))
    }

    fn length(&self) -> u64 {
        self.file.size
    }
}

async fn list_all(prefix: String) -> Result<FileMetaStream> {
    fn get_meta(path: String, metadata: Metadata) -> FileMeta {
        FileMeta {
            sized_file: SizedFile {
                path,
                size: metadata.len(),
            },
            last_modified: metadata.modified().map(chrono::DateTime::from).ok(),
        }
    }

    async fn find_files_in_dir(path: String, to_visit: &mut Vec<String>) -> Result<Vec<FileMeta>> {
        let mut dir = tokio::fs::read_dir(path).await?;
        let mut files = Vec::new();

        while let Some(child) = dir.next_entry().await? {
            if let Some(child_path) = child.path().to_str() {
                let metadata = child.metadata().await?;
                if metadata.is_dir() {
                    to_visit.push(child_path.to_string());
                } else {
                    files.push(get_meta(child_path.to_owned(), metadata))
                }
            } else {
                return Err(DataFusionError::Plan("Invalid path".to_string()));
            }
        }
        Ok(files)
    }

    let prefix_meta = tokio::fs::metadata(&prefix).await?;
    let prefix = prefix.to_owned();
    if prefix_meta.is_file() {
        Ok(Box::pin(stream::once(async move {
            Ok(get_meta(prefix, prefix_meta))
        })))
    } else {
        let result = stream::unfold(vec![prefix], move |mut to_visit| async move {
            match to_visit.pop() {
                None => None,
                Some(path) => {
                    let file_stream = match find_files_in_dir(path, &mut to_visit).await {
                        Ok(files) => stream::iter(files).map(Ok).left_stream(),
                        Err(e) => stream::once(async { Err(e) }).right_stream(),
                    };

                    Some((file_stream, to_visit))
                }
            }
        })
        .flatten();
        Ok(Box::pin(result))
    }
}

/// Create a stream of `ObjectReader` by converting each file in the `files` vector
/// into instances of `LocalFileReader`
pub fn local_object_reader_stream(files: Vec<String>) -> ObjectReaderStream {
    Box::pin(futures::stream::iter(files).map(|f| Ok(local_object_reader(f))))
}

/// Helper method to convert a file location to a `LocalFileReader`
pub fn local_object_reader(file: String) -> Arc<dyn ObjectReader> {
    LocalFileSystem
        .file_reader(sync_local_unpartitioned_file(file).file_meta.sized_file)
        .expect("File not found")
}

/// Helper method to fetch the file size and date at given path and create a `FileMeta`
pub fn sync_local_unpartitioned_file(file: String) -> PartitionedFile {
    let metadata = std::fs::metadata(&file).expect("Local file metadata");
    PartitionedFile {
        file_meta: FileMeta {
            sized_file: SizedFile {
                size: metadata.len(),
                path: file,
            },
            last_modified: metadata.modified().map(chrono::DateTime::from).ok(),
        },
        partition_values: vec![],
    }
}

/// Helper method to fetch the file size and date at given path and create a `FileMeta`
pub async fn local_unpartitioned_file(file: String) -> PartitionedFile {
    let metadata = fs::metadata(&file).await.expect("Local file metadata");
    PartitionedFile {
        file_meta: FileMeta {
            sized_file: SizedFile {
                size: metadata.len(),
                path: file,
            },
            last_modified: metadata.modified().map(chrono::DateTime::from).ok(),
        },
        partition_values: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::collections::HashSet;
    use std::fs::create_dir;
    use std::fs::File;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_recursive_listing() -> Result<()> {
        // tmp/a.txt
        // tmp/x/b.txt
        // tmp/y/c.txt
        let tmp = tempdir()?;
        let x_path = tmp.path().join("x");
        let y_path = tmp.path().join("y");
        let a_path = tmp.path().join("a.txt");
        let b_path = x_path.join("b.txt");
        let c_path = y_path.join("c.txt");
        create_dir(&x_path)?;
        create_dir(&y_path)?;
        File::create(&a_path)?;
        File::create(&b_path)?;
        File::create(&c_path)?;

        let mut all_files = HashSet::new();
        let mut files = list_all(tmp.path().to_str().unwrap().to_string()).await?;
        while let Some(file) = files.next().await {
            let file = file?;
            assert_eq!(file.size(), 0);
            all_files.insert(file.path().to_owned());
        }

        assert_eq!(all_files.len(), 3);
        assert!(all_files.contains(a_path.to_str().unwrap()));
        assert!(all_files.contains(b_path.to_str().unwrap()));
        assert!(all_files.contains(c_path.to_str().unwrap()));

        Ok(())
    }
}
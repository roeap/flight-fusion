use std::io;

/// Errors returned by this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// File too large to fit in the cache.
    #[error("File too large to fit in the cache.")]
    FileTooLarge,
    /// File not in the cache.
    #[error("File not in the cache.")]
    FileNotInCache,
    /// An IO Error occurred.
    #[error(transparent)]
    Io(#[from] io::Error),
}

/// A convenience `Result` type
pub type Result<T> = std::result::Result<T, Error>;

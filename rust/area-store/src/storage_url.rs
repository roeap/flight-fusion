use crate::error::{AreaStoreError, Result};
use object_store::path::Path;
use url::Url;

const AZURE_ADLS_HOST: &str = "dfs.core.windows.net";
const AZURE_BLOB_HOST: &str = "blob.core.windows.net";

/// A parsed URL identifying a particular [`ObjectStore`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectStoreUrl {
    url: Url,
}

impl ObjectStoreUrl {
    /// Parse an [`ObjectStoreUrl`] from a string
    pub fn parse(s: impl AsRef<str>) -> Result<Self> {
        let mut parsed =
            Url::parse(s.as_ref()).map_err(|e| AreaStoreError::External(Box::new(e)))?;

        let remaining = &parsed[url::Position::BeforePath..];
        if !remaining.is_empty() && remaining != "/" {
            return Err(AreaStoreError::Parsing(format!(
                "ObjectStoreUrl must only contain scheme and authority, got: {}",
                remaining
            )));
        }

        // Always set path for consistency
        parsed.set_path("/");
        Ok(Self { url: parsed })
    }

    /// An [`ObjectStoreUrl`] for the local filesystem
    pub fn local_filesystem() -> Self {
        Self::parse("file://").unwrap()
    }

    /// Returns this [`ObjectStoreUrl`] as a string
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }
}

impl AsRef<str> for ObjectStoreUrl {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl AsRef<Url> for ObjectStoreUrl {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::fmt::Display for ObjectStoreUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// A parsed URL identifying files for a listing table, see [`StorageUrl::parse`]
/// for more information on the supported expressions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageUrl {
    /// A URL that identifies a file or directory to list files from
    pub(crate) url: Url,
    /// The path prefix
    prefix: Path,
}

impl StorageUrl {
    /// Parse a provided string as a `StorageUrl`
    ///
    /// # Paths without a Scheme
    ///
    /// If no scheme is provided, or the string is an absolute filesystem path
    /// as determined [`std::path::Path::is_absolute`], the string will be
    /// interpreted as a path on the local filesystem using the operating
    /// system's standard path delimiter, i.e. `\` on Windows, `/` on Unix.
    ///
    /// Otherwise, the path will be resolved to an absolute path, returning
    /// an error if it does not exist, and converted to a [file URI]
    ///
    /// If you wish to specify a path that does not exist on the local
    /// machine you must provide it as a fully-qualified [file URI]
    /// e.g. `file:///myfile.txt`
    ///
    /// ## Glob File Paths
    ///
    /// If no scheme is provided, and the path contains a glob expression, it will
    /// be resolved as follows.
    ///
    /// The string up to the first path segment containing a glob expression will be extracted,
    /// and resolved in the same manner as a normal scheme-less path. That is, resolved to
    /// an absolute path on the local filesystem, returning an error if it does not exist,
    /// and converted to a [file URI]
    ///
    /// The remaining string will be interpreted as a [`glob::Pattern`] and used as a
    /// filter when listing files from object storage
    ///
    /// [file URI]: https://en.wikipedia.org/wiki/File_URI_scheme
    pub fn parse(s: impl AsRef<str>) -> Result<Self> {
        let s = s.as_ref();

        // This is necessary to handle the case of a path starting with a drive letter
        if std::path::Path::new(s).is_absolute() {
            return Self::parse_path(s);
        }

        match Url::parse(s) {
            Ok(url) => Ok(Self::new(url)),
            Err(url::ParseError::RelativeUrlWithoutBase) => Self::parse_path(s),
            Err(e) => Err(AreaStoreError::External(Box::new(e))),
        }
    }

    /// Creates a new [`StorageUrl`] interpreting `s` as a filesystem path
    fn parse_path(s: &str) -> Result<Self> {
        let path = std::path::Path::new(s).canonicalize()?;
        let url = match path.is_file() {
            true => Url::from_file_path(path).unwrap(),
            false => Url::from_directory_path(path).unwrap(),
        };

        Ok(Self::new(url))
    }

    /// Creates a new [`StorageUrl`] from a url and optional glob expression
    fn new(url: Url) -> Self {
        let prefix = Path::parse(url.path()).expect("should be URL safe");
        Self { url, prefix }
    }

    /// Returns the URL scheme
    pub fn scheme(&self) -> &str {
        self.url.scheme()
    }

    /// Strips the prefix of this [`StorageUrl`] from the provided path, returning
    /// an iterator of the remaining path segments
    pub fn strip_prefix<'a, 'b: 'a>(
        &'a self,
        path: &'b Path,
    ) -> Option<impl Iterator<Item = &'b str> + 'a> {
        use object_store::path::DELIMITER;
        let path: &str = path.as_ref();
        let stripped = match self.prefix.as_ref() {
            "" => path,
            p => path.strip_prefix(p)?.strip_prefix(DELIMITER)?,
        };
        Some(stripped.split(DELIMITER))
    }

    /// Adds the prefix of this [`StorageUrl`] to the provided path,
    /// returning a Path containing all segments
    pub fn add_prefix(&self, path: &Path) -> Path {
        let mut root = self.prefix.clone();
        path.parts().for_each(|part| {
            root = root.child(part);
        });
        root
    }

    /// Returns this [`StorageUrl`] as a string
    pub fn as_str(&self) -> &str {
        self.as_ref()
    }

    /// Return the [`ObjectStoreUrl`] for this [`StorageUrl`]
    pub fn object_store(&self) -> ObjectStoreUrl {
        let url = &self.url[url::Position::BeforeScheme..url::Position::BeforePath];
        ObjectStoreUrl::parse(url).unwrap()
    }
}

impl AsRef<str> for StorageUrl {
    fn as_ref(&self) -> &str {
        self.url.as_ref()
    }
}

impl AsRef<Url> for StorageUrl {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl std::fmt::Display for StorageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzureAccount {
    pub scheme: String,
    pub account: Option<String>,
    pub container: String,
    /// Denotes if the parsed uri is referencing a gen2 account.
    /// The service might still support request against gen2 endpoints, since
    /// the blob APIs are supported on all accounts.
    pub is_gen2: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S3Account {
    pub scheme: String,
    pub bucket: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoogleAccount {
    pub scheme: String,
    pub bucket: String,
    pub region: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageService {
    Local(Box<std::path::PathBuf>),
    Azure(AzureAccount),
    S3(S3Account),
    Google(GoogleAccount),
    Unknown(StorageUrl),
}

impl From<StorageUrl> for StorageService {
    fn from(value: StorageUrl) -> Self {
        // if we already passed through storage url, we know its valid.
        process_uri(value).expect("should be URL safe")
    }
}

impl From<&StorageUrl> for StorageService {
    fn from(value: &StorageUrl) -> Self {
        // if we already passed through storage url, we know its valid.
        process_uri(value).expect("should be URL safe")
    }
}

fn process_uri(url: impl AsRef<Url>) -> Result<StorageService> {
    let uri = url.as_ref();
    match uri.scheme().to_lowercase().as_ref() {
        "file" => Ok(StorageService::Local(Box::new(
            std::path::Path::new(uri.path()).to_owned(),
        ))),
        "az" | "abfs" | "abfss" | "adls2" | "azure" | "wasb" => {
            let host_str = uri
                .host_str()
                .ok_or_else(|| AreaStoreError::Parsing("host required".into()))?;
            let (account, container, is_gen2) =
                if let Some((account, host_suffix)) = host_str.split_once('.') {
                    let container = if uri.username().is_empty() {
                        Err(AreaStoreError::Parsing("unexpected host".into()))
                    } else {
                        Ok(uri.username())
                    }?;
                    match host_suffix {
                        AZURE_ADLS_HOST => Ok((Some(account.into()), container.into(), true)),
                        AZURE_BLOB_HOST => Ok((Some(account.into()), container.into(), false)),
                        _ => Err(AreaStoreError::Parsing("unexpected host".into())),
                    }
                } else {
                    Ok((None, host_str.into(), false))
                }?;

            Ok(StorageService::Azure(AzureAccount {
                scheme: uri.scheme().to_owned(),
                account,
                container,
                is_gen2,
            }))
        }
        "s3" | "s3a" => Ok(StorageService::S3(S3Account {
            scheme: uri.scheme().to_owned(),
            bucket: uri.host_str().unwrap().to_owned(),
            region: None,
        })),
        "gs" => Ok(StorageService::Google(GoogleAccount {
            scheme: uri.scheme().to_owned(),
            bucket: uri.host_str().unwrap().to_owned(),
            region: None,
        })),
        _ => Ok(StorageService::Unknown(
            StorageUrl::parse(uri.as_ref()).expect("should be URL safe"),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_path() {
        let root = std::env::current_dir().unwrap();
        let root = root.to_string_lossy();

        let url = StorageUrl::parse(&root).unwrap();
        let child = url.prefix.child("partition").child("file");

        let prefix: Vec<_> = url.strip_prefix(&child).unwrap().collect();
        assert_eq!(prefix, vec!["partition", "file"]);

        let url = StorageUrl::parse("file:///").unwrap();
        let child = Path::parse("/foo/bar").unwrap();
        let prefix: Vec<_> = url.strip_prefix(&child).unwrap().collect();
        assert_eq!(prefix, vec!["foo", "bar"]);

        let url = StorageUrl::parse("file:///foo").unwrap();
        let child = Path::parse("/foob/bar").unwrap();
        assert!(url.strip_prefix(&child).is_none());
    }

    #[test]
    fn test_prefix_s3() {
        let url = StorageUrl::parse("s3://bucket/foo/bar").unwrap();
        assert_eq!(url.prefix.as_ref(), "foo/bar");

        let path = Path::from("foo/bar/partition/foo.parquet");
        let prefix: Vec<_> = url.strip_prefix(&path).unwrap().collect();
        assert_eq!(prefix, vec!["partition", "foo.parquet"]);

        let path = Path::from("other/bar/partition/foo.parquet");
        assert!(url.strip_prefix(&path).is_none());
    }

    #[test]
    fn test_prefix_asd() {
        let url = StorageUrl::parse("/home/robstar/github/flight-fusion/test/db").unwrap();
        let refurl: &str = url.as_ref();
        println!("{:?}", refurl)
    }

    #[test]
    fn test_parse_services() {
        let expected_pairs = &[
            (
                "abfs://container@account.dfs.core.windows.net/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "abfs".into(),
                    account: Some("account".into()),
                    container: "container".into(),
                    is_gen2: true,
                }),
            ),
            (
                "az://container@account.blob.core.windows.net/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "az".into(),
                    account: Some("account".into()),
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "abfs://container/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "abfs".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "abfss://container/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "abfss".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "wasb://container/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "wasb".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "az://container/path/file",
                StorageService::Azure(AzureAccount {
                    scheme: "az".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
        ];

        for (raw, expected) in expected_pairs.into_iter() {
            let parsed = StorageUrl::parse(raw).unwrap();
            let location = parsed.into();
            assert_eq!(*expected, location);
        }
    }
}

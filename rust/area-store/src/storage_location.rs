use crate::error::{AreaStoreError, Result};
use crate::storage_url::StorageUrl;
use object_store::path::Path;
use std::str::FromStr;
use url::Url;

const AZURE_ADLS_HOST: &str = "dfs.core.windows.net";
const AZURE_BLOB_HOST: &str = "blob.core.windows.net";

pub enum Error {
    Parsing,
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
pub enum StorageLocation {
    Local(Box<std::path::PathBuf>),
    Azure(AzureAccount),
    S3,
    Google,
    Unknown(StorageUrl),
}

fn parse_uri(uri: impl AsRef<str>) -> Result<StorageLocation> {
    match Url::parse(uri.as_ref()) {
        Ok(parsed) => process_uri(&parsed),
        Err(url::ParseError::RelativeUrlWithoutBase) => {
            let local_path = std::path::Path::new(uri.as_ref());
            if !local_path.is_absolute() {
                // passing this through path ensures the path can be canonicalized.
                // if not, we cannot uniquely determine the intended location
                let _ = Path::from_filesystem_path(local_path)
                    .map_err(|err| AreaStoreError::Parsing(err.to_string()))?;
            };
            Ok(StorageLocation::Local(Box::new(local_path.into())))
        }
        Err(err) => Err(AreaStoreError::Parsing(err.to_string())),
    }
}

fn process_uri(uri: &Url) -> Result<StorageLocation> {
    match uri.scheme().to_lowercase().as_ref() {
        "az" | "abfs" | "abfss" | "adls2" | "azure" | "wasb" => {
            let host_str = uri
                .host_str()
                .ok_or(AreaStoreError::Parsing("host required".into()))?;
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

            Ok(StorageLocation::Azure(AzureAccount {
                scheme: uri.scheme().to_owned(),
                account,
                container,
                is_gen2,
            }))
        }
        "s3" | "s3a" => todo!(),
        "gs" => todo!(),
        _ => {
            // Since we did find some base / scheme, but don't recognize it, it
            // may be a local path (i.e. c:/.. on windows). We need to pipe it through path though
            // to get consistent path separators.
            let local_path = std::path::Path::new(uri.as_ref());
            if !local_path.is_absolute() {
                // passing this through path ensures the path can be canonicalized.
                // if not, we cannot uniquely determine the intended location
                let _ = Path::from_filesystem_path(local_path)
                    .map_err(|err| AreaStoreError::Parsing(err.to_string()))?;
            };
            Ok(StorageLocation::Local(Box::new(local_path.to_owned())))
        }
    }
}

impl FromStr for StorageLocation {
    type Err = AreaStoreError;

    fn from_str(s: &str) -> Result<Self> {
        parse_uri(s)
    }
}

impl TryFrom<&str> for StorageLocation {
    type Error = AreaStoreError;

    fn try_from(uri: &str) -> Result<Self> {
        parse_uri(uri)
    }
}

impl TryFrom<StorageUrl> for StorageLocation {
    type Error = AreaStoreError;

    fn try_from(value: StorageUrl) -> Result<Self> {
        parse_uri(&value.url)
    }
}

impl TryFrom<String> for StorageLocation {
    type Error = AreaStoreError;

    fn try_from(uri: String) -> Result<Self> {
        parse_uri(uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_uris() {
        let expected_pairs = &[
            (
                "abfs://container@account.dfs.core.windows.net/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "abfs".into(),
                    account: Some("account".into()),
                    container: "container".into(),
                    is_gen2: true,
                }),
            ),
            (
                "az://container@account.blob.core.windows.net/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "az".into(),
                    account: Some("account".into()),
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "abfs://container/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "abfs".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "abfss://container/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "abfss".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "wasb://container/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "wasb".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
            (
                "az://container/path/file",
                StorageLocation::Azure(AzureAccount {
                    scheme: "az".into(),
                    account: None,
                    container: "container".into(),
                    is_gen2: false,
                }),
            ),
        ];

        for (raw, expected) in expected_pairs.into_iter() {
            let parsed = StorageUrl::parse(raw).unwrap();
            let location = parsed.try_into().unwrap();
            assert_eq!(*expected, location);
        }
    }
}

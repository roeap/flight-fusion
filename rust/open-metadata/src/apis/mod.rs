use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub mod all_collections_in_the_catalog_api;
pub mod bots_collection_api;
pub mod catalog_version_related_operations_api;
pub mod chart_data_asset_collection_api;
pub mod dashboards_collection_api;
pub mod databases_collection_api;
pub mod events_api;
pub mod feeds_collection_api;
pub mod get_configuration_api;
pub mod ingestion_collection_api;
pub mod lineage_resource_api;
pub mod locations_collection_api;
pub mod metrics_collection_api;
pub mod ml_models_collection_api;
pub mod pipelines_collection_api;
pub mod policies_collection_api;
pub mod reports_collection_api;
pub mod search_collection_api;
pub mod services_collection_api;
pub mod services_dashboard_service_collection_api;
pub mod services_database_service_collection_api;
pub mod services_messaging_service_collection_api;
pub mod services_pipeline_service_collection_api;
pub mod services_storage_service_collection_api;
pub mod tables_collection_api;
pub mod tags_resources_collection_api;
pub mod teams_collection_api;
pub mod topic_data_asset_collection_api;
pub mod usage_resource_api;
pub mod user_collection_api;

pub mod configuration;

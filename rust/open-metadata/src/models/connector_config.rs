/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ConnectorConfig {
    /// username to connect  to the data source.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// password to connect  to the data source.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Host and port of the data source.
    #[serde(rename = "host", skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    /// Database of the data source.
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// optional configuration to turn off fetching metadata for views.
    #[serde(rename = "includeViews", skip_serializing_if = "Option::is_none")]
    pub include_views: Option<bool>,
    /// Run data profiler as part of ingestion to get table profile data.
    #[serde(rename = "enableDataProfiler", skip_serializing_if = "Option::is_none")]
    pub enable_data_profiler: Option<bool>,
    /// Regex to only fetch tables or databases that matches the pattern.
    #[serde(
        rename = "includeFilterPattern",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_filter_pattern: Option<Vec<String>>,
    /// Regex exclude tables or databases that matches the pattern.
    #[serde(
        rename = "excludeFilterPattern",
        skip_serializing_if = "Option::is_none"
    )]
    pub exclude_filter_pattern: Option<Vec<String>>,
}

impl ConnectorConfig {
    pub fn new() -> ConnectorConfig {
        ConnectorConfig {
            username: None,
            password: None,
            host: None,
            database: None,
            include_views: None,
            enable_data_profiler: None,
            include_filter_pattern: None,
            exclude_filter_pattern: None,
        }
    }
}

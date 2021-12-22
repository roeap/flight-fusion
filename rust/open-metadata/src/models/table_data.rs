/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TableData {
    /// List of local column names (not fully qualified column names) of the table.
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Data for multiple rows of the table.
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<Vec<serde_json::Value>>>,
}

impl TableData {
    pub fn new() -> TableData {
        TableData {
            columns: None,
            rows: None,
        }
    }
}
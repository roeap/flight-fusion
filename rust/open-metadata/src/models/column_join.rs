/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ColumnJoin {
    /// Local name (not fully qualified name) of the column. ColumnName is `-` when the column is not named in struct dataType. For example, BigQuery supports struct with unnamed fields.
    #[serde(rename = "columnName", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    /// Fully qualified names of the columns that this column is joined with.
    #[serde(rename = "joinedWith", skip_serializing_if = "Option::is_none")]
    pub joined_with: Option<Vec<crate::models::JoinedWith>>,
}

impl ColumnJoin {
    pub fn new() -> ColumnJoin {
        ColumnJoin {
            column_name: None,
            joined_with: None,
        }
    }
}
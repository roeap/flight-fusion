/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Table {
    /// Unique id used to identify an entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Local name (not fully qualified name) of a table.
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name that identifies this table. It could be title or label from the source services.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Fully qualified name of a table in the form `serviceName.databaseName.tableName`.
    #[serde(rename = "fullyQualifiedName", skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    /// Description of a table.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Metadata version of the entity in the form `Major.Minor`. First version always starts from `0.1` when the entity is created. When the backward compatible changes are made to the entity, only the `Minor` version is incremented - example `1.0` is changed to `1.1`. When backward incompatible changes are made the `Major` version is incremented - example `1.1` to `2.0`.
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    /// Date and time in ISO 8601 format. Example - '2018-11-13T20:20:39+00:00'.
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// User who made the update.
    #[serde(rename = "updatedBy", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// URI that points to a resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    /// This schema defines the type used for describing different types of tables.
    #[serde(rename = "tableType", skip_serializing_if = "Option::is_none")]
    pub table_type: Option<TableType>,
    /// Columns in this table.
    #[serde(rename = "columns")]
    pub columns: Vec<crate::models::Column>,
    /// Table constraints.
    #[serde(rename = "tableConstraints", skip_serializing_if = "Option::is_none")]
    pub table_constraints: Option<Vec<crate::models::TableConstraint>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::EntityReference>>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<Box<crate::models::EntityReference>>,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<crate::models::EntityReference>>,
    /// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<Box<crate::models::EntityReference>>,
    /// SQL query statement. Example - 'select * from orders'.
    #[serde(rename = "viewDefinition", skip_serializing_if = "Option::is_none")]
    pub view_definition: Option<String>,
    /// Tags for this table.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::TagLabel>>,
    #[serde(rename = "usageSummary", skip_serializing_if = "Option::is_none")]
    pub usage_summary: Option<Box<crate::models::UsageDetails>>,
    #[serde(rename = "followers", skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<crate::models::EntityReference>>,
    #[serde(rename = "joins", skip_serializing_if = "Option::is_none")]
    pub joins: Option<Box<crate::models::TableJoins>>,
    #[serde(rename = "sampleData", skip_serializing_if = "Option::is_none")]
    pub sample_data: Option<Box<crate::models::TableData>>,
    /// Data profile for a table.
    #[serde(rename = "tableProfile", skip_serializing_if = "Option::is_none")]
    pub table_profile: Option<Vec<crate::models::TableProfile>>,
    /// List of queries that ran against a table.
    #[serde(rename = "tableQueries", skip_serializing_if = "Option::is_none")]
    pub table_queries: Option<Vec<crate::models::SqlQuery>>,
    #[serde(rename = "dataModel", skip_serializing_if = "Option::is_none")]
    pub data_model: Option<Box<crate::models::DataModel>>,
    #[serde(rename = "changeDescription", skip_serializing_if = "Option::is_none")]
    pub change_description: Option<Box<crate::models::ChangeDescription>>,
}

impl Table {
    pub fn new(id: String, name: String, columns: Vec<crate::models::Column>) -> Table {
        Table {
            id,
            name,
            display_name: None,
            fully_qualified_name: None,
            description: None,
            version: None,
            updated_at: None,
            updated_by: None,
            href: None,
            table_type: None,
            columns,
            table_constraints: None,
            owner: None,
            database: None,
            service: None,
            service_type: None,
            location: None,
            view_definition: None,
            tags: None,
            usage_summary: None,
            followers: None,
            joins: None,
            sample_data: None,
            table_profile: None,
            table_queries: None,
            data_model: None,
            change_description: None,
        }
    }
}

/// This schema defines the type used for describing different types of tables.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TableType {
    #[serde(rename = "Regular")]
    Regular,
    #[serde(rename = "External")]
    External,
    #[serde(rename = "View")]
    View,
    #[serde(rename = "SecureView")]
    SecureView,
    #[serde(rename = "MaterializedView")]
    MaterializedView,
}

impl Default for TableType {
    fn default() -> TableType {
        Self::Regular
    }
}
/// Type of database service such as MySQL, BigQuery, Snowflake, Redshift, Postgres...
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(rename = "BigQuery")]
    BigQuery,
    #[serde(rename = "MySQL")]
    MySQL,
    #[serde(rename = "Redshift")]
    Redshift,
    #[serde(rename = "Snowflake")]
    Snowflake,
    #[serde(rename = "Postgres")]
    Postgres,
    #[serde(rename = "MSSQL")]
    MSSQL,
    #[serde(rename = "Oracle")]
    Oracle,
    #[serde(rename = "Athena")]
    Athena,
    #[serde(rename = "Hive")]
    Hive,
    #[serde(rename = "Presto")]
    Presto,
    #[serde(rename = "Trino")]
    Trino,
    #[serde(rename = "Vertica")]
    Vertica,
    #[serde(rename = "Glue")]
    Glue,
    #[serde(rename = "MariaDB")]
    MariaDB,
    #[serde(rename = "Druid")]
    Druid,
}

impl Default for ServiceType {
    fn default() -> ServiceType {
        Self::BigQuery
    }
}
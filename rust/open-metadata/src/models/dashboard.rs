/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dashboard {
    /// Unique id used to identify an entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Name that identifies this dashboard.
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name that identifies this Dashboard. It could be title or label from the source services.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A unique name that identifies a dashboard in the format 'ServiceName.DashboardName'.
    #[serde(rename = "fullyQualifiedName", skip_serializing_if = "Option::is_none")]
    pub fully_qualified_name: Option<String>,
    /// Description of the dashboard, what it is, and how to use it.
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
    /// Dashboard URL.
    #[serde(rename = "dashboardUrl", skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    /// All the charts included in this Dashboard.
    #[serde(rename = "charts", skip_serializing_if = "Option::is_none")]
    pub charts: Option<Vec<crate::models::EntityReference>>,
    /// URI that points to a resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::EntityReference>>,
    #[serde(rename = "followers", skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<crate::models::EntityReference>>,
    /// Tags for this dashboard.
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::TagLabel>>,
    #[serde(rename = "service")]
    pub service: Box<crate::models::EntityReference>,
    /// Type of Dashboard service - Superset, Looker, Redash or Tableau.
    #[serde(rename = "serviceType", skip_serializing_if = "Option::is_none")]
    pub service_type: Option<ServiceType>,
    #[serde(rename = "usageSummary", skip_serializing_if = "Option::is_none")]
    pub usage_summary: Option<Box<crate::models::UsageDetails>>,
    #[serde(rename = "changeDescription", skip_serializing_if = "Option::is_none")]
    pub change_description: Option<Box<crate::models::ChangeDescription>>,
}

impl Dashboard {
    pub fn new(id: String, name: String, service: crate::models::EntityReference) -> Dashboard {
        Dashboard {
            id,
            name,
            display_name: None,
            fully_qualified_name: None,
            description: None,
            version: None,
            updated_at: None,
            updated_by: None,
            dashboard_url: None,
            charts: None,
            href: None,
            owner: None,
            followers: None,
            tags: None,
            service: Box::new(service),
            service_type: None,
            usage_summary: None,
            change_description: None,
        }
    }
}

/// Type of Dashboard service - Superset, Looker, Redash or Tableau.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(rename = "Superset")]
    Superset,
    #[serde(rename = "Looker")]
    Looker,
    #[serde(rename = "Tableau")]
    Tableau,
    #[serde(rename = "Redash")]
    Redash,
    #[serde(rename = "Metabase")]
    Metabase,
}

impl Default for ServiceType {
    fn default() -> ServiceType {
        Self::Superset
    }
}
/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DashboardService {
    /// Unique id used to identify an entity.
    #[serde(rename = "id")]
    pub id: String,
    /// Name that identifies this dashboard service.
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name that identifies this dashboard service.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Type of Dashboard service - Superset, Looker, Redash or Tableau.
    #[serde(rename = "serviceType")]
    pub service_type: ServiceType,
    /// Description of a dashboard service instance.
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
    /// Dashboard Service URL. This will be used to make REST API calls to Dashboard Service.
    #[serde(rename = "dashboardUrl")]
    pub dashboard_url: String,
    /// Username to log-into Dashboard Service.
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password to log-into Dashboard Service.
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "ingestionSchedule", skip_serializing_if = "Option::is_none")]
    pub ingestion_schedule: Option<Box<crate::models::Schedule>>,
    /// URI that points to a resource.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "changeDescription", skip_serializing_if = "Option::is_none")]
    pub change_description: Option<Box<crate::models::ChangeDescription>>,
}

impl DashboardService {
    pub fn new(
        id: String,
        name: String,
        service_type: ServiceType,
        dashboard_url: String,
    ) -> DashboardService {
        DashboardService {
            id,
            name,
            display_name: None,
            service_type,
            description: None,
            version: None,
            updated_at: None,
            updated_by: None,
            dashboard_url,
            username: None,
            password: None,
            ingestion_schedule: None,
            href: None,
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
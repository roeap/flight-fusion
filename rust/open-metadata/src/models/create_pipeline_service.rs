/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePipelineService {
    /// Name that identifies the this entity instance uniquely
    #[serde(rename = "name")]
    pub name: String,
    /// Description of pipeline service entity.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Type of pipeline service - Airflow or Prefect.
    #[serde(rename = "serviceType")]
    pub service_type: ServiceType,
    /// Pipeline UI URL
    #[serde(rename = "pipelineUrl")]
    pub pipeline_url: String,
    #[serde(rename = "ingestionSchedule", skip_serializing_if = "Option::is_none")]
    pub ingestion_schedule: Option<Box<crate::models::Schedule>>,
}

impl CreatePipelineService {
    pub fn new(
        name: String,
        service_type: ServiceType,
        pipeline_url: String,
    ) -> CreatePipelineService {
        CreatePipelineService {
            name,
            description: None,
            service_type,
            pipeline_url,
            ingestion_schedule: None,
        }
    }
}

/// Type of pipeline service - Airflow or Prefect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    #[serde(rename = "Airflow")]
    Airflow,
    #[serde(rename = "Prefect")]
    Prefect,
    #[serde(rename = "Glue")]
    Glue,
}

impl Default for ServiceType {
    fn default() -> ServiceType {
        Self::Airflow
    }
}
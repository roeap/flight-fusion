/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateMlModel {
    /// Name that identifies this ML model.
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name that identifies this ML model. It could be title or label from the source services
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Description of the ML model instance. How it was trained and for what it is used.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Algorithm used to train the ML Model
    #[serde(rename = "algorithm")]
    pub algorithm: String,
    /// Features used to train the ML Model.
    #[serde(rename = "mlFeatures", skip_serializing_if = "Option::is_none")]
    pub ml_features: Option<Vec<crate::models::MlFeature>>,
    /// Hyper Parameters used to train the ML Model.
    #[serde(rename = "mlHyperParameters", skip_serializing_if = "Option::is_none")]
    pub ml_hyper_parameters: Option<Vec<crate::models::MlHyperParameter>>,
    #[serde(rename = "dashboard", skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Box<crate::models::EntityReference>>,
    #[serde(rename = "mlStore", skip_serializing_if = "Option::is_none")]
    pub ml_store: Option<Box<crate::models::MlStore>>,
    /// URI that points to a resource.
    #[serde(rename = "server", skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    /// Tags for this ML Model
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::TagLabel>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::EntityReference>>,
}

impl CreateMlModel {
    pub fn new(name: String, algorithm: String) -> CreateMlModel {
        CreateMlModel {
            name,
            display_name: None,
            description: None,
            algorithm,
            ml_features: None,
            ml_hyper_parameters: None,
            dashboard: None,
            ml_store: None,
            server: None,
            tags: None,
            owner: None,
        }
    }
}
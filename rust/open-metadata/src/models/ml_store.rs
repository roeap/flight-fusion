/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MlStore {
    /// URI that points to a resource.
    #[serde(rename = "storage", skip_serializing_if = "Option::is_none")]
    pub storage: Option<String>,
    /// URI that points to a resource.
    #[serde(rename = "imageRepository", skip_serializing_if = "Option::is_none")]
    pub image_repository: Option<String>,
}

impl MlStore {
    pub fn new() -> MlStore {
        MlStore {
            storage: None,
            image_repository: None,
        }
    }
}

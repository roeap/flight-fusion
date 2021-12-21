/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionInfo {
    /// Unique name that identifies a collection.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of collection.
    #[serde(rename = "documentation", skip_serializing_if = "Option::is_none")]
    pub documentation: Option<String>,
    /// URL of the API endpoint where given collections are available.
    #[serde(rename = "href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Box<crate::models::ImageList>>,
}

impl CollectionInfo {
    pub fn new() -> CollectionInfo {
        CollectionInfo {
            name: None,
            documentation: None,
            href: None,
            images: None,
        }
    }
}

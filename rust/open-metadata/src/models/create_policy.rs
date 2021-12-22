/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreatePolicy {
    /// Name that identifies this Policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Title for this Policy.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// A short description of the Policy, comprehensible to regular users.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::EntityReference>>,
    /// Link to a well documented definition of this Policy.
    #[serde(rename = "policyUrl", skip_serializing_if = "Option::is_none")]
    pub policy_url: Option<String>,
    /// This schema defines the type used for describing different types of policies.
    #[serde(rename = "policyType")]
    pub policy_type: PolicyType,
    /// A set of rules associated with the Policy.
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<serde_json::Value>>,
}

impl CreatePolicy {
    pub fn new(name: String, policy_type: PolicyType) -> CreatePolicy {
        CreatePolicy {
            name,
            display_name: None,
            description: None,
            owner: None,
            policy_url: None,
            policy_type,
            rules: None,
        }
    }
}

/// This schema defines the type used for describing different types of policies.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PolicyType {
    #[serde(rename = "AccessControl")]
    AccessControl,
    #[serde(rename = "Lifecycle")]
    Lifecycle,
}

impl Default for PolicyType {
    fn default() -> PolicyType {
        Self::AccessControl
    }
}

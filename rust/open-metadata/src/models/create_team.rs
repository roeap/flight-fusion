/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateTeam {
    /// A unique name of the team typically the team ID from an identity provider. Example - group Id from LDAP.
    #[serde(rename = "name")]
    pub name: String,
    /// Optional name used for display purposes. Example 'Marketing Team'
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Optional description of the team
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<Box<crate::models::Profile>>,
    /// Optional IDs of users that are part of the team
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

impl CreateTeam {
    pub fn new(name: String) -> CreateTeam {
        CreateTeam {
            name,
            display_name: None,
            description: None,
            profile: None,
            users: None,
        }
    }
}

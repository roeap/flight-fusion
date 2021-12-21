/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Schedule {
    /// Date and time in ISO 8601 format. Example - '2018-11-13T20:20:39+00:00'.
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Duration in ISO 8601 format in UTC. Example - 'P23DT23H'.
    #[serde(rename = "repeatFrequency", skip_serializing_if = "Option::is_none")]
    pub repeat_frequency: Option<String>,
}

impl Schedule {
    pub fn new() -> Schedule {
        Schedule {
            start_date: None,
            repeat_frequency: None,
        }
    }
}

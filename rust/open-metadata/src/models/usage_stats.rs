/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UsageStats {
    /// Usage count of a data asset on the start date.
    #[serde(rename = "count")]
    pub count: i32,
    /// Optional daily percentile rank data asset use when relevant.
    #[serde(rename = "percentileRank", skip_serializing_if = "Option::is_none")]
    pub percentile_rank: Option<f64>,
}

impl UsageStats {
    pub fn new(count: i32) -> UsageStats {
        UsageStats {
            count,
            percentile_rank: None,
        }
    }
}
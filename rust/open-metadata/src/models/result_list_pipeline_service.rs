/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ResultListPipelineService {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::PipelineService>,
    #[serde(rename = "paging", skip_serializing_if = "Option::is_none")]
    pub paging: Option<Box<crate::models::Paging>>,
}

impl ResultListPipelineService {
    pub fn new(data: Vec<crate::models::PipelineService>) -> ResultListPipelineService {
        ResultListPipelineService { data, paging: None }
    }
}

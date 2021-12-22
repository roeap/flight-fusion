/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateChart {
    /// Name that identifies this Chart.
    #[serde(rename = "name")]
    pub name: String,
    /// Display Name that identifies this Chart. It could be title or label from the source services
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Description of the chart instance. What it has and how to use it.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// This schema defines the type used for describing different types of charts.
    #[serde(rename = "chartType", skip_serializing_if = "Option::is_none")]
    pub chart_type: Option<ChartType>,
    /// Chart URL, pointing to its own Service URL
    #[serde(rename = "chartUrl", skip_serializing_if = "Option::is_none")]
    pub chart_url: Option<String>,
    #[serde(rename = "tables", skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<crate::models::EntityReference>>,
    /// Tags for this chart
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::TagLabel>>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<crate::models::EntityReference>>,
    #[serde(rename = "service")]
    pub service: Box<crate::models::EntityReference>,
}

impl CreateChart {
    pub fn new(name: String, service: crate::models::EntityReference) -> CreateChart {
        CreateChart {
            name,
            display_name: None,
            description: None,
            chart_type: None,
            chart_url: None,
            tables: None,
            tags: None,
            owner: None,
            service: Box::new(service),
        }
    }
}

/// This schema defines the type used for describing different types of charts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChartType {
    #[serde(rename = "Line")]
    Line,
    #[serde(rename = "Table")]
    Table,
    #[serde(rename = "Bar")]
    Bar,
    #[serde(rename = "Area")]
    Area,
    #[serde(rename = "Pie")]
    Pie,
    #[serde(rename = "Histogram")]
    Histogram,
    #[serde(rename = "Scatter")]
    Scatter,
    #[serde(rename = "Text")]
    Text,
    #[serde(rename = "BoxPlot")]
    BoxPlot,
    #[serde(rename = "Other")]
    Other,
}

impl Default for ChartType {
    fn default() -> ChartType {
        Self::Line
    }
}

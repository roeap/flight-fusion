/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct JdbcInfo {
    /// Type used for JDBC driver class.
    #[serde(rename = "driverClass")]
    pub driver_class: String,
    /// Type used for JDBC connection URL of format `url_scheme://<username>:<password>@<host>:<port>/<db_name>`.
    #[serde(rename = "connectionUrl")]
    pub connection_url: String,
}

impl JdbcInfo {
    pub fn new(driver_class: String, connection_url: String) -> JdbcInfo {
        JdbcInfo {
            driver_class,
            connection_url,
        }
    }
}

/*
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`create12`]
#[derive(Clone, Debug, Default)]
pub struct Create12Params {
    pub body: Option<crate::models::Report>,
}

/// struct for passing parameters to the method [`create_or_update10`]
#[derive(Clone, Debug, Default)]
pub struct CreateOrUpdate10Params {
    pub body: Option<crate::models::Report>,
}

/// struct for passing parameters to the method [`get14`]
#[derive(Clone, Debug, Default)]
pub struct Get14Params {
    pub id: String,
    pub fields: Option<String>,
}

/// struct for passing parameters to the method [`list12`]
#[derive(Clone, Debug, Default)]
pub struct List12Params {
    pub fields: Option<String>,
}

/// struct for typed errors of method [`create12`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Create12Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_or_update10`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdate10Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get14`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Get14Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list12`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum List12Error {
    UnknownValue(serde_json::Value),
}

pub async fn create12(
    configuration: &configuration::Configuration,
    params: Create12Params,
) -> Result<(), Error<Create12Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/reports", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<Create12Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_or_update10(
    configuration: &configuration::Configuration,
    params: CreateOrUpdate10Params,
) -> Result<(), Error<CreateOrUpdate10Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/reports", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<CreateOrUpdate10Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get14(
    configuration: &configuration::Configuration,
    params: Get14Params,
) -> Result<crate::models::Report, Error<Get14Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/reports/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<Get14Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list12(
    configuration: &configuration::Configuration,
    params: List12Params,
) -> Result<crate::models::ResultListReport, Error<List12Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/reports", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<List12Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
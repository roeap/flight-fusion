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

/// struct for passing parameters to the method [`add_follower4`]
#[derive(Clone, Debug, Default)]
pub struct AddFollower4Params {
    pub id: String,
    pub body: Option<String>,
}

/// struct for passing parameters to the method [`create8`]
#[derive(Clone, Debug, Default)]
pub struct Create8Params {
    pub body: Option<crate::models::CreateMlModel>,
}

/// struct for passing parameters to the method [`create_or_update6`]
#[derive(Clone, Debug, Default)]
pub struct CreateOrUpdate6Params {
    pub body: Option<crate::models::CreateMlModel>,
}

/// struct for passing parameters to the method [`delete5`]
#[derive(Clone, Debug, Default)]
pub struct Delete5Params {
    pub id: String,
}

/// struct for passing parameters to the method [`delete_follower4`]
#[derive(Clone, Debug, Default)]
pub struct DeleteFollower4Params {
    pub id: String,
    pub user_id: String,
}

/// struct for passing parameters to the method [`get10`]
#[derive(Clone, Debug, Default)]
pub struct Get10Params {
    pub id: String,
    pub fields: Option<String>,
}

/// struct for passing parameters to the method [`get_by_name6`]
#[derive(Clone, Debug, Default)]
pub struct GetByName6Params {
    pub fqn: String,
    pub fields: Option<String>,
}

/// struct for passing parameters to the method [`get_version5`]
#[derive(Clone, Debug, Default)]
pub struct GetVersion5Params {
    pub id: String,
    pub version: String,
}

/// struct for passing parameters to the method [`list8`]
#[derive(Clone, Debug, Default)]
pub struct List8Params {
    pub fields: Option<String>,
    pub limit: Option<i32>,
    pub before: Option<String>,
    pub after: Option<String>,
}

/// struct for passing parameters to the method [`list_versions5`]
#[derive(Clone, Debug, Default)]
pub struct ListVersions5Params {
    pub id: String,
}

/// struct for passing parameters to the method [`patch2`]
#[derive(Clone, Debug, Default)]
pub struct Patch2Params {
    pub id: String,
    pub body: Option<serde_json::Value>,
}

/// struct for typed errors of method [`add_follower4`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddFollower4Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create8`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Create8Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_or_update6`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateOrUpdate6Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete5`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Delete5Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_follower4`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFollower4Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get10`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Get10Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_by_name6`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetByName6Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_version5`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetVersion5Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list8`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum List8Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_versions5`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVersions5Error {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`patch2`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Patch2Error {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

pub async fn add_follower4(
    configuration: &configuration::Configuration,
    params: AddFollower4Params,
) -> Result<(), Error<AddFollower4Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}/followers",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
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
        let local_var_entity: Option<AddFollower4Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create8(
    configuration: &configuration::Configuration,
    params: Create8Params,
) -> Result<(), Error<Create8Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/mlmodels", local_var_configuration.base_path);
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
        let local_var_entity: Option<Create8Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_or_update6(
    configuration: &configuration::Configuration,
    params: CreateOrUpdate6Params,
) -> Result<(), Error<CreateOrUpdate6Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/mlmodels", local_var_configuration.base_path);
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
        let local_var_entity: Option<CreateOrUpdate6Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete5(
    configuration: &configuration::Configuration,
    params: Delete5Params,
) -> Result<(), Error<Delete5Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<Delete5Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn delete_follower4(
    configuration: &configuration::Configuration,
    params: DeleteFollower4Params,
) -> Result<(), Error<DeleteFollower4Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let user_id = params.user_id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}/followers/{userId}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        userId = crate::apis::urlencode(user_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<DeleteFollower4Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get10(
    configuration: &configuration::Configuration,
    params: Get10Params,
) -> Result<crate::models::MlModel, Error<Get10Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}",
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
        let local_var_entity: Option<Get10Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_by_name6(
    configuration: &configuration::Configuration,
    params: GetByName6Params,
) -> Result<crate::models::MlModel, Error<GetByName6Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fqn = params.fqn;
    let fields = params.fields;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/name/{fqn}",
        local_var_configuration.base_path,
        fqn = crate::apis::urlencode(fqn)
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
        let local_var_entity: Option<GetByName6Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_version5(
    configuration: &configuration::Configuration,
    params: GetVersion5Params,
) -> Result<crate::models::MlModel, Error<GetVersion5Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let version = params.version;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}/versions/{version}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id),
        version = crate::apis::urlencode(version)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetVersion5Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list8(
    configuration: &configuration::Configuration,
    params: List8Params,
) -> Result<crate::models::ResultListMlModel, Error<List8Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let fields = params.fields;
    let limit = params.limit;
    let before = params.before;
    let after = params.after;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/mlmodels", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = fields {
        local_var_req_builder =
            local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder =
            local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = before {
        local_var_req_builder =
            local_var_req_builder.query(&[("before", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = after {
        local_var_req_builder =
            local_var_req_builder.query(&[("after", &local_var_str.to_string())]);
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
        let local_var_entity: Option<List8Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_versions5(
    configuration: &configuration::Configuration,
    params: ListVersions5Params,
) -> Result<crate::models::EntityHistory, Error<ListVersions5Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}/versions",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<ListVersions5Error> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn patch2(
    configuration: &configuration::Configuration,
    params: Patch2Params,
) -> Result<(), Error<Patch2Error>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let body = params.body;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/mlmodels/{id}",
        local_var_configuration.base_path,
        id = crate::apis::urlencode(id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

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
        let local_var_entity: Option<Patch2Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}
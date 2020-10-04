/*
 * Slack Web API
 *
 * One way to interact with the Slack platform is its HTTP RPC-based Web API, a collection of methods requiring OAuth 2.0-based user, bot, or workspace tokens blessed with related OAuth scopes.
 *
 * The version of the OpenAPI document: 1.5.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `stars_add`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarsAddError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stars_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarsListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `stars_remove`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StarsRemoveError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Adds a star to an item.
pub async fn stars_add(configuration: &configuration::Configuration, token: &str, file_comment: Option<&str>, timestamp: Option<&str>, file: Option<&str>, channel: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<StarsAddError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/stars.add", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = file_comment {
        local_var_form_params.insert("file_comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp {
        local_var_form_params.insert("timestamp", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = file {
        local_var_form_params.insert("file", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = channel {
        local_var_form_params.insert("channel", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StarsAddError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Lists stars for a user.
pub async fn stars_list(configuration: &configuration::Configuration, count: Option<&str>, cursor: Option<&str>, token: Option<&str>, limit: Option<i32>, page: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<StarsListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/stars.list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = count {
        local_var_req_builder = local_var_req_builder.query(&[("count", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = token {
        local_var_req_builder = local_var_req_builder.query(&[("token", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StarsListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes a star from an item.
pub async fn stars_remove(configuration: &configuration::Configuration, token: Option<&str>, file_comment: Option<&str>, timestamp: Option<f32>, channel: Option<&str>, file: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<StarsRemoveError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/stars.remove", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = token {
        local_var_req_builder = local_var_req_builder.header("token", local_var_param_value.to_string());
    }
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = file_comment {
        local_var_form_params.insert("file_comment", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp {
        local_var_form_params.insert("timestamp", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = channel {
        local_var_form_params.insert("channel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = file {
        local_var_form_params.insert("file", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<StarsRemoveError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


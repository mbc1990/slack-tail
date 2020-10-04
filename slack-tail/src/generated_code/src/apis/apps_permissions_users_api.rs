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


/// struct for typed errors of method `apps_permissions_users_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsPermissionsUsersListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `apps_permissions_users_request`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AppsPermissionsUsersRequestError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Returns list of user grants and corresponding scopes this app has on a team.
pub async fn apps_permissions_users_list(configuration: &configuration::Configuration, token: &str, cursor: Option<&str>, limit: Option<i32>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AppsPermissionsUsersListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apps.permissions.users.list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
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
        let local_var_entity: Option<AppsPermissionsUsersListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Enables an app to trigger a permissions modal to grant an app access to a user access scope.
pub async fn apps_permissions_users_request(configuration: &configuration::Configuration, scopes: &str, token: &str, user: &str, trigger_id: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<AppsPermissionsUsersRequestError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/apps.permissions.users.request", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("scopes", &scopes.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("user", &user.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("trigger_id", &trigger_id.to_string())]);
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
        let local_var_entity: Option<AppsPermissionsUsersRequestError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


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


/// struct for typed errors of method `chat_delete`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatDeleteError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_delete_scheduled_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatDeleteScheduledMessageError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_get_permalink`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatGetPermalinkError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_me_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatMeMessageError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_post_ephemeral`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatPostEphemeralError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_post_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatPostMessageError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_schedule_message`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatScheduleMessageError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_scheduled_messages_list`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatScheduledMessagesListError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_unfurl`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatUnfurlError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `chat_update`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatUpdateError {
    DefaultResponse(::std::collections::HashMap<String, serde_json::Value>),
    UnknownValue(serde_json::Value),
}


/// Deletes a message.
pub async fn chat_delete(configuration: &configuration::Configuration, token: Option<&str>, as_user: Option<bool>, ts: Option<f64>, channel: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatDeleteError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.delete", configuration.base_path);
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
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = ts {
        local_var_form_params.insert("ts", local_var_param_value.to_string());
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
        let local_var_entity: Option<ChatDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a pending scheduled message from the queue.
pub async fn chat_delete_scheduled_message(configuration: &configuration::Configuration, token: &str, channel: &str, scheduled_message_id: &str, as_user: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatDeleteScheduledMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.deleteScheduledMessage", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel", channel.to_string());
    local_var_form_params.insert("scheduled_message_id", scheduled_message_id.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatDeleteScheduledMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve a permalink URL for a specific extant message
pub async fn chat_get_permalink(configuration: &configuration::Configuration, token: &str, message_ts: &str, channel: &str) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatGetPermalinkError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.getPermalink", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("token", &token.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("message_ts", &message_ts.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("channel", &channel.to_string())]);
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
        let local_var_entity: Option<ChatGetPermalinkError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Share a me message into a channel.
pub async fn chat_me_message(configuration: &configuration::Configuration, token: Option<&str>, text: Option<&str>, channel: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatMeMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.meMessage", configuration.base_path);
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
    if let Some(local_var_param_value) = text {
        local_var_form_params.insert("text", local_var_param_value.to_string());
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
        let local_var_entity: Option<ChatMeMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sends an ephemeral message to a user in a channel.
pub async fn chat_post_ephemeral(configuration: &configuration::Configuration, token: &str, user: &str, channel: &str, username: Option<&str>, thread_ts: Option<&str>, blocks: Option<&str>, attachments: Option<&str>, as_user: Option<bool>, link_names: Option<bool>, parse: Option<&str>, text: Option<&str>, icon_emoji: Option<&str>, icon_url: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatPostEphemeralError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.postEphemeral", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = username {
        local_var_form_params.insert("username", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = thread_ts {
        local_var_form_params.insert("thread_ts", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = blocks {
        local_var_form_params.insert("blocks", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = attachments {
        local_var_form_params.insert("attachments", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = link_names {
        local_var_form_params.insert("link_names", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = parse {
        local_var_form_params.insert("parse", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = text {
        local_var_form_params.insert("text", local_var_param_value.to_string());
    }
    local_var_form_params.insert("user", user.to_string());
    if let Some(local_var_param_value) = icon_emoji {
        local_var_form_params.insert("icon_emoji", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = icon_url {
        local_var_form_params.insert("icon_url", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel", channel.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatPostEphemeralError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sends a message to a channel.
pub async fn chat_post_message(configuration: &configuration::Configuration, token: &str, channel: &str, attachments: Option<&str>, unfurl_links: Option<bool>, text: Option<&str>, unfurl_media: Option<bool>, parse: Option<&str>, as_user: Option<&str>, mrkdwn: Option<bool>, blocks: Option<&str>, icon_emoji: Option<&str>, link_names: Option<bool>, reply_broadcast: Option<bool>, thread_ts: Option<&str>, username: Option<&str>, icon_url: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatPostMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.postMessage", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = attachments {
        local_var_form_params.insert("attachments", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = unfurl_links {
        local_var_form_params.insert("unfurl_links", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = text {
        local_var_form_params.insert("text", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = unfurl_media {
        local_var_form_params.insert("unfurl_media", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = parse {
        local_var_form_params.insert("parse", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = mrkdwn {
        local_var_form_params.insert("mrkdwn", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel", channel.to_string());
    if let Some(local_var_param_value) = blocks {
        local_var_form_params.insert("blocks", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = icon_emoji {
        local_var_form_params.insert("icon_emoji", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = link_names {
        local_var_form_params.insert("link_names", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = reply_broadcast {
        local_var_form_params.insert("reply_broadcast", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = thread_ts {
        local_var_form_params.insert("thread_ts", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = username {
        local_var_form_params.insert("username", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = icon_url {
        local_var_form_params.insert("icon_url", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatPostMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Schedules a message to be sent to a channel.
pub async fn chat_schedule_message(configuration: &configuration::Configuration, token: Option<&str>, thread_ts: Option<f64>, blocks: Option<&str>, attachments: Option<&str>, unfurl_links: Option<bool>, text: Option<&str>, link_names: Option<bool>, unfurl_media: Option<bool>, parse: Option<&str>, as_user: Option<bool>, post_at: Option<&str>, channel: Option<&str>, reply_broadcast: Option<bool>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatScheduleMessageError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.scheduleMessage", configuration.base_path);
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
    if let Some(local_var_param_value) = thread_ts {
        local_var_form_params.insert("thread_ts", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = blocks {
        local_var_form_params.insert("blocks", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = attachments {
        local_var_form_params.insert("attachments", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = unfurl_links {
        local_var_form_params.insert("unfurl_links", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = text {
        local_var_form_params.insert("text", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = link_names {
        local_var_form_params.insert("link_names", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = unfurl_media {
        local_var_form_params.insert("unfurl_media", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = parse {
        local_var_form_params.insert("parse", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = post_at {
        local_var_form_params.insert("post_at", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = channel {
        local_var_form_params.insert("channel", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = reply_broadcast {
        local_var_form_params.insert("reply_broadcast", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatScheduleMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of scheduled messages.
pub async fn chat_scheduled_messages_list(configuration: &configuration::Configuration, cursor: Option<&str>, token: Option<&str>, limit: Option<i32>, oldest: Option<f64>, channel: Option<&str>, latest: Option<f64>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatScheduledMessagesListError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.scheduledMessages.list", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    if let Some(ref local_var_str) = cursor {
        local_var_req_builder = local_var_req_builder.query(&[("cursor", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = oldest {
        local_var_req_builder = local_var_req_builder.query(&[("oldest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = channel {
        local_var_req_builder = local_var_req_builder.query(&[("channel", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = latest {
        local_var_req_builder = local_var_req_builder.query(&[("latest", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = token {
        local_var_req_builder = local_var_req_builder.header("token", local_var_param_value.to_string());
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
        let local_var_entity: Option<ChatScheduledMessagesListError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Provide custom unfurl behavior for user-posted URLs
pub async fn chat_unfurl(configuration: &configuration::Configuration, token: &str, ts: &str, channel: &str, user_auth_message: Option<&str>, user_auth_required: Option<bool>, unfurls: Option<&str>, user_auth_url: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatUnfurlError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.unfurl", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = user_auth_message {
        local_var_form_params.insert("user_auth_message", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = user_auth_required {
        local_var_form_params.insert("user_auth_required", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = unfurls {
        local_var_form_params.insert("unfurls", local_var_param_value.to_string());
    }
    local_var_form_params.insert("ts", ts.to_string());
    if let Some(local_var_param_value) = user_auth_url {
        local_var_form_params.insert("user_auth_url", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel", channel.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatUnfurlError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Updates a message.
pub async fn chat_update(configuration: &configuration::Configuration, token: &str, ts: &str, channel: &str, blocks: Option<&str>, attachments: Option<&str>, as_user: Option<&str>, parse: Option<&str>, text: Option<&str>, link_names: Option<&str>) -> Result<::std::collections::HashMap<String, serde_json::Value>, Error<ChatUpdateError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/chat.update", configuration.base_path);
    let mut local_var_req_builder = local_var_client.post(local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("token", token.to_string());
    if let Some(ref local_var_token) = configuration.oauth_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = blocks {
        local_var_form_params.insert("blocks", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = attachments {
        local_var_form_params.insert("attachments", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = as_user {
        local_var_form_params.insert("as_user", local_var_param_value.to_string());
    }
    local_var_form_params.insert("ts", ts.to_string());
    if let Some(local_var_param_value) = parse {
        local_var_form_params.insert("parse", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = text {
        local_var_form_params.insert("text", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = link_names {
        local_var_form_params.insert("link_names", local_var_param_value.to_string());
    }
    local_var_form_params.insert("channel", channel.to_string());
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ChatUpdateError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


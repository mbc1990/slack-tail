# \ChatApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_delete**](ChatApi.md#chat_delete) | **Post** /chat.delete | 
[**chat_delete_scheduled_message**](ChatApi.md#chat_delete_scheduled_message) | **Post** /chat.deleteScheduledMessage | 
[**chat_get_permalink**](ChatApi.md#chat_get_permalink) | **Get** /chat.getPermalink | 
[**chat_me_message**](ChatApi.md#chat_me_message) | **Post** /chat.meMessage | 
[**chat_post_ephemeral**](ChatApi.md#chat_post_ephemeral) | **Post** /chat.postEphemeral | 
[**chat_post_message**](ChatApi.md#chat_post_message) | **Post** /chat.postMessage | 
[**chat_schedule_message**](ChatApi.md#chat_schedule_message) | **Post** /chat.scheduleMessage | 
[**chat_scheduled_messages_list**](ChatApi.md#chat_scheduled_messages_list) | **Get** /chat.scheduledMessages.list | 
[**chat_unfurl**](ChatApi.md#chat_unfurl) | **Post** /chat.unfurl | 
[**chat_update**](ChatApi.md#chat_update) | **Post** /chat.update | 


# **chat_delete**
> ::std::collections::HashMap<String, Value> chat_delete(ctx, optional)


Deletes a message.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **as_user** | **bool**| Pass true to delete the message as the authed user with &#x60;chat:write:user&#x60; scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with &#x60;chat:write:bot&#x60; scope. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **ts** | **f32**| Timestamp of the message to be deleted. | 
 **channel** | **String**| Channel containing the message to be deleted. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_delete_scheduled_message**
> ::std::collections::HashMap<String, Value> chat_delete_scheduled_message(ctx, token, channel, scheduled_message_id, optional)


Deletes a pending scheduled message from the queue.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
  **channel** | **String**| The channel the scheduled_message is posting to | 
  **scheduled_message_id** | **String**| &#x60;scheduled_message_id&#x60; returned from call to chat.scheduleMessage | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **channel** | **String**| The channel the scheduled_message is posting to | 
 **scheduled_message_id** | **String**| &#x60;scheduled_message_id&#x60; returned from call to chat.scheduleMessage | 
 **as_user** | **bool**| Pass true to delete the message as the authed user with &#x60;chat:write:user&#x60; scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with &#x60;chat:write:bot&#x60; scope. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_get_permalink**
> ::std::collections::HashMap<String, Value> chat_get_permalink(ctx, token, message_ts, channel)


Retrieve a permalink URL for a specific extant message

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **message_ts** | **String**| A message&#39;s &#x60;ts&#x60; value, uniquely identifying it within a channel | 
  **channel** | **String**| The ID of the conversation or channel containing the message | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_me_message**
> ::std::collections::HashMap<String, Value> chat_me_message(ctx, optional)


Share a me message into a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **text** | **String**| Text of the message to send. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write:user&#x60; | 
 **channel** | **String**| Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_post_ephemeral**
> ::std::collections::HashMap<String, Value> chat_post_ephemeral(ctx, token, user, channel, optional)


Sends an ephemeral message to a user in a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
  **user** | **String**| &#x60;id&#x60; of the user who will receive the ephemeral message. The user should be in the channel specified by the &#x60;channel&#x60; argument. | 
  **channel** | **String**| Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **user** | **String**| &#x60;id&#x60; of the user who will receive the ephemeral message. The user should be in the channel specified by the &#x60;channel&#x60; argument. | 
 **channel** | **String**| Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. | 
 **username** | **String**| Set your bot&#39;s user name. Must be used in conjunction with &#x60;as_user&#x60; set to false, otherwise ignored. See [authorship](#authorship) below. | 
 **thread_ts** | **String**| Provide another message&#39;s &#x60;ts&#x60; value to post this message in a thread. Avoid using a reply&#39;s &#x60;ts&#x60; value; use its parent&#39;s value instead. Ephemeral messages in threads are only shown if there is already an active thread. | 
 **blocks** | **String**| A JSON-based array of structured blocks, presented as a URL-encoded string. | 
 **attachments** | **String**| A JSON-based array of structured attachments, presented as a URL-encoded string. | 
 **as_user** | **bool**| Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false. | 
 **link_names** | **bool**| Find and link channel names and usernames. | 
 **parse** | **String**| Change how messages are treated. Defaults to &#x60;none&#x60;. See [below](#formatting). | 
 **text** | **String**| How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. | 
 **icon_emoji** | **String**| Emoji to use as the icon for this message. Overrides &#x60;icon_url&#x60;. Must be used in conjunction with &#x60;as_user&#x60; set to &#x60;false&#x60;, otherwise ignored. See [authorship](#authorship) below. | 
 **icon_url** | **String**| URL to an image to use as the icon for this message. Must be used in conjunction with &#x60;as_user&#x60; set to false, otherwise ignored. See [authorship](#authorship) below. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_post_message**
> ::std::collections::HashMap<String, Value> chat_post_message(ctx, channel, token, optional)


Sends a message to a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **channel** | **String**| Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channel** | **String**| Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **attachments** | **String**| A JSON-based array of structured attachments, presented as a URL-encoded string. | 
 **unfurl_links** | **bool**| Pass true to enable unfurling of primarily text-based content. | 
 **text** | **String**| How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. | 
 **unfurl_media** | **bool**| Pass false to disable unfurling of media content. | 
 **parse** | **String**| Change how messages are treated. Defaults to &#x60;none&#x60;. See [below](#formatting). | 
 **as_user** | **String**| Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below. | 
 **mrkdwn** | **bool**| Disable Slack markup parsing by setting to &#x60;false&#x60;. Enabled by default. | 
 **blocks** | **String**| A JSON-based array of structured blocks, presented as a URL-encoded string. | 
 **icon_emoji** | **String**| Emoji to use as the icon for this message. Overrides &#x60;icon_url&#x60;. Must be used in conjunction with &#x60;as_user&#x60; set to &#x60;false&#x60;, otherwise ignored. See [authorship](#authorship) below. | 
 **link_names** | **bool**| Find and link channel names and usernames. | 
 **reply_broadcast** | **bool**| Used in conjunction with &#x60;thread_ts&#x60; and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to &#x60;false&#x60;. | 
 **thread_ts** | **String**| Provide another message&#39;s &#x60;ts&#x60; value to make this message a reply. Avoid using a reply&#39;s &#x60;ts&#x60; value; use its parent instead. | 
 **username** | **String**| Set your bot&#39;s user name. Must be used in conjunction with &#x60;as_user&#x60; set to false, otherwise ignored. See [authorship](#authorship) below. | 
 **icon_url** | **String**| URL to an image to use as the icon for this message. Must be used in conjunction with &#x60;as_user&#x60; set to false, otherwise ignored. See [authorship](#authorship) below. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_schedule_message**
> ::std::collections::HashMap<String, Value> chat_schedule_message(ctx, optional)


Schedules a message to be sent to a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **thread_ts** | **f32**| Provide another message&#39;s &#x60;ts&#x60; value to make this message a reply. Avoid using a reply&#39;s &#x60;ts&#x60; value; use its parent instead. | 
 **blocks** | **String**| A JSON-based array of structured blocks, presented as a URL-encoded string. | 
 **attachments** | **String**| A JSON-based array of structured attachments, presented as a URL-encoded string. | 
 **unfurl_links** | **bool**| Pass true to enable unfurling of primarily text-based content. | 
 **text** | **String**| How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. | 
 **link_names** | **bool**| Find and link channel names and usernames. | 
 **unfurl_media** | **bool**| Pass false to disable unfurling of media content. | 
 **parse** | **String**| Change how messages are treated. Defaults to &#x60;none&#x60;. See [chat.postMessage](chat.postMessage#formatting). | 
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **as_user** | **bool**| Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship). | 
 **post_at** | **String**| Unix EPOCH timestamp of time in future to send the message. | 
 **channel** | **String**| Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. | 
 **reply_broadcast** | **bool**| Used in conjunction with &#x60;thread_ts&#x60; and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to &#x60;false&#x60;. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_scheduled_messages_list**
> ::std::collections::HashMap<String, Value> chat_scheduled_messages_list(ctx, optional)


Returns a list of scheduled messages.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cursor** | **String**| For pagination purposes, this is the &#x60;cursor&#x60; value returned from a previous call to &#x60;chat.scheduledmessages.list&#x60; indicating where you want to start this call from. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **limit** | **i32**| Maximum number of original entries to return. | 
 **oldest** | **f32**| A UNIX timestamp of the oldest value in the time range | 
 **channel** | **String**| The channel of the scheduled messages | 
 **latest** | **f32**| A UNIX timestamp of the latest value in the time range | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_unfurl**
> ::std::collections::HashMap<String, Value> chat_unfurl(ctx, ts, token, channel, optional)


Provide custom unfurl behavior for user-posted URLs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ts** | **String**| Timestamp of the message to add unfurl behavior to. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;links:write&#x60; | 
  **channel** | **String**| Channel ID of the message | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ts** | **String**| Timestamp of the message to add unfurl behavior to. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;links:write&#x60; | 
 **channel** | **String**| Channel ID of the message | 
 **user_auth_message** | **String**| Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior | 
 **user_auth_required** | **bool**| Set to &#x60;true&#x60; or &#x60;1&#x60; to indicate the user must install your Slack app to trigger unfurls for this domain | 
 **unfurls** | **String**| URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments. | 
 **user_auth_url** | **String**| Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **chat_update**
> ::std::collections::HashMap<String, Value> chat_update(ctx, ts, token, channel, optional)


Updates a message.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ts** | **String**| Timestamp of the message to be updated. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
  **channel** | **String**| Channel containing the message to be updated. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ts** | **String**| Timestamp of the message to be updated. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;chat:write&#x60; | 
 **channel** | **String**| Channel containing the message to be updated. | 
 **blocks** | **String**| A JSON-based array of structured blocks, presented as a URL-encoded string. | 
 **attachments** | **String**| A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting &#x60;text&#x60;. | 
 **as_user** | **String**| Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users. | 
 **parse** | **String**| Change how messages are treated. Defaults to &#x60;client&#x60;, unlike &#x60;chat.postMessage&#x60;. Accepts either &#x60;none&#x60; or &#x60;full&#x60;. See [below](#formatting). | 
 **text** | **String**| New text for the message, using the [default formatting rules](/docs/formatting). It&#39;s not required when presenting &#x60;attachments&#x60;. | 
 **link_names** | **String**| Find and link channel names and usernames. Defaults to &#x60;none&#x60;. See [below](#formatting). | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


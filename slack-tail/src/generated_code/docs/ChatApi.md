# \ChatApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_delete**](ChatApi.md#chat_delete) | **post** /chat.delete | 
[**chat_delete_scheduled_message**](ChatApi.md#chat_delete_scheduled_message) | **post** /chat.deleteScheduledMessage | 
[**chat_get_permalink**](ChatApi.md#chat_get_permalink) | **get** /chat.getPermalink | 
[**chat_me_message**](ChatApi.md#chat_me_message) | **post** /chat.meMessage | 
[**chat_post_ephemeral**](ChatApi.md#chat_post_ephemeral) | **post** /chat.postEphemeral | 
[**chat_post_message**](ChatApi.md#chat_post_message) | **post** /chat.postMessage | 
[**chat_schedule_message**](ChatApi.md#chat_schedule_message) | **post** /chat.scheduleMessage | 
[**chat_scheduled_messages_list**](ChatApi.md#chat_scheduled_messages_list) | **get** /chat.scheduledMessages.list | 
[**chat_unfurl**](ChatApi.md#chat_unfurl) | **post** /chat.unfurl | 
[**chat_update**](ChatApi.md#chat_update) | **post** /chat.update | 



## chat_delete

> ::std::collections::HashMap<String, serde_json::Value> chat_delete(token, as_user, ts, channel)


Deletes a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write` |  |
**as_user** | Option<**bool**> | Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope. |  |
**ts** | Option<**f32**> | Timestamp of the message to be deleted. |  |
**channel** | Option<**String**> | Channel containing the message to be deleted. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_delete_scheduled_message

> ::std::collections::HashMap<String, serde_json::Value> chat_delete_scheduled_message(token, channel, scheduled_message_id, as_user)


Deletes a pending scheduled message from the queue.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | The channel the scheduled_message is posting to | [required] |
**scheduled_message_id** | **String** | `scheduled_message_id` returned from call to chat.scheduleMessage | [required] |
**as_user** | Option<**bool**> | Pass true to delete the message as the authed user with `chat:write:user` scope. [Bot users](/bot-users) in this context are considered authed users. If unused or false, the message will be deleted with `chat:write:bot` scope. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_get_permalink

> ::std::collections::HashMap<String, serde_json::Value> chat_get_permalink(token, message_ts, channel)


Retrieve a permalink URL for a specific extant message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**message_ts** | **String** | A message's `ts` value, uniquely identifying it within a channel | [required] |
**channel** | **String** | The ID of the conversation or channel containing the message | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_me_message

> ::std::collections::HashMap<String, serde_json::Value> chat_me_message(token, text, channel)


Share a me message into a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write:user` |  |
**text** | Option<**String**> | Text of the message to send. |  |
**channel** | Option<**String**> | Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_post_ephemeral

> ::std::collections::HashMap<String, serde_json::Value> chat_post_ephemeral(token, user, channel, username, thread_ts, blocks, attachments, as_user, link_names, parse, text, icon_emoji, icon_url)


Sends an ephemeral message to a user in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**user** | **String** | `id` of the user who will receive the ephemeral message. The user should be in the channel specified by the `channel` argument. | [required] |
**channel** | **String** | Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. | [required] |
**username** | Option<**String**> | Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |
**thread_ts** | Option<**String**> | Provide another message's `ts` value to post this message in a thread. Avoid using a reply's `ts` value; use its parent's value instead. Ephemeral messages in threads are only shown if there is already an active thread. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**as_user** | Option<**bool**> | Pass true to post the message as the authed user. Defaults to true if the chat:write:bot scope is not included. Otherwise, defaults to false. |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [below](#formatting). |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**icon_emoji** | Option<**String**> | Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below. |  |
**icon_url** | Option<**String**> | URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_post_message

> ::std::collections::HashMap<String, serde_json::Value> chat_post_message(token, channel, attachments, unfurl_links, text, unfurl_media, parse, as_user, mrkdwn, blocks, icon_emoji, link_names, reply_broadcast, thread_ts, username, icon_url)


Sends a message to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**channel** | **String** | Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. | [required] |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**unfurl_links** | Option<**bool**> | Pass true to enable unfurling of primarily text-based content. |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**unfurl_media** | Option<**bool**> | Pass false to disable unfurling of media content. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [below](#formatting). |  |
**as_user** | Option<**String**> | Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [authorship](#authorship) below. |  |
**mrkdwn** | Option<**bool**> | Disable Slack markup parsing by setting to `false`. Enabled by default. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**icon_emoji** | Option<**String**> | Emoji to use as the icon for this message. Overrides `icon_url`. Must be used in conjunction with `as_user` set to `false`, otherwise ignored. See [authorship](#authorship) below. |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**reply_broadcast** | Option<**bool**> | Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`. |  |
**thread_ts** | Option<**String**> | Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead. |  |
**username** | Option<**String**> | Set your bot's user name. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |
**icon_url** | Option<**String**> | URL to an image to use as the icon for this message. Must be used in conjunction with `as_user` set to false, otherwise ignored. See [authorship](#authorship) below. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_schedule_message

> ::std::collections::HashMap<String, serde_json::Value> chat_schedule_message(token, thread_ts, blocks, attachments, unfurl_links, text, link_names, unfurl_media, parse, as_user, post_at, channel, reply_broadcast)


Schedules a message to be sent to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `chat:write` |  |
**thread_ts** | Option<**f32**> | Provide another message's `ts` value to make this message a reply. Avoid using a reply's `ts` value; use its parent instead. |  |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. |  |
**unfurl_links** | Option<**bool**> | Pass true to enable unfurling of primarily text-based content. |  |
**text** | Option<**String**> | How this field works and whether it is required depends on other fields you use in your API call. [See below](#text_usage) for more detail. |  |
**link_names** | Option<**bool**> | Find and link channel names and usernames. |  |
**unfurl_media** | Option<**bool**> | Pass false to disable unfurling of media content. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `none`. See [chat.postMessage](chat.postMessage#formatting). |  |
**as_user** | Option<**bool**> | Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See [chat.postMessage](chat.postMessage#authorship). |  |
**post_at** | Option<**String**> | Unix EPOCH timestamp of time in future to send the message. |  |
**channel** | Option<**String**> | Channel, private group, or DM channel to send message to. Can be an encoded ID, or a name. See [below](#channels) for more details. |  |
**reply_broadcast** | Option<**bool**> | Used in conjunction with `thread_ts` and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to `false`. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_scheduled_messages_list

> ::std::collections::HashMap<String, serde_json::Value> chat_scheduled_messages_list(cursor, token, limit, oldest, channel, latest)


Returns a list of scheduled messages.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**limit** | Option<**i32**> | Maximum number of original entries to return. |  |
**oldest** | Option<**f32**> | A UNIX timestamp of the oldest value in the time range |  |
**channel** | Option<**String**> | The channel of the scheduled messages |  |
**latest** | Option<**f32**> | A UNIX timestamp of the latest value in the time range |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_unfurl

> ::std::collections::HashMap<String, serde_json::Value> chat_unfurl(token, ts, channel, user_auth_message, user_auth_required, unfurls, user_auth_url)


Provide custom unfurl behavior for user-posted URLs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `links:write` | [required] |
**ts** | **String** | Timestamp of the message to add unfurl behavior to. | [required] |
**channel** | **String** | Channel ID of the message | [required] |
**user_auth_message** | Option<**String**> | Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior |  |
**user_auth_required** | Option<**bool**> | Set to `true` or `1` to indicate the user must install your Slack app to trigger unfurls for this domain |  |
**unfurls** | Option<**String**> | URL-encoded JSON map with keys set to URLs featured in the the message, pointing to their unfurl blocks or message attachments. |  |
**user_auth_url** | Option<**String**> | Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## chat_update

> ::std::collections::HashMap<String, serde_json::Value> chat_update(token, ts, channel, blocks, attachments, as_user, parse, text, link_names)


Updates a message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `chat:write` | [required] |
**ts** | **String** | Timestamp of the message to be updated. | [required] |
**channel** | **String** | Channel containing the message to be updated. | [required] |
**blocks** | Option<**String**> | A JSON-based array of structured blocks, presented as a URL-encoded string. |  |
**attachments** | Option<**String**> | A JSON-based array of structured attachments, presented as a URL-encoded string. This field is required when not presenting `text`. |  |
**as_user** | Option<**String**> | Pass true to update the message as the authed user. [Bot users](/bot-users) in this context are considered authed users. |  |
**parse** | Option<**String**> | Change how messages are treated. Defaults to `client`, unlike `chat.postMessage`. Accepts either `none` or `full`. See [below](#formatting). |  |
**text** | Option<**String**> | New text for the message, using the [default formatting rules](/docs/formatting). It's not required when presenting `attachments`. |  |
**link_names** | Option<**String**> | Find and link channel names and usernames. Defaults to `none`. See [below](#formatting). |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


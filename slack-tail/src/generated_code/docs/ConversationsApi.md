# \ConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conversations_archive**](ConversationsApi.md#conversations_archive) | **post** /conversations.archive | 
[**conversations_close**](ConversationsApi.md#conversations_close) | **post** /conversations.close | 
[**conversations_create**](ConversationsApi.md#conversations_create) | **post** /conversations.create | 
[**conversations_history**](ConversationsApi.md#conversations_history) | **get** /conversations.history | 
[**conversations_info**](ConversationsApi.md#conversations_info) | **get** /conversations.info | 
[**conversations_invite**](ConversationsApi.md#conversations_invite) | **post** /conversations.invite | 
[**conversations_join**](ConversationsApi.md#conversations_join) | **post** /conversations.join | 
[**conversations_kick**](ConversationsApi.md#conversations_kick) | **post** /conversations.kick | 
[**conversations_leave**](ConversationsApi.md#conversations_leave) | **post** /conversations.leave | 
[**conversations_list**](ConversationsApi.md#conversations_list) | **get** /conversations.list | 
[**conversations_members**](ConversationsApi.md#conversations_members) | **get** /conversations.members | 
[**conversations_open**](ConversationsApi.md#conversations_open) | **post** /conversations.open | 
[**conversations_rename**](ConversationsApi.md#conversations_rename) | **post** /conversations.rename | 
[**conversations_replies**](ConversationsApi.md#conversations_replies) | **get** /conversations.replies | 
[**conversations_set_purpose**](ConversationsApi.md#conversations_set_purpose) | **post** /conversations.setPurpose | 
[**conversations_set_topic**](ConversationsApi.md#conversations_set_topic) | **post** /conversations.setTopic | 
[**conversations_unarchive**](ConversationsApi.md#conversations_unarchive) | **post** /conversations.unarchive | 



## conversations_archive

> ::std::collections::HashMap<String, serde_json::Value> conversations_archive(token, channel)


Archives a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to archive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_close

> ::std::collections::HashMap<String, serde_json::Value> conversations_close(token, channel)


Closes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to close. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_create

> ::std::collections::HashMap<String, serde_json::Value> conversations_create(token, name, user_ids, is_private)


Initiates a public or private channel-based conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | Name of the public or private channel to create |  |
**user_ids** | Option<**String**> | **Required** for workspace apps. A list of between 1 and 30 human users that will be added to the newly-created conversation. This argument has no effect when used by classic Slack apps. |  |
**is_private** | Option<**bool**> | Create a private channel instead of a public one |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_history

> ::std::collections::HashMap<String, serde_json::Value> conversations_history(inclusive, cursor, token, limit, oldest, channel, latest)


Fetches a conversation's history of messages and events.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Conversation ID to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_info

> ::std::collections::HashMap<String, serde_json::Value> conversations_info(include_num_members, token, channel, include_locale)


Retrieve information about a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_num_members** | Option<**bool**> | Set to `true` to include the member count for the specified conversation. Defaults to `false` |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**channel** | Option<**String**> | Conversation ID to learn more about |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this conversation. Defaults to `false` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_invite

> ::std::collections::HashMap<String, serde_json::Value> conversations_invite(token, users, channel)


Invites users to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**users** | Option<**String**> | A comma separated list of user IDs. Up to 1000 users may be listed. |  |
**channel** | Option<**String**> | The ID of the public or private channel to invite user(s) to. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_join

> ::std::collections::HashMap<String, serde_json::Value> conversations_join(token, channel)


Joins an existing conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | ID of conversation to join |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_kick

> ::std::collections::HashMap<String, serde_json::Value> conversations_kick(token, user, channel)


Removes a user from a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**user** | Option<**String**> | User ID to be removed. |  |
**channel** | Option<**String**> | ID of conversation to remove user from. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_leave

> ::std::collections::HashMap<String, serde_json::Value> conversations_leave(token, channel)


Leaves a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | Conversation to leave |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_list

> ::std::collections::HashMap<String, serde_json::Value> conversations_list(cursor, token, limit, exclude_archived, types)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_members

> ::std::collections::HashMap<String, serde_json::Value> conversations_members(cursor, token, limit, channel)


Retrieve members of a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**channel** | Option<**String**> | ID of the conversation to retrieve members for |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_open

> ::std::collections::HashMap<String, serde_json::Value> conversations_open(token, return_im, users, channel)


Opens or resumes a direct message or multi-person direct message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**return_im** | Option<**bool**> | Boolean, indicates you want the full IM channel definition in the response. |  |
**users** | Option<**String**> | Comma separated lists of users. If only one user is included, this creates a 1:1 DM.  The ordering of the users is preserved whenever a multi-person direct message is returned. Supply a `channel` when not supplying `users`. |  |
**channel** | Option<**String**> | Resume a conversation by supplying an `im` or `mpim`'s ID. Or provide the `users` field instead. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_rename

> ::std::collections::HashMap<String, serde_json::Value> conversations_rename(token, name, channel)


Renames a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**name** | Option<**String**> | New name for conversation. |  |
**channel** | Option<**String**> | ID of conversation to rename |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_replies

> ::std::collections::HashMap<String, serde_json::Value> conversations_replies(inclusive, ts, cursor, token, limit, oldest, channel, latest)


Retrieve a thread of messages posted to a conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results only when either timestamp is specified. |  |
**ts** | Option<**f32**> | Unique identifier of a thread's parent message. |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:history` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Conversation ID to fetch thread from. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_purpose

> ::std::collections::HashMap<String, serde_json::Value> conversations_set_purpose(token, purpose, channel)


Sets the purpose for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**purpose** | Option<**String**> | A new, specialer purpose |  |
**channel** | Option<**String**> | Conversation to set the purpose of |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_set_topic

> ::std::collections::HashMap<String, serde_json::Value> conversations_set_topic(token, topic, channel)


Sets the topic for a conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**topic** | Option<**String**> | The new topic string. Does not support formatting or linkification. |  |
**channel** | Option<**String**> | Conversation to set the topic of |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_unarchive

> ::std::collections::HashMap<String, serde_json::Value> conversations_unarchive(token, channel)


Reverses conversation archival.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:write` |  |
**channel** | Option<**String**> | ID of conversation to unarchive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


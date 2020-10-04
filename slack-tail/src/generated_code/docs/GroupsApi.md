# \GroupsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**groups_archive**](GroupsApi.md#groups_archive) | **post** /groups.archive | 
[**groups_create**](GroupsApi.md#groups_create) | **post** /groups.create | 
[**groups_create_child**](GroupsApi.md#groups_create_child) | **post** /groups.createChild | 
[**groups_history**](GroupsApi.md#groups_history) | **get** /groups.history | 
[**groups_info**](GroupsApi.md#groups_info) | **get** /groups.info | 
[**groups_invite**](GroupsApi.md#groups_invite) | **post** /groups.invite | 
[**groups_kick**](GroupsApi.md#groups_kick) | **post** /groups.kick | 
[**groups_leave**](GroupsApi.md#groups_leave) | **post** /groups.leave | 
[**groups_list**](GroupsApi.md#groups_list) | **get** /groups.list | 
[**groups_mark**](GroupsApi.md#groups_mark) | **post** /groups.mark | 
[**groups_open**](GroupsApi.md#groups_open) | **post** /groups.open | 
[**groups_rename**](GroupsApi.md#groups_rename) | **post** /groups.rename | 
[**groups_replies**](GroupsApi.md#groups_replies) | **get** /groups.replies | 
[**groups_set_purpose**](GroupsApi.md#groups_set_purpose) | **post** /groups.setPurpose | 
[**groups_set_topic**](GroupsApi.md#groups_set_topic) | **post** /groups.setTopic | 
[**groups_unarchive**](GroupsApi.md#groups_unarchive) | **post** /groups.unarchive | 



## groups_archive

> ::std::collections::HashMap<String, serde_json::Value> groups_archive(token, channel)


Archives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to archive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create

> ::std::collections::HashMap<String, serde_json::Value> groups_create(token, validate, name)


Creates a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of private channel to create |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create_child

> ::std::collections::HashMap<String, serde_json::Value> groups_create_child(token, channel)


Clones and archives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to clone and archive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_history

> ::std::collections::HashMap<String, serde_json::Value> groups_history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Private channel to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_info

> ::std::collections::HashMap<String, serde_json::Value> groups_info(token, include_locale, channel)


Gets information about a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:read` |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this group. Defaults to `false` |  |
**channel** | Option<**String**> | Private channel to get info on |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_invite

> ::std::collections::HashMap<String, serde_json::Value> groups_invite(token, user, channel)


Invites a user to a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**user** | Option<**String**> | User to invite. |  |
**channel** | Option<**String**> | Private channel to invite user to. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_kick

> ::std::collections::HashMap<String, serde_json::Value> groups_kick(token, user, channel)


Removes a user from a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `groups:write` | [required] |
**user** | **String** | User to remove from private channel. | [required] |
**channel** | **String** | Private channel to remove user from. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_leave

> ::std::collections::HashMap<String, serde_json::Value> groups_leave(token, channel)


Leaves a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `groups:write` | [required] |
**channel** | **String** | Private channel to leave | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_list

> ::std::collections::HashMap<String, serde_json::Value> groups_list(exclude_members, cursor, token, limit, exclude_archived)


Lists private channels that the calling user has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_members** | Option<**bool**> | Exclude the `members` from each `group` |  |
**cursor** | Option<**String**> | Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more details. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**exclude_archived** | Option<**bool**> | Don't return archived private channels. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_mark

> ::std::collections::HashMap<String, serde_json::Value> groups_mark(token, ts, channel)


Sets the read cursor in a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**ts** | Option<**f32**> | Timestamp of the most recently seen message. |  |
**channel** | Option<**String**> | Private channel to set reading cursor in. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_open

> ::std::collections::HashMap<String, serde_json::Value> groups_open(token, channel)


Opens a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to open. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_rename

> ::std::collections::HashMap<String, serde_json::Value> groups_rename(token, validate, name, channel)


Renames a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | New name for private channel. |  |
**channel** | Option<**String**> | Private channel to rename |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_replies

> ::std::collections::HashMap<String, serde_json::Value> groups_replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a private channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message |  |
**token** | Option<**String**> | Authentication token. Requires scope: `groups:history` |  |
**channel** | Option<**String**> | Private channel to fetch thread from |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_set_purpose

> ::std::collections::HashMap<String, serde_json::Value> groups_set_purpose(token, purpose, channel)


Sets the purpose for a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**purpose** | Option<**String**> | The new purpose |  |
**channel** | Option<**String**> | Private channel to set the purpose of |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_set_topic

> ::std::collections::HashMap<String, serde_json::Value> groups_set_topic(token, topic, channel)


Sets the topic for a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**topic** | Option<**String**> | The new topic |  |
**channel** | Option<**String**> | Private channel to set the topic of |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_unarchive

> ::std::collections::HashMap<String, serde_json::Value> groups_unarchive(token, channel)


Unarchives a private channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `groups:write` |  |
**channel** | Option<**String**> | Private channel to unarchive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


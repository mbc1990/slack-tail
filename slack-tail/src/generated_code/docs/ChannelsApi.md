# \ChannelsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channels_archive**](ChannelsApi.md#channels_archive) | **post** /channels.archive | 
[**channels_create**](ChannelsApi.md#channels_create) | **post** /channels.create | 
[**channels_history**](ChannelsApi.md#channels_history) | **get** /channels.history | 
[**channels_info**](ChannelsApi.md#channels_info) | **get** /channels.info | 
[**channels_invite**](ChannelsApi.md#channels_invite) | **post** /channels.invite | 
[**channels_join**](ChannelsApi.md#channels_join) | **post** /channels.join | 
[**channels_kick**](ChannelsApi.md#channels_kick) | **post** /channels.kick | 
[**channels_leave**](ChannelsApi.md#channels_leave) | **post** /channels.leave | 
[**channels_list**](ChannelsApi.md#channels_list) | **get** /channels.list | 
[**channels_mark**](ChannelsApi.md#channels_mark) | **post** /channels.mark | 
[**channels_rename**](ChannelsApi.md#channels_rename) | **post** /channels.rename | 
[**channels_replies**](ChannelsApi.md#channels_replies) | **get** /channels.replies | 
[**channels_set_purpose**](ChannelsApi.md#channels_set_purpose) | **post** /channels.setPurpose | 
[**channels_set_topic**](ChannelsApi.md#channels_set_topic) | **post** /channels.setTopic | 
[**channels_unarchive**](ChannelsApi.md#channels_unarchive) | **post** /channels.unarchive | 



## channels_archive

> ::std::collections::HashMap<String, serde_json::Value> channels_archive(token, channel)


Archives a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | Channel to archive |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_create

> ::std::collections::HashMap<String, serde_json::Value> channels_create(token, validate, name)


Creates a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of channel to create |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_history

> ::std::collections::HashMap<String, serde_json::Value> channels_history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Channel to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_info

> ::std::collections::HashMap<String, serde_json::Value> channels_info(token, include_locale, channel)


Gets information about a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:read` |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this channel. Defaults to `false` |  |
**channel** | Option<**String**> | Channel to get info on |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_invite

> ::std::collections::HashMap<String, serde_json::Value> channels_invite(token, user, channel)


Invites a user to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**user** | Option<**String**> | User to invite to channel. |  |
**channel** | Option<**String**> | Channel to invite user to. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_join

> ::std::collections::HashMap<String, serde_json::Value> channels_join(token, validate, name)


Joins a channel, creating it if needed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | Name of channel to join |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_kick

> ::std::collections::HashMap<String, serde_json::Value> channels_kick(token, user, channel)


Removes a user from a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**user** | Option<**String**> | User to remove from channel. |  |
**channel** | Option<**String**> | Channel to remove user from. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_leave

> ::std::collections::HashMap<String, serde_json::Value> channels_leave(token, channel)


Leaves a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**channel** | Option<**String**> | Channel to leave |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_list

> ::std::collections::HashMap<String, serde_json::Value> channels_list(exclude_members, cursor, token, limit, exclude_archived)


Lists all channels in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_members** | Option<**bool**> | Exclude the `members` collection from each `channel` |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**exclude_archived** | Option<**bool**> | Exclude archived channels from the list |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_mark

> ::std::collections::HashMap<String, serde_json::Value> channels_mark(token, ts, channel)


Sets the read cursor in a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**ts** | Option<**f32**> | Timestamp of the most recently seen message. |  |
**channel** | Option<**String**> | Channel to set reading cursor in. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_rename

> ::std::collections::HashMap<String, serde_json::Value> channels_rename(token, validate, name, channel)


Renames a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `channels:write` |  |
**validate** | Option<**bool**> | Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria. |  |
**name** | Option<**String**> | New name for channel. |  |
**channel** | Option<**String**> | Channel to rename |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_replies

> ::std::collections::HashMap<String, serde_json::Value> channels_replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message |  |
**token** | Option<**String**> | Authentication token. Requires scope: `channels:history` |  |
**channel** | Option<**String**> | Channel to fetch thread from |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_set_purpose

> ::std::collections::HashMap<String, serde_json::Value> channels_set_purpose(token, purpose, channel, name_tagging)


Sets the purpose for a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**purpose** | **String** | The new purpose | [required] |
**channel** | **String** | Channel to set the purpose of | [required] |
**name_tagging** | Option<**bool**> | if it is true, treat this like a message and not an unescaped thing |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_set_topic

> ::std::collections::HashMap<String, serde_json::Value> channels_set_topic(token, topic, channel)


Sets the topic for a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**topic** | **String** | The new topic | [required] |
**channel** | **String** | Channel to set the topic of | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## channels_unarchive

> ::std::collections::HashMap<String, serde_json::Value> channels_unarchive(token, channel)


Unarchives a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `channels:write` | [required] |
**channel** | **String** | Channel to unarchive | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


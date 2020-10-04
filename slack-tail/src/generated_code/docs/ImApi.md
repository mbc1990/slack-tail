# \ImApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**im_close**](ImApi.md#im_close) | **post** /im.close | 
[**im_history**](ImApi.md#im_history) | **get** /im.history | 
[**im_list**](ImApi.md#im_list) | **get** /im.list | 
[**im_mark**](ImApi.md#im_mark) | **post** /im.mark | 
[**im_open**](ImApi.md#im_open) | **post** /im.open | 
[**im_replies**](ImApi.md#im_replies) | **get** /im.replies | 



## im_close

> ::std::collections::HashMap<String, serde_json::Value> im_close(token, channel)


Close a direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `im:write` | [required] |
**channel** | **String** | Direct message channel to close. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## im_history

> ::std::collections::HashMap<String, serde_json::Value> im_history(count, unreads, inclusive, token, oldest, channel, latest)


Fetches history of messages and events from direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**i32**> | Number of messages to return, between 1 and 1000. |  |
**unreads** | Option<**bool**> | Include `unread_count_display` in the output? |  |
**inclusive** | Option<**bool**> | Include messages with latest or oldest timestamp in results. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `im:history` |  |
**oldest** | Option<**f32**> | Start of time range of messages to include in results. |  |
**channel** | Option<**String**> | Direct message channel to fetch history for. |  |
**latest** | Option<**f32**> | End of time range of messages to include in results. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## im_list

> ::std::collections::HashMap<String, serde_json::Value> im_list(cursor, token, limit)


Lists direct message channels for the calling user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `im:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## im_mark

> ::std::collections::HashMap<String, serde_json::Value> im_mark(token, channel, ts)


Sets the read cursor in a direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `im:write` | [required] |
**channel** | **String** | Direct message channel to set reading cursor in. | [required] |
**ts** | **String** | Timestamp of the most recently seen message. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## im_open

> ::std::collections::HashMap<String, serde_json::Value> im_open(token, return_im, user, include_locale)


Opens a direct message channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `im:write` |  |
**return_im** | Option<**bool**> | Boolean, indicates you want the full IM channel definition in the response. |  |
**user** | Option<**String**> | User to open a direct message channel with. |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this im. Defaults to `false` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## im_replies

> ::std::collections::HashMap<String, serde_json::Value> im_replies(thread_ts, token, channel)


Retrieve a thread of messages posted to a direct message conversation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**thread_ts** | Option<**f32**> | Unique identifier of a thread's parent message |  |
**token** | Option<**String**> | Authentication token. Requires scope: `im:history` |  |
**channel** | Option<**String**> | Direct message channel to fetch thread from |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


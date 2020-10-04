# \SearchApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_messages**](SearchApi.md#search_messages) | **get** /search.messages | 



## search_messages

> ::std::collections::HashMap<String, serde_json::Value> search_messages(query, token, sort_dir, sort, count, highlight, page)


Searches for messages matching a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Search query. | [required] |
**token** | **String** | Authentication token. Requires scope: `search:read` | [required] |
**sort_dir** | Option<**String**> | Change sort direction to ascending (`asc`) or descending (`desc`). |  |
**sort** | Option<**String**> | Return matches sorted by either `score` or `timestamp`. |  |
**count** | Option<**i32**> | Pass the number of results you want per \"page\". Maximum of `100`. |  |
**highlight** | Option<**bool**> | Pass a value of `true` to enable query highlight markers (see below). |  |
**page** | Option<**i32**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


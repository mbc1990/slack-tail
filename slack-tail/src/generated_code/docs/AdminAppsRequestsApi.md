# \AdminAppsRequestsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_requests_list**](AdminAppsRequestsApi.md#admin_apps_requests_list) | **get** /admin.apps.requests.list | 



## admin_apps_requests_list

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_requests_list(token, cursor, limit, team_id)


List app requests for a team/workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**team_id** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AdminAppsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_approve**](AdminAppsApi.md#admin_apps_approve) | **post** /admin.apps.approve | 
[**admin_apps_restrict**](AdminAppsApi.md#admin_apps_restrict) | **post** /admin.apps.restrict | 



## admin_apps_approve

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_approve(token, team_id, app_id, request_id)


Approve an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**team_id** | Option<**String**> |  |  |
**app_id** | Option<**String**> | The id of the app to approve. |  |
**request_id** | Option<**String**> | The id of the request to approve. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_restrict

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_restrict(token, team_id, app_id, request_id)


Restrict an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**team_id** | Option<**String**> |  |  |
**app_id** | Option<**String**> | The id of the app to restrict. |  |
**request_id** | Option<**String**> | The id of the request to restrict. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


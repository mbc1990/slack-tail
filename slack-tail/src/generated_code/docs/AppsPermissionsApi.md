# \AppsPermissionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_info**](AppsPermissionsApi.md#apps_permissions_info) | **get** /apps.permissions.info | 
[**apps_permissions_request**](AppsPermissionsApi.md#apps_permissions_request) | **get** /apps.permissions.request | 



## apps_permissions_info

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_info(token)


Returns list of permissions this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_request

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_request(scopes, token, trigger_id)


Allows an app to request additional scopes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scopes** | **String** | A comma separated list of scopes to request for | [required] |
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Token used to trigger the permissions API | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


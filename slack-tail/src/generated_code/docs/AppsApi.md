# \AppsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_info**](AppsApi.md#apps_permissions_info) | **get** /apps.permissions.info | 
[**apps_permissions_request**](AppsApi.md#apps_permissions_request) | **get** /apps.permissions.request | 
[**apps_permissions_resources_list**](AppsApi.md#apps_permissions_resources_list) | **get** /apps.permissions.resources.list | 
[**apps_permissions_scopes_list**](AppsApi.md#apps_permissions_scopes_list) | **get** /apps.permissions.scopes.list | 
[**apps_permissions_users_list**](AppsApi.md#apps_permissions_users_list) | **get** /apps.permissions.users.list | 
[**apps_permissions_users_request**](AppsApi.md#apps_permissions_users_request) | **get** /apps.permissions.users.request | 
[**apps_uninstall**](AppsApi.md#apps_uninstall) | **get** /apps.uninstall | 



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


## apps_permissions_resources_list

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_resources_list(token, cursor, limit)


Returns list of resource grants this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_scopes_list

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_scopes_list(token)


Returns list of scopes this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_users_list

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_users_list(token, cursor, limit)


Returns list of user grants and corresponding scopes this app has on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_permissions_users_request

> ::std::collections::HashMap<String, serde_json::Value> apps_permissions_users_request(scopes, token, user, trigger_id)


Enables an app to trigger a permissions modal to grant an app access to a user access scope.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scopes** | **String** | A comma separated list of user scopes to request for | [required] |
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**user** | **String** | The user this scope is being requested for | [required] |
**trigger_id** | **String** | Token used to trigger the request | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## apps_uninstall

> ::std::collections::HashMap<String, serde_json::Value> apps_uninstall(client_secret, token, client_id)


Uninstalls your app from a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `none` |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


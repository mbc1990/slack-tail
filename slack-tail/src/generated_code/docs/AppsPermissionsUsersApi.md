# \AppsPermissionsUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**apps_permissions_users_list**](AppsPermissionsUsersApi.md#apps_permissions_users_list) | **Get** /apps.permissions.users.list | 
[**apps_permissions_users_request**](AppsPermissionsUsersApi.md#apps_permissions_users_request) | **Get** /apps.permissions.users.request | 


# **apps_permissions_users_list**
> ::std::collections::HashMap<String, Value> apps_permissions_users_list(ctx, token, optional)


Returns list of user grants and corresponding scopes this app has on a team.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **cursor** | **String**| Paginate through collections of data by setting the &#x60;cursor&#x60; parameter to a &#x60;next_cursor&#x60; attribute returned by a previous request&#39;s &#x60;response_metadata&#x60;. Default value fetches the first \&quot;page\&quot; of the collection. See [pagination](/docs/pagination) for more detail. | 
 **limit** | **i32**| The maximum number of items to return. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **apps_permissions_users_request**
> ::std::collections::HashMap<String, Value> apps_permissions_users_request(ctx, scopes, token, user, trigger_id)


Enables an app to trigger a permissions modal to grant an app access to a user access scope.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **scopes** | **String**| A comma separated list of user scopes to request for | 
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **user** | **String**| The user this scope is being requested for | 
  **trigger_id** | **String**| Token used to trigger the request | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


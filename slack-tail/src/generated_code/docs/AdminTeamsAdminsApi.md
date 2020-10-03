# \AdminTeamsAdminsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_admins_list**](AdminTeamsAdminsApi.md#admin_teams_admins_list) | **Get** /admin.teams.admins.list | 


# **admin_teams_admins_list**
> ::std::collections::HashMap<String, Value> admin_teams_admins_list(ctx, token, team_id, optional)


List all of the admins on a given workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
  **team_id** | **String**|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
 **team_id** | **String**|  | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page. | 
 **limit** | **i32**| The maximum number of items to return. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


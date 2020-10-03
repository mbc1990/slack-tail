# \AdminTeamsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_create**](AdminTeamsApi.md#admin_teams_create) | **Post** /admin.teams.create | 
[**admin_teams_list**](AdminTeamsApi.md#admin_teams_list) | **Get** /admin.teams.list | 


# **admin_teams_create**
> ::std::collections::HashMap<String, Value> admin_teams_create(ctx, team_domain, token, team_name, optional)


Create an Enterprise team.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **team_domain** | **String**| Team domain (for example, slacksoftballteam). | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **team_name** | **String**| Team name (for example, Slack Softball Team). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **team_domain** | **String**| Team domain (for example, slacksoftballteam). | 
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
 **team_name** | **String**| Team name (for example, Slack Softball Team). | 
 **team_description** | **String**| Description for the team. | 
 **team_discoverability** | **String**| Who can join the team. A team&#39;s discoverability can be &#x60;open&#x60;, &#x60;closed&#x60;, &#x60;invite_only&#x60;, or &#x60;unlisted&#x60;. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_list**
> ::std::collections::HashMap<String, Value> admin_teams_list(ctx, token, optional)


List all teams on an Enterprise organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page. | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 100 both inclusive. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DndApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dnd_end_dnd**](DndApi.md#dnd_end_dnd) | **Post** /dnd.endDnd | 
[**dnd_end_snooze**](DndApi.md#dnd_end_snooze) | **Post** /dnd.endSnooze | 
[**dnd_info**](DndApi.md#dnd_info) | **Get** /dnd.info | 
[**dnd_set_snooze**](DndApi.md#dnd_set_snooze) | **Post** /dnd.setSnooze | 
[**dnd_team_info**](DndApi.md#dnd_team_info) | **Get** /dnd.teamInfo | 


# **dnd_end_dnd**
> ::std::collections::HashMap<String, Value> dnd_end_dnd(ctx, token)


Ends the current user's Do Not Disturb session immediately.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;dnd:write&#x60; | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dnd_end_snooze**
> ::std::collections::HashMap<String, Value> dnd_end_snooze(ctx, token)


Ends the current user's snooze mode immediately.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;dnd:write&#x60; | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dnd_info**
> ::std::collections::HashMap<String, Value> dnd_info(ctx, optional)


Retrieves a user's current Do Not Disturb status.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;dnd:read&#x60; | 
 **user** | **String**| User to fetch status for (defaults to current user) | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dnd_set_snooze**
> ::std::collections::HashMap<String, Value> dnd_set_snooze(ctx, num_minutes, token)


Turns on Do Not Disturb mode for the current user, or changes its duration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **num_minutes** | **String**| Number of minutes, from now, to snooze until. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;dnd:write&#x60; | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dnd_team_info**
> ::std::collections::HashMap<String, Value> dnd_team_info(ctx, optional)


Retrieves the Do Not Disturb status for up to 50 users on a team.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;dnd:read&#x60; | 
 **users** | **String**| Comma-separated list of users to fetch Do Not Disturb status for | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


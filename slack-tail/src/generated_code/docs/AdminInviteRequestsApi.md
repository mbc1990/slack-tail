# \AdminInviteRequestsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_invite_requests_approve**](AdminInviteRequestsApi.md#admin_invite_requests_approve) | **Post** /admin.inviteRequests.approve | 
[**admin_invite_requests_deny**](AdminInviteRequestsApi.md#admin_invite_requests_deny) | **Post** /admin.inviteRequests.deny | 
[**admin_invite_requests_list**](AdminInviteRequestsApi.md#admin_invite_requests_list) | **Get** /admin.inviteRequests.list | 


# **admin_invite_requests_approve**
> ::std::collections::HashMap<String, Value> admin_invite_requests_approve(ctx, token, invite_request_id, optional)


Approve a workspace invite request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:write&#x60; | 
  **invite_request_id** | **String**| ID of the request to invite. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:write&#x60; | 
 **invite_request_id** | **String**| ID of the request to invite. | 
 **team_id** | **String**| ID for the workspace where the invite request was made. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_invite_requests_deny**
> ::std::collections::HashMap<String, Value> admin_invite_requests_deny(ctx, token, invite_request_id, optional)


Deny a workspace invite request.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:write&#x60; | 
  **invite_request_id** | **String**| ID of the request to invite. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:write&#x60; | 
 **invite_request_id** | **String**| ID of the request to invite. | 
 **team_id** | **String**| ID for the workspace where the invite request was made. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_invite_requests_list**
> ::std::collections::HashMap<String, Value> admin_invite_requests_list(ctx, token, optional)


List all pending workspace invite requests.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.invites:read&#x60; | 
 **cursor** | **String**| Value of the &#x60;next_cursor&#x60; field sent as part of the previous API response | 
 **limit** | **i32**| The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive | 
 **team_id** | **String**| ID for the workspace where the invite requests were made. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AdminUsersSessionApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_users_session_reset**](AdminUsersSessionApi.md#admin_users_session_reset) | **Post** /admin.users.session.reset | 


# **admin_users_session_reset**
> ::std::collections::HashMap<String, Value> admin_users_session_reset(ctx, token, user_id, optional)


Wipes all valid sessions on all devices for a given user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| The ID of the user to wipe sessions for | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
 **user_id** | **String**| The ID of the user to wipe sessions for | 
 **mobile_only** | **bool**| Only expire mobile sessions (default: false) | 
 **web_only** | **bool**| Only expire web sessions (default: false) | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


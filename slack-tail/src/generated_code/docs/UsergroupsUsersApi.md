# \UsergroupsUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**usergroups_users_list**](UsergroupsUsersApi.md#usergroups_users_list) | **Get** /usergroups.users.list | 
[**usergroups_users_update**](UsergroupsUsersApi.md#usergroups_users_update) | **Post** /usergroups.users.update | 


# **usergroups_users_list**
> ::std::collections::HashMap<String, Value> usergroups_users_list(ctx, token, usergroup, optional)


List all users in a User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:read&#x60; | 
  **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:read&#x60; | 
 **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **include_disabled** | **bool**| Allow results that involve disabled User Groups. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **usergroups_users_update**
> ::std::collections::HashMap<String, Value> usergroups_users_update(ctx, users, token, usergroup, optional)


Update the list of users for a User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **users** | **String**| A comma separated string of encoded user IDs that represent the entire list of users for the User Group. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
  **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **users** | **String**| A comma separated string of encoded user IDs that represent the entire list of users for the User Group. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
 **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **include_count** | **bool**| Include the number of users in the User Group. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


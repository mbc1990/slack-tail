# \UsergroupsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**usergroups_create**](UsergroupsApi.md#usergroups_create) | **Post** /usergroups.create | 
[**usergroups_disable**](UsergroupsApi.md#usergroups_disable) | **Post** /usergroups.disable | 
[**usergroups_enable**](UsergroupsApi.md#usergroups_enable) | **Post** /usergroups.enable | 
[**usergroups_list**](UsergroupsApi.md#usergroups_list) | **Get** /usergroups.list | 
[**usergroups_update**](UsergroupsApi.md#usergroups_update) | **Post** /usergroups.update | 
[**usergroups_users_list**](UsergroupsApi.md#usergroups_users_list) | **Get** /usergroups.users.list | 
[**usergroups_users_update**](UsergroupsApi.md#usergroups_users_update) | **Post** /usergroups.users.update | 


# **usergroups_create**
> ::std::collections::HashMap<String, Value> usergroups_create(ctx, token, name, optional)


Create a User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
  **name** | **String**| A name for the User Group. Must be unique among User Groups. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
 **name** | **String**| A name for the User Group. Must be unique among User Groups. | 
 **handle** | **String**| A mention handle. Must be unique among channels, users and User Groups. | 
 **description** | **String**| A short description of the User Group. | 
 **channels** | **String**| A comma separated string of encoded channel IDs for which the User Group uses as a default. | 
 **include_count** | **bool**| Include the number of users in each User Group. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **usergroups_disable**
> ::std::collections::HashMap<String, Value> usergroups_disable(ctx, token, usergroup, optional)


Disable an existing User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
  **usergroup** | **String**| The encoded ID of the User Group to disable. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
 **usergroup** | **String**| The encoded ID of the User Group to disable. | 
 **include_count** | **bool**| Include the number of users in the User Group. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **usergroups_enable**
> ::std::collections::HashMap<String, Value> usergroups_enable(ctx, token, usergroup, optional)


Enable a User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
  **usergroup** | **String**| The encoded ID of the User Group to enable. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
 **usergroup** | **String**| The encoded ID of the User Group to enable. | 
 **include_count** | **bool**| Include the number of users in the User Group. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **usergroups_list**
> ::std::collections::HashMap<String, Value> usergroups_list(ctx, token, optional)


List all User Groups for a team

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:read&#x60; | 
 **include_users** | **bool**| Include the list of users for each User Group. | 
 **include_count** | **bool**| Include the number of users in each User Group. | 
 **include_disabled** | **bool**| Include disabled User Groups. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **usergroups_update**
> ::std::collections::HashMap<String, Value> usergroups_update(ctx, token, usergroup, optional)


Update an existing User Group

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
  **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;usergroups:write&#x60; | 
 **usergroup** | **String**| The encoded ID of the User Group to update. | 
 **handle** | **String**| A mention handle. Must be unique among channels, users and User Groups. | 
 **description** | **String**| A short description of the User Group. | 
 **channels** | **String**| A comma separated string of encoded channel IDs for which the User Group uses as a default. | 
 **include_count** | **bool**| Include the number of users in the User Group. | 
 **name** | **String**| A name for the User Group. Must be unique among User Groups. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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


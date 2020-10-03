# \AdminUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_users_assign**](AdminUsersApi.md#admin_users_assign) | **Post** /admin.users.assign | 
[**admin_users_invite**](AdminUsersApi.md#admin_users_invite) | **Post** /admin.users.invite | 
[**admin_users_list**](AdminUsersApi.md#admin_users_list) | **Get** /admin.users.list | 
[**admin_users_remove**](AdminUsersApi.md#admin_users_remove) | **Post** /admin.users.remove | 
[**admin_users_set_admin**](AdminUsersApi.md#admin_users_set_admin) | **Post** /admin.users.setAdmin | 
[**admin_users_set_expiration**](AdminUsersApi.md#admin_users_set_expiration) | **Post** /admin.users.setExpiration | 
[**admin_users_set_owner**](AdminUsersApi.md#admin_users_set_owner) | **Post** /admin.users.setOwner | 
[**admin_users_set_regular**](AdminUsersApi.md#admin_users_set_regular) | **Post** /admin.users.setRegular | 


# **admin_users_assign**
> ::std::collections::HashMap<String, Value> admin_users_assign(ctx, user_id, team_id, token, optional)


Add an Enterprise user to a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **user_id** | **String**| The ID of the user to add to the workspace. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **user_id** | **String**| The ID of the user to add to the workspace. | 
 **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
 **channel_ids** | **String**| Comma separated values of channel IDs to add user in the new workspace. | 
 **is_ultra_restricted** | **bool**| True if user should be added to the workspace as a single-channel guest. | 
 **is_restricted** | **bool**| True if user should be added to the workspace as a guest. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_invite**
> ::std::collections::HashMap<String, Value> admin_users_invite(ctx, channel_ids, team_id, token, email, optional)


Invite a user to a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **channel_ids** | **String**| A comma-separated list of &#x60;channel_id&#x60;s for this user to join. At least one channel is required. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **email** | **String**| The email address of the person to invite. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channel_ids** | **String**| A comma-separated list of &#x60;channel_id&#x60;s for this user to join. At least one channel is required. | 
 **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
 **email** | **String**| The email address of the person to invite. | 
 **real_name** | **String**| Full name of the user. | 
 **is_ultra_restricted** | **bool**| Is this user a single channel guest user? (default: false) | 
 **custom_message** | **String**| An optional message to send to the user in the invite email. | 
 **is_restricted** | **bool**| Is this user a multi-channel guest user? (default: false) | 
 **guest_expiration_ts** | **String**| Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date. | 
 **resend** | **bool**| Allow this invite to be resent in the future if a user has not signed up yet. (default: false) | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_list**
> ::std::collections::HashMap<String, Value> admin_users_list(ctx, token, team_id, optional)


List users on a workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:read&#x60; | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:read&#x60; | 
 **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page. | 
 **limit** | **i32**| Limit for how many users to be retrieved per page | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_remove**
> ::std::collections::HashMap<String, Value> admin_users_remove(ctx, token, user_id, team_id)


Remove a user from a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| The ID of the user to remove. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_set_admin**
> ::std::collections::HashMap<String, Value> admin_users_set_admin(ctx, token, user_id, team_id)


Set an existing guest, regular user, or owner to be an admin user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| The ID of the user to designate as an admin. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_set_expiration**
> ::std::collections::HashMap<String, Value> admin_users_set_expiration(ctx, expiration_ts, token, user_id, team_id)


Set an expiration for a guest user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **expiration_ts** | **i32**| Timestamp when guest account should be disabled. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| The ID of the user to set an expiration for. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_set_owner**
> ::std::collections::HashMap<String, Value> admin_users_set_owner(ctx, token, user_id, team_id)


Set an existing guest, regular user, or admin user to be a workspace owner.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| Id of the user to promote to owner. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_users_set_regular**
> ::std::collections::HashMap<String, Value> admin_users_set_regular(ctx, token, user_id, team_id)


Set an existing guest user, admin user, or owner to be a regular user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.users:write&#x60; | 
  **user_id** | **String**| The ID of the user to designate as a regular user. | 
  **team_id** | **String**| The ID (&#x60;T1234&#x60;) of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


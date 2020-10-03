# \AdminApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_approve**](AdminApi.md#admin_apps_approve) | **Post** /admin.apps.approve | 
[**admin_apps_approved_list**](AdminApi.md#admin_apps_approved_list) | **Get** /admin.apps.approved.list | 
[**admin_apps_requests_list**](AdminApi.md#admin_apps_requests_list) | **Get** /admin.apps.requests.list | 
[**admin_apps_restrict**](AdminApi.md#admin_apps_restrict) | **Post** /admin.apps.restrict | 
[**admin_apps_restricted_list**](AdminApi.md#admin_apps_restricted_list) | **Get** /admin.apps.restricted.list | 
[**admin_conversations_set_teams**](AdminApi.md#admin_conversations_set_teams) | **Post** /admin.conversations.setTeams | 
[**admin_emoji_add**](AdminApi.md#admin_emoji_add) | **Post** /admin.emoji.add | 
[**admin_emoji_add_alias**](AdminApi.md#admin_emoji_add_alias) | **Post** /admin.emoji.addAlias | 
[**admin_emoji_list**](AdminApi.md#admin_emoji_list) | **Get** /admin.emoji.list | 
[**admin_emoji_remove**](AdminApi.md#admin_emoji_remove) | **Post** /admin.emoji.remove | 
[**admin_emoji_rename**](AdminApi.md#admin_emoji_rename) | **Post** /admin.emoji.rename | 
[**admin_invite_requests_approve**](AdminApi.md#admin_invite_requests_approve) | **Post** /admin.inviteRequests.approve | 
[**admin_invite_requests_approved_list**](AdminApi.md#admin_invite_requests_approved_list) | **Get** /admin.inviteRequests.approved.list | 
[**admin_invite_requests_denied_list**](AdminApi.md#admin_invite_requests_denied_list) | **Get** /admin.inviteRequests.denied.list | 
[**admin_invite_requests_deny**](AdminApi.md#admin_invite_requests_deny) | **Post** /admin.inviteRequests.deny | 
[**admin_invite_requests_list**](AdminApi.md#admin_invite_requests_list) | **Get** /admin.inviteRequests.list | 
[**admin_teams_admins_list**](AdminApi.md#admin_teams_admins_list) | **Get** /admin.teams.admins.list | 
[**admin_teams_create**](AdminApi.md#admin_teams_create) | **Post** /admin.teams.create | 
[**admin_teams_list**](AdminApi.md#admin_teams_list) | **Get** /admin.teams.list | 
[**admin_teams_owners_list**](AdminApi.md#admin_teams_owners_list) | **Get** /admin.teams.owners.list | 
[**admin_teams_settings_info**](AdminApi.md#admin_teams_settings_info) | **Get** /admin.teams.settings.info | 
[**admin_teams_settings_set_default_channels**](AdminApi.md#admin_teams_settings_set_default_channels) | **Post** /admin.teams.settings.setDefaultChannels | 
[**admin_teams_settings_set_description**](AdminApi.md#admin_teams_settings_set_description) | **Post** /admin.teams.settings.setDescription | 
[**admin_teams_settings_set_discoverability**](AdminApi.md#admin_teams_settings_set_discoverability) | **Post** /admin.teams.settings.setDiscoverability | 
[**admin_teams_settings_set_icon**](AdminApi.md#admin_teams_settings_set_icon) | **Post** /admin.teams.settings.setIcon | 
[**admin_teams_settings_set_name**](AdminApi.md#admin_teams_settings_set_name) | **Post** /admin.teams.settings.setName | 
[**admin_users_assign**](AdminApi.md#admin_users_assign) | **Post** /admin.users.assign | 
[**admin_users_invite**](AdminApi.md#admin_users_invite) | **Post** /admin.users.invite | 
[**admin_users_list**](AdminApi.md#admin_users_list) | **Get** /admin.users.list | 
[**admin_users_remove**](AdminApi.md#admin_users_remove) | **Post** /admin.users.remove | 
[**admin_users_session_reset**](AdminApi.md#admin_users_session_reset) | **Post** /admin.users.session.reset | 
[**admin_users_set_admin**](AdminApi.md#admin_users_set_admin) | **Post** /admin.users.setAdmin | 
[**admin_users_set_expiration**](AdminApi.md#admin_users_set_expiration) | **Post** /admin.users.setExpiration | 
[**admin_users_set_owner**](AdminApi.md#admin_users_set_owner) | **Post** /admin.users.setOwner | 
[**admin_users_set_regular**](AdminApi.md#admin_users_set_regular) | **Post** /admin.users.setRegular | 


# **admin_apps_approve**
> ::std::collections::HashMap<String, Value> admin_apps_approve(ctx, token, optional)


Approve an app for installation on a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:write&#x60; | 
 **team_id** | **String**|  | 
 **app_id** | **String**| The id of the app to approve. | 
 **request_id** | **String**| The id of the request to approve. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_apps_approved_list**
> ::std::collections::HashMap<String, Value> admin_apps_approved_list(ctx, token, optional)


List approved apps for an org or workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 
 **team_id** | **String**|  | 
 **enterprise_id** | **String**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_apps_requests_list**
> ::std::collections::HashMap<String, Value> admin_apps_requests_list(ctx, token, optional)


List app requests for a team/workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 
 **team_id** | **String**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_apps_restrict**
> ::std::collections::HashMap<String, Value> admin_apps_restrict(ctx, token, optional)


Restrict an app for installation on a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:write&#x60; | 
 **team_id** | **String**|  | 
 **app_id** | **String**| The id of the app to restrict. | 
 **request_id** | **String**| The id of the request to restrict. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_apps_restricted_list**
> ::std::collections::HashMap<String, Value> admin_apps_restricted_list(ctx, token, optional)


List restricted apps for an org or workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.apps:read&#x60; | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 
 **team_id** | **String**|  | 
 **enterprise_id** | **String**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_conversations_set_teams**
> ::std::collections::HashMap<String, Value> admin_conversations_set_teams(ctx, channel_id, token, optional)


Set the workspaces in an Enterprise grid org that connect to a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **channel_id** | **String**| The encoded &#x60;channel_id&#x60; to add or remove to workspaces. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.conversations:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channel_id** | **String**| The encoded &#x60;channel_id&#x60; to add or remove to workspaces. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.conversations:write&#x60; | 
 **org_channel** | **bool**| True if channel has to be converted to an org channel | 
 **team_id** | **String**| The workspace to which the channel belongs. Omit this argument if the channel is a cross-workspace shared channel. | 
 **target_team_ids** | **String**| The list of workspaces to which the channel should be shared. Not required if the channel is being shared orgwide. Example: &#x60;[&#39;T1234&#39;, &#39;T5678&#39;]&#x60; | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_add**
> ::std::collections::HashMap<String, Value> admin_emoji_add(ctx, url, token, name)


Add an emoji.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **url** | **String**| The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be removed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_add_alias**
> ::std::collections::HashMap<String, Value> admin_emoji_add_alias(ctx, token, name, alias_for)


Add an emoji alias.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be aliased. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 
  **alias_for** | **String**| The alias of the emoji. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_list**
> ::std::collections::HashMap<String, Value> admin_emoji_list(ctx, token, optional)


List emoji for an Enterprise Grid organization.

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
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_remove**
> ::std::collections::HashMap<String, Value> admin_emoji_remove(ctx, token, name)


Remove an emoji across an Enterprise Grid organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be removed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_rename**
> ::std::collections::HashMap<String, Value> admin_emoji_rename(ctx, new_name, token, name)


Rename an emoji.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **new_name** | **String**| The new name of the emoji. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be renamed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **admin_invite_requests_approved_list**
> ::std::collections::HashMap<String, Value> admin_invite_requests_approved_list(ctx, token, optional)


List all approved workspace invite requests.

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

# **admin_invite_requests_denied_list**
> ::std::collections::HashMap<String, Value> admin_invite_requests_denied_list(ctx, token, optional)


List all denied workspace invite requests.

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
 **cursor** | **String**| Value of the &#x60;next_cursor&#x60; field sent as part of the previous api response | 
 **limit** | **i32**| The number of results that will be returned by the API on each invocation. Must be between 1 - 1000 both inclusive | 
 **team_id** | **String**| ID for the workspace where the invite requests were made. | 

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

# **admin_teams_owners_list**
> ::std::collections::HashMap<String, Value> admin_teams_owners_list(ctx, token, team_id, optional)


List all of the owners on a given workspace.

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
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_info**
> ::std::collections::HashMap<String, Value> admin_teams_settings_info(ctx, token, team_id)


Fetch information about settings in a workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
  **team_id** | **String**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_set_default_channels**
> ::std::collections::HashMap<String, Value> admin_teams_settings_set_default_channels(ctx, channel_ids, token, team_id)


Set the default channels of a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **channel_ids** | **String**| An array of channel IDs. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **team_id** | **String**| ID for the workspace to set the default channel for. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_set_description**
> ::std::collections::HashMap<String, Value> admin_teams_settings_set_description(ctx, token, team_id, description)


Set the description of a given workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **team_id** | **String**| ID for the workspace to set the description for. | 
  **description** | **String**| The new description for the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_set_discoverability**
> ::std::collections::HashMap<String, Value> admin_teams_settings_set_discoverability(ctx, token, team_id, discoverability)


An API method that allows admins to set the discoverability of a given workspace

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **team_id** | **String**| The ID of the workspace to set discoverability on. | 
  **discoverability** | **String**| This workspace&#39;s discovery setting. It must be set to one of &#x60;open&#x60;, &#x60;invite_only&#x60;, &#x60;closed&#x60;, or &#x60;unlisted&#x60;. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_set_icon**
> ::std::collections::HashMap<String, Value> admin_teams_settings_set_icon(ctx, token, image_url, team_id)


Sets the icon of a workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **image_url** | **String**| Image URL for the icon | 
  **team_id** | **String**| ID for the workspace to set the icon for. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_teams_settings_set_name**
> ::std::collections::HashMap<String, Value> admin_teams_settings_set_name(ctx, token, team_id, name)


Set the name of a given workspace.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **team_id** | **String**| ID for the workspace to set the name for. | 
  **name** | **String**| The new name of the workspace. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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


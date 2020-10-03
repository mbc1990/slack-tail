# \AdminTeamsSettingsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_settings_info**](AdminTeamsSettingsApi.md#admin_teams_settings_info) | **Get** /admin.teams.settings.info | 
[**admin_teams_settings_set_default_channels**](AdminTeamsSettingsApi.md#admin_teams_settings_set_default_channels) | **Post** /admin.teams.settings.setDefaultChannels | 
[**admin_teams_settings_set_description**](AdminTeamsSettingsApi.md#admin_teams_settings_set_description) | **Post** /admin.teams.settings.setDescription | 
[**admin_teams_settings_set_discoverability**](AdminTeamsSettingsApi.md#admin_teams_settings_set_discoverability) | **Post** /admin.teams.settings.setDiscoverability | 
[**admin_teams_settings_set_icon**](AdminTeamsSettingsApi.md#admin_teams_settings_set_icon) | **Post** /admin.teams.settings.setIcon | 
[**admin_teams_settings_set_name**](AdminTeamsSettingsApi.md#admin_teams_settings_set_name) | **Post** /admin.teams.settings.setName | 


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


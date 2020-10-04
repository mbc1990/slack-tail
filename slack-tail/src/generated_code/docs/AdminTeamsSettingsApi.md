# \AdminTeamsSettingsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_settings_info**](AdminTeamsSettingsApi.md#admin_teams_settings_info) | **get** /admin.teams.settings.info | 
[**admin_teams_settings_set_default_channels**](AdminTeamsSettingsApi.md#admin_teams_settings_set_default_channels) | **post** /admin.teams.settings.setDefaultChannels | 
[**admin_teams_settings_set_description**](AdminTeamsSettingsApi.md#admin_teams_settings_set_description) | **post** /admin.teams.settings.setDescription | 
[**admin_teams_settings_set_discoverability**](AdminTeamsSettingsApi.md#admin_teams_settings_set_discoverability) | **post** /admin.teams.settings.setDiscoverability | 
[**admin_teams_settings_set_icon**](AdminTeamsSettingsApi.md#admin_teams_settings_set_icon) | **post** /admin.teams.settings.setIcon | 
[**admin_teams_settings_set_name**](AdminTeamsSettingsApi.md#admin_teams_settings_set_name) | **post** /admin.teams.settings.setName | 



## admin_teams_settings_info

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_info(token, team_id)


Fetch information about settings in a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**team_id** | **String** |  | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_default_channels

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_set_default_channels(channel_ids, token, team_id)


Set the default channels of a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_ids** | **String** | An array of channel IDs. | [required] |
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the default channel for. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_description

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_set_description(token, team_id, description)


Set the description of a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the description for. | [required] |
**description** | **String** | The new description for the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_discoverability

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_set_discoverability(token, team_id, discoverability)


An API method that allows admins to set the discoverability of a given workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | The ID of the workspace to set discoverability on. | [required] |
**discoverability** | **String** | This workspace's discovery setting. It must be set to one of `open`, `invite_only`, `closed`, or `unlisted`. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_icon

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_set_icon(token, image_url, team_id)


Sets the icon of a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**image_url** | **String** | Image URL for the icon | [required] |
**team_id** | **String** | ID for the workspace to set the icon for. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_settings_set_name

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_settings_set_name(token, team_id, name)


Set the name of a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_id** | **String** | ID for the workspace to set the name for. | [required] |
**name** | **String** | The new name of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


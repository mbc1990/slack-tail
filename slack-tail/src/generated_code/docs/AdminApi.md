# \AdminApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_apps_approve**](AdminApi.md#admin_apps_approve) | **post** /admin.apps.approve | 
[**admin_apps_approved_list**](AdminApi.md#admin_apps_approved_list) | **get** /admin.apps.approved.list | 
[**admin_apps_requests_list**](AdminApi.md#admin_apps_requests_list) | **get** /admin.apps.requests.list | 
[**admin_apps_restrict**](AdminApi.md#admin_apps_restrict) | **post** /admin.apps.restrict | 
[**admin_apps_restricted_list**](AdminApi.md#admin_apps_restricted_list) | **get** /admin.apps.restricted.list | 
[**admin_conversations_set_teams**](AdminApi.md#admin_conversations_set_teams) | **post** /admin.conversations.setTeams | 
[**admin_emoji_add**](AdminApi.md#admin_emoji_add) | **post** /admin.emoji.add | 
[**admin_emoji_add_alias**](AdminApi.md#admin_emoji_add_alias) | **post** /admin.emoji.addAlias | 
[**admin_emoji_list**](AdminApi.md#admin_emoji_list) | **get** /admin.emoji.list | 
[**admin_emoji_remove**](AdminApi.md#admin_emoji_remove) | **post** /admin.emoji.remove | 
[**admin_emoji_rename**](AdminApi.md#admin_emoji_rename) | **post** /admin.emoji.rename | 
[**admin_invite_requests_approve**](AdminApi.md#admin_invite_requests_approve) | **post** /admin.inviteRequests.approve | 
[**admin_invite_requests_approved_list**](AdminApi.md#admin_invite_requests_approved_list) | **get** /admin.inviteRequests.approved.list | 
[**admin_invite_requests_denied_list**](AdminApi.md#admin_invite_requests_denied_list) | **get** /admin.inviteRequests.denied.list | 
[**admin_invite_requests_deny**](AdminApi.md#admin_invite_requests_deny) | **post** /admin.inviteRequests.deny | 
[**admin_invite_requests_list**](AdminApi.md#admin_invite_requests_list) | **get** /admin.inviteRequests.list | 
[**admin_teams_admins_list**](AdminApi.md#admin_teams_admins_list) | **get** /admin.teams.admins.list | 
[**admin_teams_create**](AdminApi.md#admin_teams_create) | **post** /admin.teams.create | 
[**admin_teams_list**](AdminApi.md#admin_teams_list) | **get** /admin.teams.list | 
[**admin_teams_owners_list**](AdminApi.md#admin_teams_owners_list) | **get** /admin.teams.owners.list | 
[**admin_teams_settings_info**](AdminApi.md#admin_teams_settings_info) | **get** /admin.teams.settings.info | 
[**admin_teams_settings_set_default_channels**](AdminApi.md#admin_teams_settings_set_default_channels) | **post** /admin.teams.settings.setDefaultChannels | 
[**admin_teams_settings_set_description**](AdminApi.md#admin_teams_settings_set_description) | **post** /admin.teams.settings.setDescription | 
[**admin_teams_settings_set_discoverability**](AdminApi.md#admin_teams_settings_set_discoverability) | **post** /admin.teams.settings.setDiscoverability | 
[**admin_teams_settings_set_icon**](AdminApi.md#admin_teams_settings_set_icon) | **post** /admin.teams.settings.setIcon | 
[**admin_teams_settings_set_name**](AdminApi.md#admin_teams_settings_set_name) | **post** /admin.teams.settings.setName | 
[**admin_users_assign**](AdminApi.md#admin_users_assign) | **post** /admin.users.assign | 
[**admin_users_invite**](AdminApi.md#admin_users_invite) | **post** /admin.users.invite | 
[**admin_users_list**](AdminApi.md#admin_users_list) | **get** /admin.users.list | 
[**admin_users_remove**](AdminApi.md#admin_users_remove) | **post** /admin.users.remove | 
[**admin_users_session_reset**](AdminApi.md#admin_users_session_reset) | **post** /admin.users.session.reset | 
[**admin_users_set_admin**](AdminApi.md#admin_users_set_admin) | **post** /admin.users.setAdmin | 
[**admin_users_set_expiration**](AdminApi.md#admin_users_set_expiration) | **post** /admin.users.setExpiration | 
[**admin_users_set_owner**](AdminApi.md#admin_users_set_owner) | **post** /admin.users.setOwner | 
[**admin_users_set_regular**](AdminApi.md#admin_users_set_regular) | **post** /admin.users.setRegular | 



## admin_apps_approve

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_approve(token, team_id, app_id, request_id)


Approve an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**team_id** | Option<**String**> |  |  |
**app_id** | Option<**String**> | The id of the app to approve. |  |
**request_id** | Option<**String**> | The id of the request to approve. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_approved_list

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_approved_list(token, cursor, limit, team_id, enterprise_id)


List approved apps for an org or workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**team_id** | Option<**String**> |  |  |
**enterprise_id** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_requests_list

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_requests_list(token, cursor, limit, team_id)


List app requests for a team/workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**team_id** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_restrict

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_restrict(token, team_id, app_id, request_id)


Restrict an app for installation on a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:write` | [required] |
**team_id** | Option<**String**> |  |  |
**app_id** | Option<**String**> | The id of the app to restrict. |  |
**request_id** | Option<**String**> | The id of the request to restrict. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_apps_restricted_list

> ::std::collections::HashMap<String, serde_json::Value> admin_apps_restricted_list(token, cursor, limit, team_id, enterprise_id)


List restricted apps for an org or workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.apps:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |
**team_id** | Option<**String**> |  |  |
**enterprise_id** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_conversations_set_teams

> ::std::collections::HashMap<String, serde_json::Value> admin_conversations_set_teams(token, channel_id, org_channel, team_id, target_team_ids)


Set the workspaces in an Enterprise grid org that connect to a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.conversations:write` | [required] |
**channel_id** | **String** | The encoded `channel_id` to add or remove to workspaces. | [required] |
**org_channel** | Option<**bool**> | True if channel has to be converted to an org channel |  |
**team_id** | Option<**String**> | The workspace to which the channel belongs. Omit this argument if the channel is a cross-workspace shared channel. |  |
**target_team_ids** | Option<**String**> | The list of workspaces to which the channel should be shared. Not required if the channel is being shared orgwide. Example: `['T1234', 'T5678']` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_add

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_add(url, token, name)


Add an emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best. | [required] |
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_add_alias

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_add_alias(token, name, alias_for)


Add an emoji alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be aliased. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |
**alias_for** | **String** | The alias of the emoji. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_list

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_list(token, cursor, limit)


List emoji for an Enterprise Grid organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_remove

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_remove(token, name)


Remove an emoji across an Enterprise Grid organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_rename

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_rename(new_name, token, name)


Rename an emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_name** | **String** | The new name of the emoji. | [required] |
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be renamed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_approve

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_approve(token, invite_request_id, team_id)


Approve a workspace invite request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:write` | [required] |
**invite_request_id** | **String** | ID of the request to invite. | [required] |
**team_id** | Option<**String**> | ID for the workspace where the invite request was made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_approved_list

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_approved_list(token, cursor, limit, team_id)


List all approved workspace invite requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:read` | [required] |
**cursor** | Option<**String**> | Value of the `next_cursor` field sent as part of the previous API response |  |
**limit** | Option<**i32**> | The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive |  |
**team_id** | Option<**String**> | ID for the workspace where the invite requests were made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_denied_list

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_denied_list(token, cursor, limit, team_id)


List all denied workspace invite requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:read` | [required] |
**cursor** | Option<**String**> | Value of the `next_cursor` field sent as part of the previous api response |  |
**limit** | Option<**i32**> | The number of results that will be returned by the API on each invocation. Must be between 1 - 1000 both inclusive |  |
**team_id** | Option<**String**> | ID for the workspace where the invite requests were made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_deny

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_deny(token, invite_request_id, team_id)


Deny a workspace invite request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:write` | [required] |
**invite_request_id** | **String** | ID of the request to invite. | [required] |
**team_id** | Option<**String**> | ID for the workspace where the invite request was made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_list

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_list(token, cursor, limit, team_id)


List all pending workspace invite requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:read` | [required] |
**cursor** | Option<**String**> | Value of the `next_cursor` field sent as part of the previous API response |  |
**limit** | Option<**i32**> | The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive |  |
**team_id** | Option<**String**> | ID for the workspace where the invite requests were made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_admins_list

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_admins_list(token, team_id, cursor, limit)


List all of the admins on a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**team_id** | **String** |  | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_create

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_create(token, team_domain, team_name, team_description, team_discoverability)


Create an Enterprise team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**team_domain** | **String** | Team domain (for example, slacksoftballteam). | [required] |
**team_name** | **String** | Team name (for example, Slack Softball Team). | [required] |
**team_description** | Option<**String**> | Description for the team. |  |
**team_discoverability** | Option<**String**> | Who can join the team. A team's discoverability can be `open`, `closed`, `invite_only`, or `unlisted`. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_list

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_list(token, cursor, limit)


List all teams on an Enterprise organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 100 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_teams_owners_list

> ::std::collections::HashMap<String, serde_json::Value> admin_teams_owners_list(token, team_id, cursor, limit)


List all of the owners on a given workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**team_id** | **String** |  | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## admin_users_assign

> ::std::collections::HashMap<String, serde_json::Value> admin_users_assign(token, user_id, team_id, channel_ids, is_ultra_restricted, is_restricted)


Add an Enterprise user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to add to the workspace. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**channel_ids** | Option<**String**> | Comma separated values of channel IDs to add user in the new workspace. |  |
**is_ultra_restricted** | Option<**bool**> | True if user should be added to the workspace as a single-channel guest. |  |
**is_restricted** | Option<**bool**> | True if user should be added to the workspace as a guest. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_invite

> ::std::collections::HashMap<String, serde_json::Value> admin_users_invite(token, channel_ids, team_id, email, real_name, is_ultra_restricted, custom_message, is_restricted, guest_expiration_ts, resend)


Invite a user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**channel_ids** | **String** | A comma-separated list of `channel_id`s for this user to join. At least one channel is required. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**email** | **String** | The email address of the person to invite. | [required] |
**real_name** | Option<**String**> | Full name of the user. |  |
**is_ultra_restricted** | Option<**bool**> | Is this user a single channel guest user? (default: false) |  |
**custom_message** | Option<**String**> | An optional message to send to the user in the invite email. |  |
**is_restricted** | Option<**bool**> | Is this user a multi-channel guest user? (default: false) |  |
**guest_expiration_ts** | Option<**String**> | Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date. |  |
**resend** | Option<**bool**> | Allow this invite to be resent in the future if a user has not signed up yet. (default: false) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_list

> ::std::collections::HashMap<String, serde_json::Value> admin_users_list(token, team_id, cursor, limit)


List users on a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:read` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | Limit for how many users to be retrieved per page |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_remove

> ::std::collections::HashMap<String, serde_json::Value> admin_users_remove(token, user_id, team_id)


Remove a user from a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to remove. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_session_reset

> ::std::collections::HashMap<String, serde_json::Value> admin_users_session_reset(token, user_id, mobile_only, web_only)


Wipes all valid sessions on all devices for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to wipe sessions for | [required] |
**mobile_only** | Option<**bool**> | Only expire mobile sessions (default: false) |  |
**web_only** | Option<**bool**> | Only expire web sessions (default: false) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_admin

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_admin(token, user_id, team_id)


Set an existing guest, regular user, or owner to be an admin user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to designate as an admin. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_expiration

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_expiration(token, expiration_ts, user_id, team_id)


Set an expiration for a guest user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**expiration_ts** | **i32** | Timestamp when guest account should be disabled. | [required] |
**user_id** | **String** | The ID of the user to set an expiration for. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_owner

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_owner(token, user_id, team_id)


Set an existing guest, regular user, or admin user to be a workspace owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | Id of the user to promote to owner. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_regular

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_regular(token, user_id, team_id)


Set an existing guest user, admin user, or owner to be a regular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to designate as a regular user. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


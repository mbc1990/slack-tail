# \AdminTeamsAdminsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_teams_admins_list**](AdminTeamsAdminsApi.md#admin_teams_admins_list) | **get** /admin.teams.admins.list | 



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


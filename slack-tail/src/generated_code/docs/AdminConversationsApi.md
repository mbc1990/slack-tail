# \AdminConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_conversations_set_teams**](AdminConversationsApi.md#admin_conversations_set_teams) | **post** /admin.conversations.setTeams | 



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


# \AdminConversationsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_conversations_set_teams**](AdminConversationsApi.md#admin_conversations_set_teams) | **Post** /admin.conversations.setTeams | 


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


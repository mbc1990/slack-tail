# \ChatScheduledMessagesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**chat_scheduled_messages_list**](ChatScheduledMessagesApi.md#chat_scheduled_messages_list) | **Get** /chat.scheduledMessages.list | 


# **chat_scheduled_messages_list**
> ::std::collections::HashMap<String, Value> chat_scheduled_messages_list(ctx, optional)


Returns a list of scheduled messages.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cursor** | **String**| For pagination purposes, this is the &#x60;cursor&#x60; value returned from a previous call to &#x60;chat.scheduledmessages.list&#x60; indicating where you want to start this call from. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **limit** | **i32**| Maximum number of original entries to return. | 
 **oldest** | **f32**| A UNIX timestamp of the oldest value in the time range | 
 **channel** | **String**| The channel of the scheduled messages | 
 **latest** | **f32**| A UNIX timestamp of the latest value in the time range | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DialogApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dialog_open**](DialogApi.md#dialog_open) | **Get** /dialog.open | 


# **dialog_open**
> ::std::collections::HashMap<String, Value> dialog_open(ctx, token, trigger_id, dialog)


Open a dialog with a user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **trigger_id** | **String**| Exchange a trigger to post to the user. | 
  **dialog** | **String**| The dialog definition. This must be a JSON-encoded string. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


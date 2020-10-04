# \DialogApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dialog_open**](DialogApi.md#dialog_open) | **get** /dialog.open | 



## dialog_open

> ::std::collections::HashMap<String, serde_json::Value> dialog_open(token, trigger_id, dialog)


Open a dialog with a user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `none` | [required] |
**trigger_id** | **String** | Exchange a trigger to post to the user. | [required] |
**dialog** | **String** | The dialog definition. This must be a JSON-encoded string. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


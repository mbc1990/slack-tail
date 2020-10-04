# \OauthV2Api

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth_v2_access**](OauthV2Api.md#oauth_v2_access) | **get** /oauth.v2.access | 



## oauth_v2_access

> ::std::collections::HashMap<String, serde_json::Value> oauth_v2_access(code, client_secret, client_id, redirect_uri)


Exchanges a temporary OAuth verifier code for an access token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | The `code` param returned via the OAuth callback. | [required] |
**client_secret** | Option<**String**> | Issued when you created your application. |  |
**client_id** | Option<**String**> | Issued when you created your application. |  |
**redirect_uri** | Option<**String**> | This must match the originally submitted URI (if one was sent). |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


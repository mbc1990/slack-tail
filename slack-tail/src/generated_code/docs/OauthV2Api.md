# \OauthV2Api

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth_v2_access**](OauthV2Api.md#oauth_v2_access) | **Get** /oauth.v2.access | 


# **oauth_v2_access**
> ::std::collections::HashMap<String, Value> oauth_v2_access(ctx, code, optional)


Exchanges a temporary OAuth verifier code for an access token.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **code** | **String**| The &#x60;code&#x60; param returned via the OAuth callback. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **code** | **String**| The &#x60;code&#x60; param returned via the OAuth callback. | 
 **client_secret** | **String**| Issued when you created your application. | 
 **client_id** | **String**| Issued when you created your application. | 
 **redirect_uri** | **String**| This must match the originally submitted URI (if one was sent). | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


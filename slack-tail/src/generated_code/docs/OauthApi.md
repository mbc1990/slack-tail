# \OauthApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**oauth_access**](OauthApi.md#oauth_access) | **Get** /oauth.access | 
[**oauth_token**](OauthApi.md#oauth_token) | **Get** /oauth.token | 
[**oauth_v2_access**](OauthApi.md#oauth_v2_access) | **Get** /oauth.v2.access | 


# **oauth_access**
> ::std::collections::HashMap<String, Value> oauth_access(ctx, optional)


Exchanges a temporary OAuth verifier code for an access token.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **code** | **String**| The &#x60;code&#x60; param returned via the OAuth callback. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **redirect_uri** | **String**| This must match the originally submitted URI (if one was sent). | 
 **single_channel** | **bool**| Request the user to add your app only to a single channel. Only valid with a [legacy workspace app](https://api.slack.com/legacy-workspace-apps). | 
 **client_id** | **String**| Issued when you created your application. | 
 **client_secret** | **String**| Issued when you created your application. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **oauth_token**
> ::std::collections::HashMap<String, Value> oauth_token(ctx, optional)


Exchanges a temporary OAuth verifier code for a workspace token.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **client_secret** | **String**| Issued when you created your application. | 
 **code** | **String**| The &#x60;code&#x60; param returned via the OAuth callback. | 
 **single_channel** | **bool**| Request the user to add your app only to a single channel. | 
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


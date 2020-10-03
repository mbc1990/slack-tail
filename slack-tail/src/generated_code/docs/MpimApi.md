# \MpimApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**mpim_close**](MpimApi.md#mpim_close) | **Post** /mpim.close | 
[**mpim_history**](MpimApi.md#mpim_history) | **Get** /mpim.history | 
[**mpim_list**](MpimApi.md#mpim_list) | **Get** /mpim.list | 
[**mpim_mark**](MpimApi.md#mpim_mark) | **Post** /mpim.mark | 
[**mpim_open**](MpimApi.md#mpim_open) | **Post** /mpim.open | 
[**mpim_replies**](MpimApi.md#mpim_replies) | **Get** /mpim.replies | 


# **mpim_close**
> ::std::collections::HashMap<String, Value> mpim_close(ctx, token, channel)


Closes a multiparty direct message channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;mpim:write&#x60; | 
  **channel** | **String**| MPIM to close. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mpim_history**
> ::std::collections::HashMap<String, Value> mpim_history(ctx, optional)


Fetches history of messages and events from a multiparty direct message.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **i32**| Number of messages to return, between 1 and 1000. | 
 **unreads** | **bool**| Include &#x60;unread_count_display&#x60; in the output? | 
 **inclusive** | **bool**| Include messages with latest or oldest timestamp in results. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;mpim:history&#x60; | 
 **oldest** | **f32**| Start of time range of messages to include in results. | 
 **channel** | **String**| Multiparty direct message to fetch history for. | 
 **latest** | **f32**| End of time range of messages to include in results. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mpim_list**
> ::std::collections::HashMap<String, Value> mpim_list(ctx, optional)


Lists multiparty direct message channels for the calling user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cursor** | **String**| Parameter for pagination. Set &#x60;cursor&#x60; equal to the &#x60;next_cursor&#x60; attribute returned by the previous request&#39;s &#x60;response_metadata&#x60;. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \&quot;page\&quot; of the collection. See [pagination](/docs/pagination) for more details. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;mpim:read&#x60; | 
 **limit** | **i32**| The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn&#39;t been reached. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mpim_mark**
> ::std::collections::HashMap<String, Value> mpim_mark(ctx, optional)


Sets the read cursor in a multiparty direct message channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;mpim:write&#x60; | 
 **ts** | **f32**| Timestamp of the most recently seen message. | 
 **channel** | **String**| multiparty direct message channel to set reading cursor in. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mpim_open**
> ::std::collections::HashMap<String, Value> mpim_open(ctx, optional)


This method opens a multiparty direct message.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;mpim:write&#x60; | 
 **users** | **String**| Comma separated lists of users.  The ordering of the users is preserved whenever a MPIM group is returned. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **mpim_replies**
> ::std::collections::HashMap<String, Value> mpim_replies(ctx, optional)


Retrieve a thread of messages posted to a direct message conversation from a multiparty direct message.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **thread_ts** | **f32**| Unique identifier of a thread&#39;s parent message. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;mpim:history&#x60; | 
 **channel** | **String**| Multiparty direct message channel to fetch thread from. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


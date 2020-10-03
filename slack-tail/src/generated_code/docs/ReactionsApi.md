# \ReactionsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reactions_add**](ReactionsApi.md#reactions_add) | **Post** /reactions.add | 
[**reactions_get**](ReactionsApi.md#reactions_get) | **Get** /reactions.get | 
[**reactions_list**](ReactionsApi.md#reactions_list) | **Get** /reactions.list | 
[**reactions_remove**](ReactionsApi.md#reactions_remove) | **Post** /reactions.remove | 


# **reactions_add**
> ::std::collections::HashMap<String, Value> reactions_add(ctx, timestamp, token, name, channel)


Adds a reaction to an item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **timestamp** | **String**| Timestamp of the message to add reaction to. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;reactions:write&#x60; | 
  **name** | **String**| Reaction (emoji) name. | 
  **channel** | **String**| Channel where the message to add reaction to was posted. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reactions_get**
> Value reactions_get(ctx, token, optional)


Gets reactions for an item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;reactions:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;reactions:read&#x60; | 
 **full** | **bool**| If true always return the complete reaction list. | 
 **file_comment** | **String**| File comment to get reactions for. | 
 **timestamp** | **String**| Timestamp of the message to get reactions for. | 
 **file** | **String**| File to get reactions for. | 
 **channel** | **String**| Channel where the message to get reactions for was posted. | 

### Return type

[**Value**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reactions_list**
> ::std::collections::HashMap<String, Value> reactions_list(ctx, token, optional)


Lists reactions made by a user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;reactions:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;reactions:read&#x60; | 
 **count** | **i32**|  | 
 **full** | **bool**| If true always return the complete reaction list. | 
 **cursor** | **String**| Parameter for pagination. Set &#x60;cursor&#x60; equal to the &#x60;next_cursor&#x60; attribute returned by the previous request&#39;s &#x60;response_metadata&#x60;. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \&quot;page\&quot; of the collection. See [pagination](/docs/pagination) for more details. | 
 **limit** | **i32**| The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn&#39;t been reached. | 
 **user** | **String**| Show reactions made by this user. Defaults to the authed user. | 
 **page** | **i32**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **reactions_remove**
> ::std::collections::HashMap<String, Value> reactions_remove(ctx, name, token, optional)


Removes a reaction from an item.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **name** | **String**| Reaction (emoji) name. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;reactions:write&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **name** | **String**| Reaction (emoji) name. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;reactions:write&#x60; | 
 **file_comment** | **String**| File comment to remove reaction from. | 
 **timestamp** | **String**| Timestamp of the message to remove reaction from. | 
 **file** | **String**| File to remove reaction from. | 
 **channel** | **String**| Channel where the message to remove reaction from was posted. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ViewsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**views_open**](ViewsApi.md#views_open) | **Get** /views.open | 
[**views_publish**](ViewsApi.md#views_publish) | **Get** /views.publish | 
[**views_push**](ViewsApi.md#views_push) | **Get** /views.push | 
[**views_update**](ViewsApi.md#views_update) | **Get** /views.update | 


# **views_open**
> ::std::collections::HashMap<String, Value> views_open(ctx, token, trigger_id, view)


Open a view for a user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **trigger_id** | **String**| Exchange a trigger to post to the user. | 
  **view** | **String**| A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **views_publish**
> ::std::collections::HashMap<String, Value> views_publish(ctx, token, user_id, view, optional)


Publish a static view for a User.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **user_id** | **String**| &#x60;id&#x60; of the user you want publish a view to. | 
  **view** | **String**| A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **user_id** | **String**| &#x60;id&#x60; of the user you want publish a view to. | 
 **view** | **String**| A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | 
 **hash** | **String**| A string that represents view state to protect against possible race conditions. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **views_push**
> ::std::collections::HashMap<String, Value> views_push(ctx, token, trigger_id, view)


Push a view onto the stack of a root view.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
  **trigger_id** | **String**| Exchange a trigger to post to the user. | 
  **view** | **String**| A [view payload](/reference/surfaces/views). This must be a JSON-encoded string. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **views_update**
> ::std::collections::HashMap<String, Value> views_update(ctx, token, optional)


Update an existing view.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;none&#x60; | 
 **hash** | **String**| A string that represents view state to protect against possible race conditions. | 
 **view_id** | **String**| A unique identifier of the view to be updated. Either &#x60;view_id&#x60; or &#x60;external_id&#x60; is required. | 
 **external_id** | **String**| A unique identifier of the view set by the developer. Must be unique for all views on a team. Max length of 255 characters. Either &#x60;view_id&#x60; or &#x60;external_id&#x60; is required. | 
 **view** | **String**| A [view payload](/reference/surfaces/views) This must be a JSON-encoded string. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


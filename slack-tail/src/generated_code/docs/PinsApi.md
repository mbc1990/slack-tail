# \PinsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pins_add**](PinsApi.md#pins_add) | **Post** /pins.add | 
[**pins_list**](PinsApi.md#pins_list) | **Get** /pins.list | 
[**pins_remove**](PinsApi.md#pins_remove) | **Post** /pins.remove | 


# **pins_add**
> ::std::collections::HashMap<String, Value> pins_add(ctx, optional)


Pins an item to a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **timestamp** | **f32**| Timestamp of the message to pin. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;pins:write&#x60; | 
 **channel** | **String**| Channel to pin the item in. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pins_list**
> Value pins_list(ctx, optional)


Lists items pinned to a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;pins:read&#x60; | 
 **channel** | **String**| Channel to get pinned items for. | 

### Return type

[**Value**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **pins_remove**
> ::std::collections::HashMap<String, Value> pins_remove(ctx, optional)


Un-pins an item from a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file_comment** | **String**| File comment to un-pin. | 
 **timestamp** | **f32**| Timestamp of the message to un-pin. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;pins:write&#x60; | 
 **file** | **String**| File to un-pin. | 
 **channel** | **String**| Channel where the item is pinned to. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


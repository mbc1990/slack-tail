# \SearchApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_messages**](SearchApi.md#search_messages) | **Get** /search.messages | 


# **search_messages**
> ::std::collections::HashMap<String, Value> search_messages(ctx, query, token, optional)


Searches for messages matching a query.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **query** | **String**| Search query. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;search:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **query** | **String**| Search query. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;search:read&#x60; | 
 **sort_dir** | **String**| Change sort direction to ascending (&#x60;asc&#x60;) or descending (&#x60;desc&#x60;). | 
 **sort** | **String**| Return matches sorted by either &#x60;score&#x60; or &#x60;timestamp&#x60;. | 
 **count** | **i32**| Pass the number of results you want per \&quot;page\&quot;. Maximum of &#x60;100&#x60;. | 
 **highlight** | **bool**| Pass a value of &#x60;true&#x60; to enable query highlight markers (see below). | 
 **page** | **i32**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


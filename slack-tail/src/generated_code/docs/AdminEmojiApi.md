# \AdminEmojiApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_emoji_add**](AdminEmojiApi.md#admin_emoji_add) | **Post** /admin.emoji.add | 
[**admin_emoji_add_alias**](AdminEmojiApi.md#admin_emoji_add_alias) | **Post** /admin.emoji.addAlias | 
[**admin_emoji_list**](AdminEmojiApi.md#admin_emoji_list) | **Get** /admin.emoji.list | 
[**admin_emoji_remove**](AdminEmojiApi.md#admin_emoji_remove) | **Post** /admin.emoji.remove | 
[**admin_emoji_rename**](AdminEmojiApi.md#admin_emoji_rename) | **Post** /admin.emoji.rename | 


# **admin_emoji_add**
> ::std::collections::HashMap<String, Value> admin_emoji_add(ctx, url, token, name)


Add an emoji.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **url** | **String**| The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be removed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_add_alias**
> ::std::collections::HashMap<String, Value> admin_emoji_add_alias(ctx, token, name, alias_for)


Add an emoji alias.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be aliased. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 
  **alias_for** | **String**| The alias of the emoji. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_list**
> ::std::collections::HashMap<String, Value> admin_emoji_list(ctx, token, optional)


List emoji for an Enterprise Grid organization.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:read&#x60; | 
 **cursor** | **String**| Set &#x60;cursor&#x60; to &#x60;next_cursor&#x60; returned by the previous call to list items in the next page | 
 **limit** | **i32**| The maximum number of items to return. Must be between 1 - 1000 both inclusive. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_remove**
> ::std::collections::HashMap<String, Value> admin_emoji_remove(ctx, token, name)


Remove an emoji across an Enterprise Grid organization

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be removed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **admin_emoji_rename**
> ::std::collections::HashMap<String, Value> admin_emoji_rename(ctx, new_name, token, name)


Rename an emoji.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **new_name** | **String**| The new name of the emoji. | 
  **token** | **String**| Authentication token. Requires scope: &#x60;admin.teams:write&#x60; | 
  **name** | **String**| The name of the emoji to be renamed. Colons (&#x60;:myemoji:&#x60;) around the value are not required, although they may be included. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


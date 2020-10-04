# \AdminEmojiApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_emoji_add**](AdminEmojiApi.md#admin_emoji_add) | **post** /admin.emoji.add | 
[**admin_emoji_add_alias**](AdminEmojiApi.md#admin_emoji_add_alias) | **post** /admin.emoji.addAlias | 
[**admin_emoji_list**](AdminEmojiApi.md#admin_emoji_list) | **get** /admin.emoji.list | 
[**admin_emoji_remove**](AdminEmojiApi.md#admin_emoji_remove) | **post** /admin.emoji.remove | 
[**admin_emoji_rename**](AdminEmojiApi.md#admin_emoji_rename) | **post** /admin.emoji.rename | 



## admin_emoji_add

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_add(url, token, name)


Add an emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**url** | **String** | The URL of a file to use as an image for the emoji. Square images under 128KB and with transparent backgrounds work best. | [required] |
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_add_alias

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_add_alias(token, name, alias_for)


Add an emoji alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be aliased. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |
**alias_for** | **String** | The alias of the emoji. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_list

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_list(token, cursor, limit)


List emoji for an Enterprise Grid organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:read` | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page |  |
**limit** | Option<**i32**> | The maximum number of items to return. Must be between 1 - 1000 both inclusive. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_remove

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_remove(token, name)


Remove an emoji across an Enterprise Grid organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be removed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_emoji_rename

> ::std::collections::HashMap<String, serde_json::Value> admin_emoji_rename(new_name, token, name)


Rename an emoji.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_name** | **String** | The new name of the emoji. | [required] |
**token** | **String** | Authentication token. Requires scope: `admin.teams:write` | [required] |
**name** | **String** | The name of the emoji to be renamed. Colons (`:myemoji:`) around the value are not required, although they may be included. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


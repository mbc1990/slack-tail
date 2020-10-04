# \UsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**users_conversations**](UsersApi.md#users_conversations) | **get** /users.conversations | 
[**users_delete_photo**](UsersApi.md#users_delete_photo) | **post** /users.deletePhoto | 
[**users_get_presence**](UsersApi.md#users_get_presence) | **get** /users.getPresence | 
[**users_identity**](UsersApi.md#users_identity) | **get** /users.identity | 
[**users_info**](UsersApi.md#users_info) | **get** /users.info | 
[**users_list**](UsersApi.md#users_list) | **get** /users.list | 
[**users_lookup_by_email**](UsersApi.md#users_lookup_by_email) | **get** /users.lookupByEmail | 
[**users_profile_get**](UsersApi.md#users_profile_get) | **get** /users.profile.get | 
[**users_profile_set**](UsersApi.md#users_profile_set) | **post** /users.profile.set | 
[**users_set_active**](UsersApi.md#users_set_active) | **post** /users.setActive | 
[**users_set_photo**](UsersApi.md#users_set_photo) | **post** /users.setPhoto | 
[**users_set_presence**](UsersApi.md#users_set_presence) | **post** /users.setPresence | 



## users_conversations

> ::std::collections::HashMap<String, serde_json::Value> users_conversations(cursor, token, limit, user, exclude_archived, types)


List conversations the calling user may access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `conversations:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000. |  |
**user** | Option<**String**> | Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership. |  |
**exclude_archived** | Option<**bool**> | Set to `true` to exclude archived channels from the list |  |
**types** | Option<**String**> | Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_delete_photo

> ::std::collections::HashMap<String, serde_json::Value> users_delete_photo(token)


Delete the user profile photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users.profile:write` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_get_presence

> ::std::collections::HashMap<String, serde_json::Value> users_get_presence(token, user)


Gets user presence information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read` | [required] |
**user** | Option<**String**> | User to get presence info on. Defaults to the authed user. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_identity

> serde_json::Value users_identity(token)


Get a user's identity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `identity.basic` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_info

> ::std::collections::HashMap<String, serde_json::Value> users_info(token, user, include_locale)


Gets information about a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:read` | [required] |
**user** | Option<**String**> | User to get info on |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for this user. Defaults to `false` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_list

> ::std::collections::HashMap<String, serde_json::Value> users_list(cursor, token, limit, include_locale)


Lists all users in a Slack team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `users:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. |  |
**include_locale** | Option<**bool**> | Set this to `true` to receive the locale for users. Defaults to `false` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_lookup_by_email

> ::std::collections::HashMap<String, serde_json::Value> users_lookup_by_email(token, email)


Find a user with an email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `users:read.email` |  |
**email** | Option<**String**> | An email address belonging to a user in the workspace |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_profile_get

> ::std::collections::HashMap<String, serde_json::Value> users_profile_get(token, include_labels, user)


Retrieves a user's profile information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `users.profile:read` |  |
**include_labels** | Option<**bool**> | Include labels for each ID in custom profile fields |  |
**user** | Option<**String**> | User to retrieve profile info for |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_profile_set

> ::std::collections::HashMap<String, serde_json::Value> users_profile_set(token, profile, user, value, name)


Set the profile information for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `users.profile:write` |  |
**profile** | Option<**String**> | Collection of key:value pairs presented as a URL-encoded JSON hash. At most 50 fields may be set. Each field name is limited to 255 characters. |  |
**user** | Option<**String**> | ID of user to change. This argument may only be specified by team admins on paid teams. |  |
**value** | Option<**String**> | Value to set a single key to. Usable only if `profile` is not passed. |  |
**name** | Option<**String**> | Name of a single key to set. Usable only if `profile` is not passed. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_set_active

> ::std::collections::HashMap<String, serde_json::Value> users_set_active(token)


Marked a user as active. Deprecated and non-functional.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:write` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_set_photo

> ::std::collections::HashMap<String, serde_json::Value> users_set_photo(image, crop_w, token, crop_y, crop_x)


Set the user profile photo

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image** | Option<**String**> | File contents via `multipart/form-data`. |  |
**crop_w** | Option<**i32**> | Width/height of crop box (always square) |  |
**token** | Option<**String**> | Authentication token. Requires scope: `users.profile:write` |  |
**crop_y** | Option<**i32**> | Y coordinate of top-left corner of crop box |  |
**crop_x** | Option<**i32**> | X coordinate of top-left corner of crop box |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_set_presence

> ::std::collections::HashMap<String, serde_json::Value> users_set_presence(token, presence)


Manually sets user presence.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `users:write` | [required] |
**presence** | **String** | Either `auto` or `away` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


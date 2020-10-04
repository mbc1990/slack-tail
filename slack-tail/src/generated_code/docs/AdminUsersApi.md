# \AdminUsersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_users_assign**](AdminUsersApi.md#admin_users_assign) | **post** /admin.users.assign | 
[**admin_users_invite**](AdminUsersApi.md#admin_users_invite) | **post** /admin.users.invite | 
[**admin_users_list**](AdminUsersApi.md#admin_users_list) | **get** /admin.users.list | 
[**admin_users_remove**](AdminUsersApi.md#admin_users_remove) | **post** /admin.users.remove | 
[**admin_users_set_admin**](AdminUsersApi.md#admin_users_set_admin) | **post** /admin.users.setAdmin | 
[**admin_users_set_expiration**](AdminUsersApi.md#admin_users_set_expiration) | **post** /admin.users.setExpiration | 
[**admin_users_set_owner**](AdminUsersApi.md#admin_users_set_owner) | **post** /admin.users.setOwner | 
[**admin_users_set_regular**](AdminUsersApi.md#admin_users_set_regular) | **post** /admin.users.setRegular | 



## admin_users_assign

> ::std::collections::HashMap<String, serde_json::Value> admin_users_assign(token, user_id, team_id, channel_ids, is_ultra_restricted, is_restricted)


Add an Enterprise user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to add to the workspace. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**channel_ids** | Option<**String**> | Comma separated values of channel IDs to add user in the new workspace. |  |
**is_ultra_restricted** | Option<**bool**> | True if user should be added to the workspace as a single-channel guest. |  |
**is_restricted** | Option<**bool**> | True if user should be added to the workspace as a guest. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_invite

> ::std::collections::HashMap<String, serde_json::Value> admin_users_invite(token, channel_ids, team_id, email, real_name, is_ultra_restricted, custom_message, is_restricted, guest_expiration_ts, resend)


Invite a user to a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**channel_ids** | **String** | A comma-separated list of `channel_id`s for this user to join. At least one channel is required. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**email** | **String** | The email address of the person to invite. | [required] |
**real_name** | Option<**String**> | Full name of the user. |  |
**is_ultra_restricted** | Option<**bool**> | Is this user a single channel guest user? (default: false) |  |
**custom_message** | Option<**String**> | An optional message to send to the user in the invite email. |  |
**is_restricted** | Option<**bool**> | Is this user a multi-channel guest user? (default: false) |  |
**guest_expiration_ts** | Option<**String**> | Timestamp when guest account should be disabled. Only include this timestamp if you are inviting a guest user and you want their account to expire on a certain date. |  |
**resend** | Option<**bool**> | Allow this invite to be resent in the future if a user has not signed up yet. (default: false) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_list

> ::std::collections::HashMap<String, serde_json::Value> admin_users_list(token, team_id, cursor, limit)


List users on a workspace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:read` | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |
**cursor** | Option<**String**> | Set `cursor` to `next_cursor` returned by the previous call to list items in the next page. |  |
**limit** | Option<**i32**> | Limit for how many users to be retrieved per page |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_remove

> ::std::collections::HashMap<String, serde_json::Value> admin_users_remove(token, user_id, team_id)


Remove a user from a workspace.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to remove. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_admin

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_admin(token, user_id, team_id)


Set an existing guest, regular user, or owner to be an admin user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to designate as an admin. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_expiration

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_expiration(token, expiration_ts, user_id, team_id)


Set an expiration for a guest user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**expiration_ts** | **i32** | Timestamp when guest account should be disabled. | [required] |
**user_id** | **String** | The ID of the user to set an expiration for. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_owner

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_owner(token, user_id, team_id)


Set an existing guest, regular user, or admin user to be a workspace owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | Id of the user to promote to owner. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_users_set_regular

> ::std::collections::HashMap<String, serde_json::Value> admin_users_set_regular(token, user_id, team_id)


Set an existing guest user, admin user, or owner to be a regular user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.users:write` | [required] |
**user_id** | **String** | The ID of the user to designate as a regular user. | [required] |
**team_id** | **String** | The ID (`T1234`) of the workspace. | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


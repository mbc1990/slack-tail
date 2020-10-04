# \AdminInviteRequestsApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admin_invite_requests_approve**](AdminInviteRequestsApi.md#admin_invite_requests_approve) | **post** /admin.inviteRequests.approve | 
[**admin_invite_requests_deny**](AdminInviteRequestsApi.md#admin_invite_requests_deny) | **post** /admin.inviteRequests.deny | 
[**admin_invite_requests_list**](AdminInviteRequestsApi.md#admin_invite_requests_list) | **get** /admin.inviteRequests.list | 



## admin_invite_requests_approve

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_approve(token, invite_request_id, team_id)


Approve a workspace invite request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:write` | [required] |
**invite_request_id** | **String** | ID of the request to invite. | [required] |
**team_id** | Option<**String**> | ID for the workspace where the invite request was made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_deny

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_deny(token, invite_request_id, team_id)


Deny a workspace invite request.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:write` | [required] |
**invite_request_id** | **String** | ID of the request to invite. | [required] |
**team_id** | Option<**String**> | ID for the workspace where the invite request was made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admin_invite_requests_list

> ::std::collections::HashMap<String, serde_json::Value> admin_invite_requests_list(token, cursor, limit, team_id)


List all pending workspace invite requests.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `admin.invites:read` | [required] |
**cursor** | Option<**String**> | Value of the `next_cursor` field sent as part of the previous API response |  |
**limit** | Option<**i32**> | The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive |  |
**team_id** | Option<**String**> | ID for the workspace where the invite requests were made. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


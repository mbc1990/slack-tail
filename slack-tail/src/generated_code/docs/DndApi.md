# \DndApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**dnd_end_dnd**](DndApi.md#dnd_end_dnd) | **post** /dnd.endDnd | 
[**dnd_end_snooze**](DndApi.md#dnd_end_snooze) | **post** /dnd.endSnooze | 
[**dnd_info**](DndApi.md#dnd_info) | **get** /dnd.info | 
[**dnd_set_snooze**](DndApi.md#dnd_set_snooze) | **post** /dnd.setSnooze | 
[**dnd_team_info**](DndApi.md#dnd_team_info) | **get** /dnd.teamInfo | 



## dnd_end_dnd

> ::std::collections::HashMap<String, serde_json::Value> dnd_end_dnd(token)


Ends the current user's Do Not Disturb session immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dnd_end_snooze

> ::std::collections::HashMap<String, serde_json::Value> dnd_end_snooze(token)


Ends the current user's snooze mode immediately.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dnd_info

> ::std::collections::HashMap<String, serde_json::Value> dnd_info(token, user)


Retrieves a user's current Do Not Disturb status.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `dnd:read` |  |
**user** | Option<**String**> | User to fetch status for (defaults to current user) |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dnd_set_snooze

> ::std::collections::HashMap<String, serde_json::Value> dnd_set_snooze(num_minutes, token)


Turns on Do Not Disturb mode for the current user, or changes its duration.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**num_minutes** | **String** | Number of minutes, from now, to snooze until. | [required] |
**token** | **String** | Authentication token. Requires scope: `dnd:write` | [required] |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## dnd_team_info

> ::std::collections::HashMap<String, serde_json::Value> dnd_team_info(token, users)


Retrieves the Do Not Disturb status for up to 50 users on a team.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `dnd:read` |  |
**users** | Option<**String**> | Comma-separated list of users to fetch Do Not Disturb status for |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


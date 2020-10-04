# \RemindersApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reminders_add**](RemindersApi.md#reminders_add) | **post** /reminders.add | 
[**reminders_complete**](RemindersApi.md#reminders_complete) | **post** /reminders.complete | 
[**reminders_delete**](RemindersApi.md#reminders_delete) | **post** /reminders.delete | 
[**reminders_info**](RemindersApi.md#reminders_info) | **get** /reminders.info | 
[**reminders_list**](RemindersApi.md#reminders_list) | **get** /reminders.list | 



## reminders_add

> ::std::collections::HashMap<String, serde_json::Value> reminders_add(token, text, user, time)


Creates a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**text** | Option<**String**> | The content of the reminder |  |
**user** | Option<**String**> | The user who will receive the reminder. If no user is specified, the reminder will go to user who created it. |  |
**time** | Option<**String**> | When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. \\\"in 15 minutes,\\\" or \\\"every Thursday\\\") |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_complete

> ::std::collections::HashMap<String, serde_json::Value> reminders_complete(token, reminder)


Marks a reminder as complete.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**reminder** | Option<**String**> | The ID of the reminder to be marked as complete |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_delete

> ::std::collections::HashMap<String, serde_json::Value> reminders_delete(token, reminder)


Deletes a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:write` |  |
**reminder** | Option<**String**> | The ID of the reminder |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_info

> ::std::collections::HashMap<String, serde_json::Value> reminders_info(token, reminder)


Gets information about a reminder.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:read` |  |
**reminder** | Option<**String**> | The ID of the reminder |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reminders_list

> ::std::collections::HashMap<String, serde_json::Value> reminders_list(token)


Lists all reminders created by or for a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `reminders:read` |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


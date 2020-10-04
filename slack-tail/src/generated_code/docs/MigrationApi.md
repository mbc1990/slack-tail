# \MigrationApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**migration_exchange**](MigrationApi.md#migration_exchange) | **get** /migration.exchange | 



## migration_exchange

> ::std::collections::HashMap<String, serde_json::Value> migration_exchange(token, users, to_old)


For Enterprise Grid workspaces, map local user IDs to global user IDs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | **String** | Authentication token. Requires scope: `tokens.basic` | [required] |
**users** | **String** | A comma-separated list of user ids, up to 400 per request | [required] |
**to_old** | Option<**bool**> | Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


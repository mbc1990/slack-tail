# \FilesRemoteApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_remote_add**](FilesRemoteApi.md#files_remote_add) | **post** /files.remote.add | 
[**files_remote_info**](FilesRemoteApi.md#files_remote_info) | **get** /files.remote.info | 
[**files_remote_list**](FilesRemoteApi.md#files_remote_list) | **get** /files.remote.list | 
[**files_remote_remove**](FilesRemoteApi.md#files_remote_remove) | **post** /files.remote.remove | 
[**files_remote_share**](FilesRemoteApi.md#files_remote_share) | **get** /files.remote.share | 
[**files_remote_update**](FilesRemoteApi.md#files_remote_update) | **post** /files.remote.update | 



## files_remote_add

> ::std::collections::HashMap<String, serde_json::Value> files_remote_add(title, filetype, token, indexable_file_contents, preview_image, external_id, external_url)


Adds a file from a remote service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**title** | Option<**String**> | Title of the file being shared. |  |
**filetype** | Option<**String**> | type of file |  |
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**indexable_file_contents** | Option<**String**> | A text file (txt, pdf, doc, etc.) containing textual search terms that are used to improve discovery of the remote file. |  |
**preview_image** | Option<**String**> | Preview of the document via `multipart/form-data`. |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**external_url** | Option<**String**> | URL of the remote file. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_info

> ::std::collections::HashMap<String, serde_json::Value> files_remote_info(token, external_id, file)


Retrieve information about a remote file added to Slack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:read` |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_list

> ::std::collections::HashMap<String, serde_json::Value> files_remote_list(ts_to, cursor, ts_from, token, limit, channel)


Retrieve information about a remote file added to Slack

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ts_to** | Option<**f32**> | Filter files created before this timestamp (inclusive). |  |
**cursor** | Option<**String**> | Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first \"page\" of the collection. See [pagination](/docs/pagination) for more detail. |  |
**ts_from** | Option<**f32**> | Filter files created after this timestamp (inclusive). |  |
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. |  |
**channel** | Option<**String**> | Filter files appearing in a specific channel, indicated by its ID. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_remove

> ::std::collections::HashMap<String, serde_json::Value> files_remote_remove(token, external_id, file)


Remove a remote file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_share

> ::std::collections::HashMap<String, serde_json::Value> files_remote_share(channels, token, external_id, file)


Share a remote file into a channel.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channels** | Option<**String**> | Comma-separated list of channel IDs where the file will be shared. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:share` |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_remote_update

> ::std::collections::HashMap<String, serde_json::Value> files_remote_update(title, filetype, token, file, indexable_file_contents, preview_image, external_id, external_url)


Updates an existing remote file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**title** | Option<**String**> | Title of the file being shared. |  |
**filetype** | Option<**String**> | type of file |  |
**token** | Option<**String**> | Authentication token. Requires scope: `remote_files:write` |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**indexable_file_contents** | Option<**String**> | File containing contents that can be used to improve searchability for the remote file. |  |
**preview_image** | Option<**String**> | Preview of the document via `multipart/form-data`. |  |
**external_id** | Option<**String**> | Creator defined GUID for the file. |  |
**external_url** | Option<**String**> | URL of the remote file. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


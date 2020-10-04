# \FilesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_comments_delete**](FilesApi.md#files_comments_delete) | **post** /files.comments.delete | 
[**files_delete**](FilesApi.md#files_delete) | **post** /files.delete | 
[**files_info**](FilesApi.md#files_info) | **get** /files.info | 
[**files_list**](FilesApi.md#files_list) | **get** /files.list | 
[**files_remote_add**](FilesApi.md#files_remote_add) | **post** /files.remote.add | 
[**files_remote_info**](FilesApi.md#files_remote_info) | **get** /files.remote.info | 
[**files_remote_list**](FilesApi.md#files_remote_list) | **get** /files.remote.list | 
[**files_remote_remove**](FilesApi.md#files_remote_remove) | **post** /files.remote.remove | 
[**files_remote_share**](FilesApi.md#files_remote_share) | **get** /files.remote.share | 
[**files_remote_update**](FilesApi.md#files_remote_update) | **post** /files.remote.update | 
[**files_revoke_public_url**](FilesApi.md#files_revoke_public_url) | **post** /files.revokePublicURL | 
[**files_shared_public_url**](FilesApi.md#files_shared_public_url) | **post** /files.sharedPublicURL | 
[**files_upload**](FilesApi.md#files_upload) | **post** /files.upload | 



## files_comments_delete

> ::std::collections::HashMap<String, serde_json::Value> files_comments_delete(token, id, file)


Deletes an existing comment on a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**id** | Option<**String**> | The comment to delete. |  |
**file** | Option<**String**> | File to delete a comment from. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_delete

> ::std::collections::HashMap<String, serde_json::Value> files_delete(token, file)


Deletes a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | ID of file to delete. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_info

> ::std::collections::HashMap<String, serde_json::Value> files_info(count, cursor, token, limit, file, page)


Gets information about a team file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**String**> |  |  |
**cursor** | Option<**String**> | Parameter for pagination. File comments are paginated for a single file. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \"page\" of the collection of comments. See [pagination](/docs/pagination) for more details. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `files:read` |  |
**limit** | Option<**i32**> | The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. |  |
**file** | Option<**String**> | Specify a file by providing its ID. |  |
**page** | Option<**String**> |  |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_list

> ::std::collections::HashMap<String, serde_json::Value> files_list(count, channel, ts_to, ts_from, token, user, show_files_hidden_by_limit, page, types)


Lists & filters team files.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**count** | Option<**String**> |  |  |
**channel** | Option<**String**> | Filter files appearing in a specific channel, indicated by its ID. |  |
**ts_to** | Option<**f32**> | Filter files created before this timestamp (inclusive). |  |
**ts_from** | Option<**f32**> | Filter files created after this timestamp (inclusive). |  |
**token** | Option<**String**> | Authentication token. Requires scope: `files:read` |  |
**user** | Option<**String**> | Filter files created by a single user. |  |
**show_files_hidden_by_limit** | Option<**bool**> | Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit. |  |
**page** | Option<**String**> |  |  |
**types** | Option<**String**> | Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like `types=spaces,snippets`.The default value is `all`, which does not filter the list. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## files_revoke_public_url

> ::std::collections::HashMap<String, serde_json::Value> files_revoke_public_url(token, file)


Revokes public/external sharing access for a file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File to revoke |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_shared_public_url

> ::std::collections::HashMap<String, serde_json::Value> files_shared_public_url(token, file)


Enables a file for public/external sharing.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File to share |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded, application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_upload

> ::std::collections::HashMap<String, serde_json::Value> files_upload(channels, title, initial_comment, filetype, filename, content, token, file, thread_ts)


Uploads or creates a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channels** | Option<**String**> | Comma-separated list of channel names or IDs where the file will be shared. |  |
**title** | Option<**String**> | Title of file. |  |
**initial_comment** | Option<**String**> | The message text introducing the file in specified `channels`. |  |
**filetype** | Option<**String**> | A [file type](/types/file#file_types) identifier. |  |
**filename** | Option<**String**> | Filename of file. |  |
**content** | Option<**String**> | File contents via a POST variable. If omitting this parameter, you must provide a `file`. |  |
**token** | Option<**String**> | Authentication token. Requires scope: `files:write:user` |  |
**file** | Option<**String**> | File contents via `multipart/form-data`. If omitting this parameter, you must submit `content`. |  |
**thread_ts** | Option<**f32**> | Provide another message's `ts` value to upload this file as a reply. Never use a reply's `ts` value; use its parent instead. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


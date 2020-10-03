# \FilesRemoteApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_remote_add**](FilesRemoteApi.md#files_remote_add) | **Post** /files.remote.add | 
[**files_remote_info**](FilesRemoteApi.md#files_remote_info) | **Get** /files.remote.info | 
[**files_remote_list**](FilesRemoteApi.md#files_remote_list) | **Get** /files.remote.list | 
[**files_remote_remove**](FilesRemoteApi.md#files_remote_remove) | **Post** /files.remote.remove | 
[**files_remote_share**](FilesRemoteApi.md#files_remote_share) | **Get** /files.remote.share | 
[**files_remote_update**](FilesRemoteApi.md#files_remote_update) | **Post** /files.remote.update | 


# **files_remote_add**
> ::std::collections::HashMap<String, Value> files_remote_add(ctx, optional)


Adds a file from a remote service

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **title** | **String**| Title of the file being shared. | 
 **filetype** | **String**| type of file | 
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:write&#x60; | 
 **indexable_file_contents** | **String**| A text file (txt, pdf, doc, etc.) containing textual search terms that are used to improve discovery of the remote file. | 
 **preview_image** | **String**| Preview of the document via &#x60;multipart/form-data&#x60;. | 
 **external_id** | **String**| Creator defined GUID for the file. | 
 **external_url** | **String**| URL of the remote file. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_remote_info**
> ::std::collections::HashMap<String, Value> files_remote_info(ctx, optional)


Retrieve information about a remote file added to Slack

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:read&#x60; | 
 **external_id** | **String**| Creator defined GUID for the file. | 
 **file** | **String**| Specify a file by providing its ID. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_remote_list**
> ::std::collections::HashMap<String, Value> files_remote_list(ctx, optional)


Retrieve information about a remote file added to Slack

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ts_to** | **f32**| Filter files created before this timestamp (inclusive). | 
 **cursor** | **String**| Paginate through collections of data by setting the &#x60;cursor&#x60; parameter to a &#x60;next_cursor&#x60; attribute returned by a previous request&#39;s &#x60;response_metadata&#x60;. Default value fetches the first \&quot;page\&quot; of the collection. See [pagination](/docs/pagination) for more detail. | 
 **ts_from** | **f32**| Filter files created after this timestamp (inclusive). | 
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:read&#x60; | 
 **limit** | **i32**| The maximum number of items to return. | 
 **channel** | **String**| Filter files appearing in a specific channel, indicated by its ID. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_remote_remove**
> ::std::collections::HashMap<String, Value> files_remote_remove(ctx, optional)


Remove a remote file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:write&#x60; | 
 **external_id** | **String**| Creator defined GUID for the file. | 
 **file** | **String**| Specify a file by providing its ID. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_remote_share**
> ::std::collections::HashMap<String, Value> files_remote_share(ctx, optional)


Share a remote file into a channel.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channels** | **String**| Comma-separated list of channel IDs where the file will be shared. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:share&#x60; | 
 **external_id** | **String**| Creator defined GUID for the file. | 
 **file** | **String**| Specify a file by providing its ID. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_remote_update**
> ::std::collections::HashMap<String, Value> files_remote_update(ctx, optional)


Updates an existing remote file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **title** | **String**| Title of the file being shared. | 
 **filetype** | **String**| type of file | 
 **token** | **String**| Authentication token. Requires scope: &#x60;remote_files:write&#x60; | 
 **file** | **String**| Specify a file by providing its ID. | 
 **indexable_file_contents** | **String**| File containing contents that can be used to improve searchability for the remote file. | 
 **preview_image** | **String**| Preview of the document via &#x60;multipart/form-data&#x60;. | 
 **external_id** | **String**| Creator defined GUID for the file. | 
 **external_url** | **String**| URL of the remote file. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


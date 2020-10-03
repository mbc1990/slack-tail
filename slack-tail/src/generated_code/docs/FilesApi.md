# \FilesApi

All URIs are relative to *https://slack.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_comments_delete**](FilesApi.md#files_comments_delete) | **Post** /files.comments.delete | 
[**files_delete**](FilesApi.md#files_delete) | **Post** /files.delete | 
[**files_info**](FilesApi.md#files_info) | **Get** /files.info | 
[**files_list**](FilesApi.md#files_list) | **Get** /files.list | 
[**files_remote_add**](FilesApi.md#files_remote_add) | **Post** /files.remote.add | 
[**files_remote_info**](FilesApi.md#files_remote_info) | **Get** /files.remote.info | 
[**files_remote_list**](FilesApi.md#files_remote_list) | **Get** /files.remote.list | 
[**files_remote_remove**](FilesApi.md#files_remote_remove) | **Post** /files.remote.remove | 
[**files_remote_share**](FilesApi.md#files_remote_share) | **Get** /files.remote.share | 
[**files_remote_update**](FilesApi.md#files_remote_update) | **Post** /files.remote.update | 
[**files_revoke_public_url**](FilesApi.md#files_revoke_public_url) | **Post** /files.revokePublicURL | 
[**files_shared_public_url**](FilesApi.md#files_shared_public_url) | **Post** /files.sharedPublicURL | 
[**files_upload**](FilesApi.md#files_upload) | **Post** /files.upload | 


# **files_comments_delete**
> ::std::collections::HashMap<String, Value> files_comments_delete(ctx, optional)


Deletes an existing comment on a file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;files:write:user&#x60; | 
 **id** | **String**| The comment to delete. | 
 **file** | **String**| File to delete a comment from. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_delete**
> ::std::collections::HashMap<String, Value> files_delete(ctx, optional)


Deletes a file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;files:write:user&#x60; | 
 **file** | **String**| ID of file to delete. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_info**
> ::std::collections::HashMap<String, Value> files_info(ctx, optional)


Gets information about a team file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **String**|  | 
 **cursor** | **String**| Parameter for pagination. File comments are paginated for a single file. Set &#x60;cursor&#x60; equal to the &#x60;next_cursor&#x60; attribute returned by the previous request&#39;s &#x60;response_metadata&#x60;. This parameter is optional, but pagination is mandatory: the default value simply fetches the first \&quot;page\&quot; of the collection of comments. See [pagination](/docs/pagination) for more details. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;files:read&#x60; | 
 **limit** | **i32**| The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn&#39;t been reached. | 
 **file** | **String**| Specify a file by providing its ID. | 
 **page** | **String**|  | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_list**
> ::std::collections::HashMap<String, Value> files_list(ctx, optional)


Lists & filters team files.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **count** | **String**|  | 
 **channel** | **String**| Filter files appearing in a specific channel, indicated by its ID. | 
 **ts_to** | **f32**| Filter files created before this timestamp (inclusive). | 
 **ts_from** | **f32**| Filter files created after this timestamp (inclusive). | 
 **token** | **String**| Authentication token. Requires scope: &#x60;files:read&#x60; | 
 **user** | **String**| Filter files created by a single user. | 
 **show_files_hidden_by_limit** | **bool**| Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit. | 
 **page** | **String**|  | 
 **types** | **String**| Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like &#x60;types&#x3D;spaces,snippets&#x60;.The default value is &#x60;all&#x60;, which does not filter the list. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

# **files_revoke_public_url**
> ::std::collections::HashMap<String, Value> files_revoke_public_url(ctx, optional)


Revokes public/external sharing access for a file

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;files:write:user&#x60; | 
 **file** | **String**| File to revoke | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_shared_public_url**
> ::std::collections::HashMap<String, Value> files_shared_public_url(ctx, optional)


Enables a file for public/external sharing.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **String**| Authentication token. Requires scope: &#x60;files:write:user&#x60; | 
 **file** | **String**| File to share | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded, application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **files_upload**
> ::std::collections::HashMap<String, Value> files_upload(ctx, optional)


Uploads or creates a file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **channels** | **String**| Comma-separated list of channel names or IDs where the file will be shared. | 
 **title** | **String**| Title of file. | 
 **initial_comment** | **String**| The message text introducing the file in specified &#x60;channels&#x60;. | 
 **filetype** | **String**| A [file type](/types/file#file_types) identifier. | 
 **filename** | **String**| Filename of file. | 
 **content** | **String**| File contents via a POST variable. If omitting this parameter, you must provide a &#x60;file&#x60;. | 
 **token** | **String**| Authentication token. Requires scope: &#x60;files:write:user&#x60; | 
 **file** | **String**| File contents via &#x60;multipart/form-data&#x60;. If omitting this parameter, you must submit &#x60;content&#x60;. | 
 **thread_ts** | **f32**| Provide another message&#39;s &#x60;ts&#x60; value to upload this file as a reply. Never use a reply&#39;s &#x60;ts&#x60; value; use its parent instead. | 

### Return type

[**::std::collections::HashMap<String, Value>**](Value.md)

### Authorization

[slackAuth](../README.md#slackAuth)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


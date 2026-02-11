# \FileOperationsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_path**](FileOperationsApi.md#delete_path) | **DELETE** /v3/globus-proxy/ops/{client_id}/{endpoint_id}/{path} | Delete a path
[**list_files**](FileOperationsApi.md#list_files) | **GET** /v3/globus-proxy/ops/{client_id}/{endpoint_id}/{path} | List files at path
[**make_dir**](FileOperationsApi.md#make_dir) | **POST** /v3/globus-proxy/ops/{client_id}/{endpoint_id}/{path} | Create a directory
[**rename_path**](FileOperationsApi.md#rename_path) | **PUT** /v3/globus-proxy/ops/{client_id}/{endpoint_id}/{path} | Rename a path



## delete_path

> models::RespBasic delete_path(client_id, endpoint_id, path, access_token, refresh_token, recurse)
Delete a path

Delete a file or directory on an endpoint at path {path} relative to the default directory. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Endpoint Id | [required] |
**path** | **String** | Path relative to default directory of the endpoint | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |
**recurse** | Option<**bool**> | remove the directory and all subdirectories |  |[default to false]

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files

> models::RespFileList list_files(client_id, endpoint_id, path, access_token, refresh_token, limit, offset, filter)
List files at path

List files for an endpoint at given path relative to the default directory of the endpoint. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Endpoint Id | [required] |
**path** | **String** | Path relative to default directory of the endpoint | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |
**limit** | Option<**i32**> | pagination limit |  |[default to 1000]
**offset** | Option<**i32**> | pagination offset |  |[default to 0]
**filter** | Option<**String**> | select filter |  |

### Return type

[**models::RespFileList**](RespFileList.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## make_dir

> models::RespBasic make_dir(client_id, endpoint_id, path, access_token, refresh_token, req_make_dir)
Create a directory

Create a directory on the endpoint. Path is relative to the endpoint default directory. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Endpoint Id | [required] |
**path** | **String** | Path relative to default directory of the endpoint | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |
**req_make_dir** | Option<[**ReqMakeDir**](ReqMakeDir.md)> |  |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rename_path

> models::RespBasic rename_path(client_id, endpoint_id, path, access_token, refresh_token, req_rename)
Rename a path

Rename a file or directory on the endpoint. Paths are relative to the endpoint default directory. Source and destination paths must be specified in the request body. When renaming to a different parent directory, the parent directory of the new path must already exist. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Endpoint Id | [required] |
**path** | **String** | Source path relative to default directory of the endpoint | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |
**req_rename** | Option<[**ReqRename**](ReqRename.md)> |  |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


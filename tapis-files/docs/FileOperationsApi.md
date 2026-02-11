# \FileOperationsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete**](FileOperationsApi.md#delete) | **DELETE** /v3/files/ops/{systemId}/{path} | 
[**get_facl**](FileOperationsApi.md#get_facl) | **GET** /v3/files/utils/linux/facl/{systemId}/{path} | 
[**get_stat_info**](FileOperationsApi.md#get_stat_info) | **GET** /v3/files/utils/linux/{systemId}/{path} | 
[**insert**](FileOperationsApi.md#insert) | **POST** /v3/files/ops/{systemId}/{path} | 
[**list_files**](FileOperationsApi.md#list_files) | **GET** /v3/files/ops/{systemId}/{path} | 
[**mkdir**](FileOperationsApi.md#mkdir) | **POST** /v3/files/ops/{systemId} | 
[**move_copy**](FileOperationsApi.md#move_copy) | **PUT** /v3/files/ops/{systemId}/{path} | 
[**run_linux_native_op**](FileOperationsApi.md#run_linux_native_op) | **POST** /v3/files/utils/linux/{systemId}/{path} | 
[**set_facl**](FileOperationsApi.md#set_facl) | **POST** /v3/files/utils/linux/facl/{systemId}/{path} | 



## delete

> models::FileStringResponse delete(system_id, path)


Delete a file, directory or object on {systemID} at path {path}.  For a LINUX directory this will be a recursive delete.  For an S3 system, the path will represent either a single object or all objects in the bucket with a prefix matching the system *rootDir* if the path is the empty string.  **WARNING** For an S3 system if the path is the empty string, then all objects in the bucket with a key matching the prefix *rootDir* will be deleted. So if the *rootDir* is also the empty string, then all objects in the bucket will be removed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |

### Return type

[**models::FileStringResponse**](FileStringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_facl

> models::NativeLinuxGetFaclResponse get_facl(system_id, path)


Get file ACLs for files or directories for a system of type LINUX. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |

### Return type

[**models::NativeLinuxGetFaclResponse**](NativeLinuxGetFaclResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_stat_info

> models::FileStatInfoResponse get_stat_info(system_id, path, follow_links)


Get native stat information for a file or directory for a system of type LINUX. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**follow_links** | Option<**bool**> | When path is a symbolic link whether to get information about the link (false) or the link target (true) |  |[default to false]

### Return type

[**models::FileStatInfoResponse**](FileStatInfoResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert

> models::FileStringResponse insert(system_id, path, file)


The file or object will be uploaded at the {path} independent of the original name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**file** | **std::path::PathBuf** |  | [required] |

### Return type

[**models::FileStringResponse**](FileStringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files

> models::FileListingResponse list_files(system_id, path, pattern, limit, offset, recurse, impersonation_id, shared_ctx)


List files or objects on a Tapis system. Type for items will depend on system type. For example, for LINUX they will be posix files and for S3 they will be storage objects. For S3 the recurse flag is ignored and all objects with keys matching the path as a prefix are included.  For system types that support directory hierarchies the maximum recursion depth is 20.  Note that S3 buckets do not have a hierarchical structure. There are no directories. Everything is an object associated with a key.  Certain services may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId* for the system.  Certain services may use the query parameter *sharedCtx* to indicate that the request is in a shared context. *sharedCtx* must be set to the share grantor. Tapis will include the share grantor as part of authorization checks. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**pattern** | Option<**String**> | Wildcard pattern (glob) or regular expression to filter the results returned by this request.  Regular  expressions must have the prefix \"regex:\".  Only files where the name matches the pattern will be  returned.  The pattern is evaluated against the filename portion of the path only, and not the entire  path.  For example to match a file that begins with \"myFile\" you could use a pattern of \"myFile*\" or  \"regex:myFile.*\".  Or, to match files that end with .txt, you could supply a pattern of \"*.txt\" or  \"regex:.*\\.txt$\".  Recursive listings filter only against filnames also - so even if the directory  doesn't match the pattern, files in the directory can be returned.  See documentation for Java regex  for the exact details of how Java handles regular expressions.  NOTE - this is only supported for linux  systems at present.  |  |
**limit** | Option<**i32**> | pagination limit |  |[default to 1000]
**offset** | Option<**i64**> | pagination offset |  |[default to 0]
**recurse** | Option<**bool**> | Recursive listing. Maximum recursion depth is 20. |  |[default to false]
**impersonation_id** | Option<**String**> | Restricted. Only certain services may impersonate a Tapis user. |  |
**shared_ctx** | Option<**String**> | Restricted. Only certain services may indicate that the request is in a shared context. Must be set to the share grantor. |  |

### Return type

[**models::FileListingResponse**](FileListingResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mkdir

> models::FileStringResponse mkdir(system_id, shared_ctx, mkdir_request)


Create a directory on the system at the given path. Not supported for all system types. Currently supported for LINUX, IRODS and GLOBUS type systems.  Certain services may use the query parameter *sharedCtx* to indicate that the request is in a shared context. *sharedCtx* must be set to the share grantor. Tapis will include the share grantor as part of authorization checks.  If the path already exists as a directory, no error will be returned. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**shared_ctx** | Option<**String**> | Restricted. Only certain services may indicate that the request is in a shared context. Must be set to the share grantor. |  |
**mkdir_request** | Option<[**MkdirRequest**](MkdirRequest.md)> |  |  |

### Return type

[**models::FileStringResponse**](FileStringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## move_copy

> models::FileStringResponse move_copy(system_id, path, move_copy_request)


Move or copy a file, directory or object on {systemID} at path {path}. Not all operations supported for all system types. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**move_copy_request** | Option<[**MoveCopyRequest**](MoveCopyRequest.md)> |  |  |

### Return type

[**models::FileStringResponse**](FileStringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_linux_native_op

> models::NativeLinuxOpResultResponse run_linux_native_op(system_id, path, recursive, native_linux_op_request)


Run a native operation on a path. Operations are chmod, chown or chgrp. For a system of type LINUX. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**recursive** | Option<**bool**> | If path is a directory this indicates whether or not to apply the changes recursively |  |[default to false]
**native_linux_op_request** | Option<[**NativeLinuxOpRequest**](NativeLinuxOpRequest.md)> |  |  |

### Return type

[**models::NativeLinuxOpResultResponse**](NativeLinuxOpResultResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_facl

> models::NativeLinuxSetFaclResponse set_facl(system_id, path, native_linux_set_facl_request)


Set file ACLs for files or directories for a system of type LINUX.  This can be used for a single file or directory, or can be recursive.  If recursion is used, it can be made to follow symlinks, or not follow symlinks.  The operations support adding or removing Acl Entries as well as removing all acls or all default acls 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**native_linux_set_facl_request** | [**NativeLinuxSetFaclRequest**](NativeLinuxSetFaclRequest.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::NativeLinuxSetFaclResponse**](NativeLinuxSetFaclResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


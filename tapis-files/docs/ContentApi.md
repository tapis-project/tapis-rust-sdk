# \ContentApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_contents**](ContentApi.md#get_contents) | **GET** /v3/files/content/{systemId}/{path} | 



## get_contents

> get_contents(system_id, path, range, zip, more, impersonation_id, shared_ctx)


Get file or directory contents as a stream of data.  Certain services may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId* for the system.  Use the query parameter *zip* to request a stream compressed using the ZIP file format. This is not allowed if system *rootDir* plus *path* would result in all files on the host being included. Please download individual directories, files or objects.  Certain services may use the query parameter *sharedCtx* to indicate that the request is in a shared context. *sharedCtx* must be set to the share grantor. Tapis will include the share grantor as part of authorization checks. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**range** | Option<[**HeaderByteRange**](HeaderByteRange.md)> | Optional range of bytes to send. If not specified all content will be sent. |  |
**zip** | Option<**bool**> | Indicates a zip output stream should be provided. |  |
**more** | Option<**i64**> | Send 1k of UTF-8 encoded string back starting at 'page' 1, e.g. more=1 |  |
**impersonation_id** | Option<**String**> | Restricted. Only certain services may impersonate a Tapis user. |  |
**shared_ctx** | Option<**String**> | Restricted. Only certain services may indicate that the request is in a shared context. Must be set to the share grantor. |  |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


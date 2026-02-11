# \PermissionsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_permissions**](PermissionsApi.md#delete_permissions) | **DELETE** /v3/files/permissions/{systemId}/{path} | 
[**get_permissions**](PermissionsApi.md#get_permissions) | **GET** /v3/files/permissions/{systemId}/{path} | 
[**grant_permissions**](PermissionsApi.md#grant_permissions) | **POST** /v3/files/permissions/{systemId}/{path} | 



## delete_permissions

> models::StringResponse delete_permissions(system_id, path, username)


Revoke access for a user for the system and path. All access is revoked. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**username** | **String** | Username to remove | [required] |

### Return type

[**models::StringResponse**](StringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_permissions

> models::FilePermissionResponse get_permissions(system_id, path, username)


Get the Tapis permissions for a user for the system and path. If no user specified then permissions are retrieved for the user making the request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**username** | Option<**String**> | Username to list |  |

### Return type

[**models::FilePermissionResponse**](FilePermissionResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_permissions

> models::FilePermissionResponse grant_permissions(system_id, path, create_permission_request)


Grant access to a path for a user. Access may be READ or MODIFY. MODIFY implies READ. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | System ID | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**create_permission_request** | [**CreatePermissionRequest**](CreatePermissionRequest.md) |  | [required] |

### Return type

[**models::FilePermissionResponse**](FilePermissionResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


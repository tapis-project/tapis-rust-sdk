# \PermissionsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_user_perms**](PermissionsApi.md#get_user_perms) | **GET** /v3/apps/perms/{appId}/user/{userName} | 
[**grant_user_perms**](PermissionsApi.md#grant_user_perms) | **POST** /v3/apps/perms/{appId}/user/{userName} | 
[**revoke_user_perm**](PermissionsApi.md#revoke_user_perm) | **DELETE** /v3/apps/perms/{appId}/user/{userName}/{permission} | 
[**revoke_user_perms**](PermissionsApi.md#revoke_user_perms) | **POST** /v3/apps/perms/{appId}/user/{userName}/revoke | 



## get_user_perms

> models::RespNameArray get_user_perms(app_id, user_name)


Retrieve all application related permissions for a given application and user. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_user_perms

> models::RespBasic grant_user_perms(app_id, user_name, req_perms)


Create permissions in the Security Kernel for a user. Requester must be owner. Permissions are READ, MODIFY, EXECUTE. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**req_perms** | [**ReqPerms**](ReqPerms.md) | A JSON object specifying a list of permissions. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_perm

> models::RespBasic revoke_user_perm(app_id, user_name, permission)


Remove user permission from the Security Kernel. Requester must be owner. Permissions are READ, MODIFY, EXECUTE. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**permission** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_perms

> models::RespBasic revoke_user_perms(app_id, user_name, req_perms)


Remove permissions from the Security Kernel for a user. Requester must be owner. Permissions are READ, MODIFY, EXECUTE. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**req_perms** | [**ReqPerms**](ReqPerms.md) | A JSON object specifying a list of permissions. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


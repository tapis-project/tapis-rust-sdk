# \PermissionsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_permissions**](PermissionsApi.md#list_permissions) | **GET** /v3/actors/{actor_id}/permissions | list_permissions
[**update_permissions**](PermissionsApi.md#update_permissions) | **POST** /v3/actors/{actor_id}/permissions | update_permissions



## list_permissions

> models::ListPermissions200Response list_permissions(actor_id)
list_permissions

List permissions for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::ListPermissions200Response**](list_permissions_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_permissions

> models::ListPermissions200Response update_permissions(actor_id, actor_permission)
update_permissions

Add or update permissions for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**actor_permission** | [**ActorPermission**](ActorPermission.md) |  | [required] |

### Return type

[**models::ListPermissions200Response**](list_permissions_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


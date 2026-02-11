# \RolesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grant_role**](RolesApi.md#grant_role) | **POST** /v3/streams/roles | Grant user role.
[**list_roles**](RolesApi.md#list_roles) | **GET** /v3/streams/roles | List roles for a given user



## grant_role

> models::ListRoles200Response grant_role(new_role)
Grant user role.

Grant user role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_role** | [**NewRole**](NewRole.md) |  | [required] |

### Return type

[**models::ListRoles200Response**](list_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> models::ListRoles200Response list_roles(user, resource_type, resource_id)
List roles for a given user

Get roles for a given user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** | Check roles for this user | [required] |
**resource_type** | **String** | Check roles for this user | [required] |
**resource_id** | **String** | Project id or channel id | [required] |

### Return type

[**models::ListRoles200Response**](list_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


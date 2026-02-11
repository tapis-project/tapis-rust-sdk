# \RevokeRolesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**revoke_role**](RevokeRolesApi.md#revoke_role) | **POST** /v3/streams/roles/revokeRole | Revoke user role.



## revoke_role

> models::ListRoles200Response revoke_role(revoke_role)
Revoke user role.

Revoke user role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_role** | [**RevokeRole**](RevokeRole.md) |  | [required] |

### Return type

[**models::ListRoles200Response**](list_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


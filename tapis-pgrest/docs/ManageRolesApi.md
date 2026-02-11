# \ManageRolesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_role**](ManageRolesApi.md#create_role) | **POST** /v3/pgrest/manage/roles | create_role
[**get_role**](ManageRolesApi.md#get_role) | **GET** /v3/pgrest/manage/roles/{role_name} | get_role
[**list_roles**](ManageRolesApi.md#list_roles) | **GET** /v3/pgrest/manage/roles | list_roles
[**manage_role**](ManageRolesApi.md#manage_role) | **POST** /v3/pgrest/manage/roles/{role_name} | manage_role



## create_role

> models::CreateRole200Response create_role(new_role)
create_role

Create a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_role** | [**NewRole**](NewRole.md) |  | [required] |

### Return type

[**models::CreateRole200Response**](create_role_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role

> models::GetRole200Response get_role(role_name)
get_role

Get role info and users in the role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** | The name of the role. | [required] |

### Return type

[**models::GetRole200Response**](get_role_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_roles

> models::ListRoles200Response list_roles()
list_roles

List all roles for the tenant.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListRoles200Response**](list_roles_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_role

> models::CreateRole200Response manage_role(role_name, manage_role)
manage_role

Either grant or revoke role for users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** | The name of the role. | [required] |
**manage_role** | [**ManageRole**](ManageRole.md) |  | [required] |

### Return type

[**models::CreateRole200Response**](create_role_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


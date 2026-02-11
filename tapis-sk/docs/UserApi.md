# \UserApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_admins**](UserApi.md#get_admins) | **GET** /security/user/admins/{tenant} | 
[**get_default_user_role1**](UserApi.md#get_default_user_role1) | **GET** /security/user/defaultRole/{user} | 
[**get_user_names**](UserApi.md#get_user_names) | **GET** /security/user | 
[**get_user_perms**](UserApi.md#get_user_perms) | **GET** /security/user/perms/{user} | 
[**get_user_roles**](UserApi.md#get_user_roles) | **GET** /security/user/roles/{user} | 
[**get_users_with_permission**](UserApi.md#get_users_with_permission) | **GET** /security/user/withPermission/{permSpec} | 
[**get_users_with_role**](UserApi.md#get_users_with_role) | **GET** /security/user/withRole/{roleName} | 
[**grant_role**](UserApi.md#grant_role) | **POST** /security/user/grantRole | 
[**grant_role_with_permission**](UserApi.md#grant_role_with_permission) | **POST** /security/user/grantRoleWithPerm | 
[**grant_user_permission**](UserApi.md#grant_user_permission) | **POST** /security/user/grantUserPermission | 
[**has_role**](UserApi.md#has_role) | **POST** /security/user/hasRole | 
[**has_role_all**](UserApi.md#has_role_all) | **POST** /security/user/hasRoleAll | 
[**has_role_any**](UserApi.md#has_role_any) | **POST** /security/user/hasRoleAny | 
[**is_admin**](UserApi.md#is_admin) | **POST** /security/user/isAdmin | 
[**is_permitted**](UserApi.md#is_permitted) | **POST** /security/user/isPermitted | 
[**is_permitted_all**](UserApi.md#is_permitted_all) | **POST** /security/user/isPermittedAll | 
[**is_permitted_any**](UserApi.md#is_permitted_any) | **POST** /security/user/isPermittedAny | 
[**revoke_role**](UserApi.md#revoke_role) | **POST** /security/user/revokeRole | 
[**revoke_user_permission**](UserApi.md#revoke_user_permission) | **POST** /security/user/revokeUserPermission | 



## get_admins

> models::RespNameArray get_admins(tenant)


Get all users assigned the tenant administrator role ($!tenant_admin).  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** |  | [required] |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_user_role1

> models::RespName get_default_user_role1(user)


Get a user's default role. The default role is implicitly created by the system when needed if it doesn't already exist. No authorization required.  A user's default role is constructed by prepending '$$' to the user's name. This implies the maximum length of a user name is 58 since role names are limited to 60 characters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** |  | [required] |

### Return type

[**models::RespName**](RespName.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_names

> models::RespNameArray get_user_names(tenant)


Get the names of all users in the tenant that have been granted a role or permission.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | Option<**String**> |  |  |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_perms

> models::RespNameArray get_user_perms(user, tenant, implies, implied_by)


Get the permissions assigned to a user in a tenant, including those assigned transively. The result list can be optionally filtered by the one or both of the query parameters: implies and impliedBy.  When implies is set, the filter _implies_ each entry in the result set. When impliedBy is set, each entry in the result set is _implied by_ the filter. Below are some examples.  Consider a user that is assigned these permissions:      stream:dev:read:project1     stream:dev:read,write:project1     stream:dev:read,write,exec:project1  **Using the *implies* Query Parameter**  When _implies=stream:dev:*:project1_, this endpoint returns:      stream:dev:read:project1     stream:dev:read,write:project1     stream:dev:read,write,exec:project1  When _implies=stream:dev:write:project1_, this endpoint returns an empty list.  **Using the *impliedBy* Query Parameter**  When _impliedBy=stream:dev:*:project1_, this endpoint returns an empty list.  When _impliedBy=stream:dev:write:project1_, this endpoint returns:      stream:dev:read,write:project1     stream:dev:read,write,exec:project1  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**implies** | Option<**String**> |  |  |[default to ]
**implied_by** | Option<**String**> |  |  |[default to ]

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_roles

> models::RespNameArray get_user_roles(user, tenant)


Get the roles assigned to a user in the specified tenant, including those assigned transively.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_with_permission

> models::RespNameArray get_users_with_permission(perm_spec, tenant)


Get all users in a tenant assigned a permission. The permSpec parameter is a permission specification that uses colons as separators, the asterisk as a wildcard character and commas to define lists. Here are examples of permission specifications:      system:mytenant:read:mysystem     system:mytenant:*:mysystem     system:mytenant     files:mytenant:read,write:mysystems This method recognizes the percent sign (%) as a string wildcard only in the context of database searching. If a percent sign (%) appears in the permSpec it is interpreted as a zero or more character wildcard. For example, the following specification would match the first three of the above example specifications but not the fourth:      system:mytenant:%  The wildcard character cannot appear as the first character in the permSpec.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**perm_spec** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_users_with_role

> models::RespNameArray get_users_with_role(role_name, tenant, role_type)


Get all users assigned a role. The role must exist in the tenant.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**role_type** | Option<[**RoleTypeEnum**](.md)> |  |  |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_role

> models::RespChangeCount grant_role(req_grant_role)


Grant a user the specified role. A valid tenant and user must be specified in the request body. The type of role to grant may be specified in the request body. Allowed types are USER, TENANT_ADMIN and RESTRICTED_SVC. Default type is USER.  The user and the role must be in the same tenant.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_grant_role** | [**ReqGrantRole**](ReqGrantRole.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_role_with_permission

> models::RespChangeCount grant_role_with_permission(req_grant_role_with_permission)


Grant a user the specified role containing the specified permission. This compound request first adds the permission to the role if it is not already a member of the role and then assigns the role to the user. The change count returned can range from zero to two depending on how many insertions were actually required.  The user and the role must be in the same tenant.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_grant_role_with_permission** | [**ReqGrantRoleWithPermission**](ReqGrantRoleWithPermission.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grant_user_permission

> models::RespChangeCount grant_user_permission(req_grant_user_permission)


Grant a user the specified permission by assigning that permission to to the user's default role. If the user's default role does not exist it will be created.  A user's default role name is discoverable by calling either of the user/defaultRole or role/defaultRole endpoints.  The change count returned can range from zero to three depending on how many insertions and updates were actually required.  The caller must be an administrator or service allowed to perform updates in the user's tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_grant_user_permission** | [**ReqGrantUserPermission**](ReqGrantUserPermission.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role

> models::RespAuthorized has_role(req_user_has_role)


Check whether a user in a tenant has been assigned the specified role, either directly or transitively.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_has_role** | [**ReqUserHasRole**](ReqUserHasRole.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role_all

> models::RespAuthorized has_role_all(req_user_has_role_multi)


Check whether a user in a tenant has been assigned all of the roles specified in the request body.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_has_role_multi** | [**ReqUserHasRoleMulti**](ReqUserHasRoleMulti.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_role_any

> models::RespAuthorized has_role_any(req_user_has_role_multi)


Check whether a user in a tenant has been assigned any of the roles specified in the request body.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_has_role_multi** | [**ReqUserHasRoleMulti**](ReqUserHasRoleMulti.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_admin

> models::RespAuthorized is_admin(req_user_is_admin)


Check whether a user in a tenant has been assigned the tenant administrator role, either directly or transitively.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_is_admin** | [**ReqUserIsAdmin**](ReqUserIsAdmin.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_permitted

> models::RespAuthorized is_permitted(req_user_is_permitted)


Check whether specified permission matches a permission assigned to the user, either directly or transitively.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_is_permitted** | [**ReqUserIsPermitted**](ReqUserIsPermitted.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_permitted_all

> models::RespAuthorized is_permitted_all(req_user_is_permitted_multi)


Check whether a user's permissions satisfy all of the permission specifications contained in the request body.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_is_permitted_multi** | [**ReqUserIsPermittedMulti**](ReqUserIsPermittedMulti.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_permitted_any

> models::RespAuthorized is_permitted_any(req_user_is_permitted_multi)


Check whether a user's permissions satisfy any of the permission specifications contained in the request body.  This request is authorized if the requestor is a service or a user that has access to the specified tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_user_is_permitted_multi** | [**ReqUserIsPermittedMulti**](ReqUserIsPermittedMulti.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_role

> models::RespChangeCount revoke_role(req_revoke_role)


Revoke a previously granted role from a user. No action is taken if the user is not currently assigned the role. This request is idempotent.  The type of role to grant may be specified in the request body. Allowed types are USER, TENANT_ADMIN and RESTRICTED_SVC. Default type is USER.  The user and the role must be in the same tenant.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_revoke_role** | [**ReqRevokeRole**](ReqRevokeRole.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_permission

> models::RespChangeCount revoke_user_permission(req_revoke_user_permission)


Revoke the specified permission from the user's default role. A user's default role is constructed by prepending '$$' to the user's name. Default roles are created on demand. If the role does not exist when this method is called no error is reported and no changes occur.  The change count returned can be zero or one depending on how many permissions were revoked.  A valid tenant and user must be specified in the request body. The caller must be an administrator, a service or the user themselves. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_revoke_user_permission** | [**ReqRevokeUserPermission**](ReqRevokeUserPermission.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


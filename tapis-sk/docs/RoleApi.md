# \RoleApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_child_role**](RoleApi.md#add_child_role) | **POST** /security/role/addChild | 
[**add_role_permission**](RoleApi.md#add_role_permission) | **POST** /security/role/addPerm | 
[**create_role**](RoleApi.md#create_role) | **POST** /security/role | 
[**delete_role_by_name**](RoleApi.md#delete_role_by_name) | **DELETE** /security/role/{roleName} | 
[**get_default_user_role**](RoleApi.md#get_default_user_role) | **GET** /security/role/defaultRole/{user} | 
[**get_role_by_name**](RoleApi.md#get_role_by_name) | **GET** /security/role/{roleName} | 
[**get_role_names**](RoleApi.md#get_role_names) | **GET** /security/role | 
[**get_role_permissions**](RoleApi.md#get_role_permissions) | **GET** /security/role/{roleName}/perms | 
[**preview_path_prefix**](RoleApi.md#preview_path_prefix) | **POST** /security/role/previewPathPrefix | 
[**remove_child_role**](RoleApi.md#remove_child_role) | **POST** /security/role/removeChild | 
[**remove_path_permission_from_all_roles**](RoleApi.md#remove_path_permission_from_all_roles) | **POST** /security/role/removePathPermFromAllRoles | 
[**remove_permission_from_all_roles**](RoleApi.md#remove_permission_from_all_roles) | **POST** /security/role/removePermFromAllRoles | 
[**remove_role_permission**](RoleApi.md#remove_role_permission) | **POST** /security/role/removePerm | 
[**replace_path_prefix**](RoleApi.md#replace_path_prefix) | **POST** /security/role/replacePathPrefix | 
[**role_permits**](RoleApi.md#role_permits) | **POST** /security/role/{roleName}/permits | 
[**update_role_description**](RoleApi.md#update_role_description) | **POST** /security/role/updateDesc/{roleName} | 
[**update_role_name**](RoleApi.md#update_role_name) | **POST** /security/role/updateName/{roleName} | 
[**update_role_owner**](RoleApi.md#update_role_owner) | **POST** /security/role/updateOwner/{roleName} | 



## add_child_role

> models::RespChangeCount add_child_role(req_add_child_role)


Add a child role to another role using a request body. If the child already exists, then the request has no effect and the change count returned is zero. Otherwise, the child is added and the change count is one. Supported only for roles of type *USER*.  The user@tenant identity specified in JWT is authorized to make this request only if that user is an administrator or if the user owns both the parent and child roles. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_add_child_role** | [**ReqAddChildRole**](ReqAddChildRole.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_role_permission

> models::RespChangeCount add_role_permission(req_add_role_permission)


Add a permission to an existing role using a request body. If the permission already exists, then the request has no effect and the change count returned is zero. Otherwise, the permission is added and the change count is one.  Permissions are case-sensitive strings that follow the format defined by Apache Shiro (https://shiro.apache.org/permissions.html). This format defines any number of colon-separated (:) parts, with the possible use of asterisks (*) as wildcards and commas (,) as aggregators. Here are two example permission strings:      system:MyTenant:read,write:system1     system:MyTenant:create,read,write,delete:*  See the Shiro documentation for further details. Note that the three reserved characters, [: * ,], cannot appear in the text of any part. It's the application's responsibility to escape those characters in a manner that is safe in the application's domain.  ### Extended Permissions Tapis extends Shiro permission checking with *path semantics*. Path semantics allows the last part of pre-configured permissions to be treated as hierarchical path names, such as the paths used in POSIX file systems. Currently, only permissions that start with *files:* have their last (5th) component configured with path semantics.  Path semantics treat the extended permission part as the root of the subtree to which the permission is applied recursively. Grantees assigned the permission will have the permission on the path itself and on all its children.  As an example, consider a role that's assigned the following permission:      files:iplantc.org:read:stampede2:/home/bud  Users granted the role have read permission on the following file system resources on stampede2:      /home/bud     /home/bud/     /home/bud/myfile     /home/bud/mydir/myfile  Those users, however, will not have access to /home.  When an extended permission part ends with a slash, such as /home/bud/, then that part is interpreted as a directory or, more generally, some type of container. In such cases, the permission applies to the children of the path and to the path as written with a slash. For instance, for the file permission path /home/bud/, the permission allows access to /home/bud/ and /home/bud/myfile, but not to /home/bud.  When an extended permission part does not end with a slash, such as /home/bud, then the permission applies to the children of the path and to the path written with or without a trailing slash. For instance, for the file permission path /home/bud, the permission allows access to /home/bud, /home/bud/ and /home/bud/myfile.  In the previous examples, we assumed /home/bud was a directory. If /home/bud is a file (or more generally a leaf), then specifying the permission path /home/bud/ will not work as intended. Permissions with paths that have trailing slashes should only be used for directories, and they require a trailing slash whenever refering to the root directory. Permissions that don't have a trailing slash can represent directories or files, and thus are more general.  Extended permission checking avoids *false capture*. Whether a path has a trailing slash or not, permission checking will not capture similarly named sibling paths. For example, using the file permission path /home/bud, grantees are allowed access to /home/bud and all its children (if it's a directory), but not to the file /home/buddy.txt nor the directory /home/bud2.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_add_role_permission** | [**ReqAddRolePermission**](ReqAddRolePermission.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_role

> models::RespResourceUrl create_role(req_create_role)


Create a role using a request body. Role names are case sensitive, alpha-numeric strings that can also contain underscores. Role names must start with an alphbetic character and can be no more than 58 characters in length. The desciption can be no more than 2048 characters long. If the role already exists, this request has no effect.  For the request to be authorized, the requestor must be either an administrator or a service allowed to perform updates in the new role's tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_create_role** | [**ReqCreateRole**](ReqCreateRole.md) |  | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_role_by_name

> models::RespChangeCount delete_role_by_name(role_name, tenant, role_type)


Delete the named role. A valid tenant and user must be specified as query parameters.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**role_type** | Option<[**RoleTypeEnum**](RoleTypeEnum.md)> |  |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_user_role

> models::RespName get_default_user_role(user)


Get a user's default role. The default role is implicitly created by the system when needed if it doesn't  already exist. No authorization required.  A user's default role is constructed by prepending '$$' to the user's name. This implies the maximum length of a user name is 58 since role names are limited to 60 characters. 

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


## get_role_by_name

> models::RespRole get_role_by_name(role_name, tenant, role_type)


Get the named role's definition. A valid tenant must be specified as a query parameter. This request is authorized if the requestor is a user that has access to the specified tenant or if the requestor is a service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**role_type** | Option<[**RoleTypeEnum**](RoleTypeEnum.md)> |  |  |

### Return type

[**models::RespRole**](RespRole.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_names

> models::RespNameArray get_role_names(tenant, role_type)


Get the names of all roles in the tenant in alphabetic order. Future enhancements will include search filtering.  A valid tenant must be specified as a query parameter. This request is authorized if the requestor is a user that has access to the specified tenant or if the requestor is a service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | Option<**String**> |  |  |
**role_type** | Option<[**RoleTypeEnum**](RoleTypeEnum.md)> |  |  |

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_role_permissions

> models::RespNameArray get_role_permissions(role_name, tenant, immediate)


Get the named role's permissions. By default, all permissions assigned to the role, whether directly and transitively through child roles, are returned. Set the immediate query parameter to only retrieve permissions directly assigned to the role. A valid tenant must be specified.  This request is authorized if the requestor is a user that has access to the specified tenant or if the requestor is a service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**immediate** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::RespNameArray**](RespNameArray.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## preview_path_prefix

> models::RespPathPrefixes preview_path_prefix(req_preview_path_prefix)


This read-only endpoint previews the transformations that would take place if the same input was used on a  replacePathPrefix POST call. This call is also implemented as a POST so that the same input as used on replacePathPrefix can be used here, but this call changes nothing.  This endpoint can be used to get an accounting of existing system/path combinations that match the input specification. Such information is useful when trying to duplicate a set of permissions. For example, one may want to copy a file subtree to another location and assign the same permissions to the new subtree as currently exist on the original subtree. One could use  this call to calculate the users that should be granted permission on the new subtree.  The optional parameters are roleName, oldPrefix and newPrefix. No wildcards are defined for the path prefix parameters. When roleName is specified then only permissions assigned to that role are considered.  When the oldPrefix parameter is provided, it's used to filter out permissions whose paths do not begin with the specified string; when not provided, no path prefix filtering occurs.  When the newPrefix parameter is not provided no new characters are prepended to the new path, effectively just removing the oldPrefix from the new path. When neither oldPrefix nor newPrefix are provided, no path transformation occurs, though system IDs can still be transformed.  The result object contains an array of transformation objects, each of which contains the unique permission sequence number, the existing permission that matched the search criteria and the new permission if the specified transformations were applied.  A valid tenant and user must be specified in the request body. This request is authorized if the requestor is a user that has access to the specified tenant or if the requestor is a service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_preview_path_prefix** | [**ReqPreviewPathPrefix**](ReqPreviewPathPrefix.md) |  | [required] |

### Return type

[**models::RespPathPrefixes**](RespPathPrefixes.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_child_role

> models::RespChangeCount remove_child_role(req_remove_child_role)


Remove a child role from a parent role using a request body. A valid tenant and user must be specified in the request body. Supported only for roles of type *USER*.  The user@tenant identity specified in JWT is authorized to make this request only if that user is an administrator or if they own the parent role. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_remove_child_role** | [**ReqRemoveChildRole**](ReqRemoveChildRole.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_path_permission_from_all_roles

> models::RespChangeCount remove_path_permission_from_all_roles(req_remove_permission_from_all_roles)


Remove an extended permission from all roles in a tenant using a request body. The tenant and permission must be specified in the request body.  Each role in the tenant is searched for the extended permission string and, where found, that permission is removed. The matching algorithm is string comparison with wildcard semantics on the path component. This is the same as an exact string match for all parts of the permission specification up to the path part. A match on the path part, however, occurs when its path is a prefix of a role permission's path. Consider the following permission specification:      files:mytenant:read:mysystem:/my/dir  which will match both of the following role permissions:      files:mytenant:read:mysystem:/my/dir/subdir/myfile     files:mytenant:read:mysystem:/my/dir33/yourfile  Note that a match to the second role permission might be a *false capture* if the intension was to remove all permissions to resources in the /my/dir subtree, but not those in other directories. To avoid this potential problem, callers can make two calls, one to this endpoint with a permSpec that ends with a slash (\"/\") and one to the removePermissionFromeAllRoles endpoint with no trailing slash. The former removes all children from the directory subtree, the latter removes the directory itself.  Only the Files service is authorized to make this call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_remove_permission_from_all_roles** | [**ReqRemovePermissionFromAllRoles**](ReqRemovePermissionFromAllRoles.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_permission_from_all_roles

> models::RespChangeCount remove_permission_from_all_roles(req_remove_permission_from_all_roles)


Remove a permission from all roles in a tenant using a request body. The tenant and permission must be specified in the request body.  Each role in the tenant is searched for the *exact* permission string and, where found, that permission is removed. The matching algorithm is simple, character by character, string comparison.  Permissions are not interpreted. For example, a permission that contains a wildcard (*) will only match a role's permission when the same wildcard is found in the exact same position. The same rule applies to permission segments with multiple, comma separated components: a match requires the exact same ordering and spacing of components.  Only services are authorized to make this call. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_remove_permission_from_all_roles** | [**ReqRemovePermissionFromAllRoles**](ReqRemovePermissionFromAllRoles.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_role_permission

> models::RespChangeCount remove_role_permission(req_remove_role_permission)


Remove a permission from a role using a request body. A valid role, roleTenant and permission must be specified in the request body.  For roles of type USER the request is authorized only if the requestor is the role owner, a tenant administrator or a site administrator. For roles of type TENANT_ADMIN the requestor must a tenant or site administrator. For roles of type RESTRICTED_SVC the requestor must a site administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_remove_role_permission** | [**ReqRemoveRolePermission**](ReqRemoveRolePermission.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_path_prefix

> models::RespChangeCount replace_path_prefix(req_replace_path_prefix)


Replace the text in a permission specification when its last component defines an *extended path attribute*. Extended path attributes enhance the standard Shiro matching algorithm with one that treats designated components in a permission specification as a path name, such as a posix file or directory path name. This request is useful when files or directories have been renamed or moved and their authorizations need to be adjusted. Consider, for example, permissions that conform to the following specification:        files:tenantId:op:systemId:path  By definition, the last component is an extended path attribute whose content can be changed by replacePathPrefix requests. Specifically, paths that begin with the oldPrefix will have that prefix replaced with the newPrefix value. Replacement only occurs on permissions that also match the schema and oldSystemId parameter values. The systemId attribute is required to immediately precede the path attribute, which must be the last attribute.  Additionally, the oldSystemId is replaced with the newSystemId when a match is found. If a roleName is provided, then replacement is limited to permissions defined only in that role. Otherwise, permissions in all roles that meet the other matching criteria will be considered.  The optional parameters are roleName, oldPrefix and newPrefix. No wildcards are defined for the path prefix parameters. When roleName is specified then only permissions assigned to that role are considered.  When the oldPrefix parameter is provided, it's used to filter out permissions whose paths do not begin with the specified string; when not provided, no path prefix filtering occurs.  When the newPrefix parameter is not provided no new characters are prepended to the new path, effectively just removing the oldPrefix from the new path. When neither oldPrefix nor newPrefix are provided, no path transformation occurs, though system IDs can still be transformed.  The previewPathPrefix request provides a way to do a dry run using the same input as this request. The preview call calculates the permissions that would change and what their new values would be, but it does not actually change those permissions as replacePathPrefix does.  The input parameters are passed in the payload of this request. The response indicates the number of changed permission specifications.  The path prefix replacement operation is authorized if the user@tenant in the JWT represents a tenant administrator or the Files service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_replace_path_prefix** | [**ReqReplacePathPrefix**](ReqReplacePathPrefix.md) |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## role_permits

> models::RespAuthorized role_permits(role_name, req_role_permits, immediate)


Check to see if the specified role allows the specified permission.  Any authenticated user may make this request.   

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**req_role_permits** | [**ReqRolePermits**](ReqRolePermits.md) |  | [required] |
**immediate** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_description

> models::RespBasic update_role_description(role_name, req_update_role_description)


Update an existing role's decription using a request body. The size limit on a description is 2048 characters.  This request is authorized if the requestor is the role owner or an administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**req_update_role_description** | [**ReqUpdateRoleDescription**](ReqUpdateRoleDescription.md) |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_name

> models::RespBasic update_role_name(role_name, req_update_role_name)


Update an existing role's name using a request body. Role names are case sensitive, alphanumeric strings that can contain underscores but must begin with an alphabetic character. The limit on role name is 58 characters.  This request is authorized if the requestor is the role owner or an administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**req_update_role_name** | [**ReqUpdateRoleName**](ReqUpdateRoleName.md) |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role_owner

> models::RespBasic update_role_owner(role_name, req_update_role_owner)


Update an existing role's owner using a request body. Required parameters in the payload are the *roleTenant*, which is the tenant of named role, and *newOwner*, which is the user to which role ownership is being transferred. The *newTenant* payload parameter is optional and only needed when the new owner resides in a different tenant than that of the current owner.  This request is authorized if the requestor is the role owner or an administrator. If a new tenant is specified, then the requestor must also be allowed to act in the new tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**role_name** | **String** |  | [required] |
**req_update_role_owner** | [**ReqUpdateRoleOwner**](ReqUpdateRoleOwner.md) |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


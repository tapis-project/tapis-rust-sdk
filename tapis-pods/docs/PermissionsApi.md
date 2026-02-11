# \PermissionsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_pod_permission**](PermissionsApi.md#delete_pod_permission) | **DELETE** /pods/{pod_id}/permissions/{user} | delete_pod_permission
[**delete_snapshot_permission**](PermissionsApi.md#delete_snapshot_permission) | **DELETE** /pods/snapshots/{snapshot_id}/permissions/{user} | delete_snapshot_permission
[**delete_template_permission**](PermissionsApi.md#delete_template_permission) | **DELETE** /pods/templates/{template_id}/permissions/{user} | delete_template_permission
[**delete_volume_permission**](PermissionsApi.md#delete_volume_permission) | **DELETE** /pods/volumes/{volume_id}/permissions/{user} | delete_volume_permission
[**get_pod_permissions**](PermissionsApi.md#get_pod_permissions) | **GET** /pods/{pod_id}/permissions | get_pod_permissions
[**get_snapshot_permissions**](PermissionsApi.md#get_snapshot_permissions) | **GET** /pods/snapshots/{snapshot_id}/permissions | get_snapshot_permissions
[**get_template_permissions**](PermissionsApi.md#get_template_permissions) | **GET** /pods/templates/{template_id}/permissions | get_template_permissions
[**get_volume_permissions**](PermissionsApi.md#get_volume_permissions) | **GET** /pods/volumes/{volume_id}/permissions | get_volume_permissions
[**set_pod_permission**](PermissionsApi.md#set_pod_permission) | **POST** /pods/{pod_id}/permissions | set_pod_permission
[**set_snapshot_permission**](PermissionsApi.md#set_snapshot_permission) | **POST** /pods/snapshots/{snapshot_id}/permissions | set_snapshot_permission
[**set_template_permission**](PermissionsApi.md#set_template_permission) | **POST** /pods/templates/{template_id}/permissions | set_template_permission
[**set_volume_permission**](PermissionsApi.md#set_volume_permission) | **POST** /pods/volumes/{volume_id}/permissions | set_volume_permission



## delete_pod_permission

> models::PodPermissionsResponse delete_pod_permission(pod_id, user)
delete_pod_permission

Delete a permission from a pod.  Returns updated pod permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**models::PodPermissionsResponse**](PodPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot_permission

> models::SnapshotPermissionsResponse delete_snapshot_permission(snapshot_id, user)
delete_snapshot_permission

Delete a permission from a snapshot.  Returns updated snapshot permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**models::SnapshotPermissionsResponse**](SnapshotPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_template_permission

> models::TemplatePermissionsResponse delete_template_permission(template_id, user)
delete_template_permission

Delete a permission from a template.  Returns updated template permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**models::TemplatePermissionsResponse**](TemplatePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume_permission

> models::VolumePermissionsResponse delete_volume_permission(volume_id, user)
delete_volume_permission

Delete a permission from a volume.  Returns updated volume permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**models::VolumePermissionsResponse**](VolumePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pod_permissions

> models::PodPermissionsResponse get_pod_permissions(pod_id)
get_pod_permissions

Get a pods permissions.  Note: - There are 3 levels of permissions, READ, USER, and ADMIN. - Permissions are granted/revoked to individual TACC usernames.  Returns all pod permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodPermissionsResponse**](PodPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot_permissions

> models::SnapshotPermissionsResponse get_snapshot_permissions(snapshot_id)
get_snapshot_permissions

Get a snapshots permissions.  Note: - There are 3 levels of permissions, READ, USER, and ADMIN. - Permissions are granted/revoked to individual TACC usernames.  Returns all volue permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

[**models::SnapshotPermissionsResponse**](SnapshotPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template_permissions

> models::TemplatePermissionsResponse get_template_permissions(template_id)
get_template_permissions

Get a templates permissions.  Note: - There are 3 levels of permissions, READ, USER, and ADMIN. - Permissions are granted/revoked to individual TACC usernames. - Permissions can be set for TENANT or SITE keys for tenant-level or site-level sharing.  Returns all template permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**models::TemplatePermissionsResponse**](TemplatePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_permissions

> models::VolumePermissionsResponse get_volume_permissions(volume_id)
get_volume_permissions

Get a volumes permissions.  Note: - There are 3 levels of permissions, READ, USER, and ADMIN. - Permissions are granted/revoked to individual TACC usernames.  Returns all volue permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |

### Return type

[**models::VolumePermissionsResponse**](VolumePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_pod_permission

> models::PodPermissionsResponse set_pod_permission(pod_id, set_permission)
set_pod_permission

Set a permission for a pod.  Returns updated pod permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**set_permission** | [**SetPermission**](SetPermission.md) |  | [required] |

### Return type

[**models::PodPermissionsResponse**](PodPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_snapshot_permission

> models::SnapshotPermissionsResponse set_snapshot_permission(snapshot_id, set_permission)
set_snapshot_permission

Set a permission for a snapshot.  Returns updated snapshot permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**set_permission** | [**SetPermission**](SetPermission.md) |  | [required] |

### Return type

[**models::SnapshotPermissionsResponse**](SnapshotPermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_template_permission

> models::TemplatePermissionsResponse set_template_permission(template_id, set_permission)
set_template_permission

Set a permission for a template.  Permission formats: - username:LEVEL - Standard user permission (e.g., 'jsmith:READ') - **:READ - Site-wide public access (all users across all tenants can READ) [ADMIN ONLY] - tenant.<tenant_id>:READ - Tenant-wide public access (all users in specified tenant can READ) [ADMIN ONLY]  Notes: - Both '**' and 'tenant.*' only allow READ level for security - '**' and 'tenant.*' permissions require admin privileges - There are 3 levels of permissions, READ, USER, and ADMIN. - Permissions are granted to an individual usernames and are active across tenants.  Returns updated template permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**set_permission** | [**SetPermission**](SetPermission.md) |  | [required] |

### Return type

[**models::TemplatePermissionsResponse**](TemplatePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_volume_permission

> models::VolumePermissionsResponse set_volume_permission(volume_id, set_permission)
set_volume_permission

Set a permission for a volume.  Returns updated volume permissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |
**set_permission** | [**SetPermission**](SetPermission.md) |  | [required] |

### Return type

[**models::VolumePermissionsResponse**](VolumePermissionsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


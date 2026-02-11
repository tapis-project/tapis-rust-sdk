# \ChildSystemsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_child_system**](ChildSystemsApi.md#create_child_system) | **POST** /v3/systems/{parentId}/createChildSystem | 
[**unlink_children**](ChildSystemsApi.md#unlink_children) | **POST** /v3/systems/{parentSystemId}/unlinkChildren | 
[**unlink_from_parent**](ChildSystemsApi.md#unlink_from_parent) | **POST** /v3/systems/{childSystemId}/unlinkFromParent | 



## create_child_system

> models::RespResourceUrl create_child_system(parent_id, req_post_child_system)


Create a child system based on a parent system. The child system inherits most attributes from the parent. The following fields are filled in when the child system is created:   - *id*   - *effectiveUserId*   - *rootDir*   - *owner*  The owner will be the user who is creating the system. The caller must have read permission on the parent system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_id** | **String** |  | [required] |
**req_post_child_system** | [**ReqPostChildSystem**](ReqPostChildSystem.md) | A JSON object specifying the attributes of the child system. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_children

> models::RespChangeCount unlink_children(parent_system_id, all, req_unlink_children)


Make a child system a standalone system.  This will break the connection with it's parent, and from this point on, the child system will not be connected to the parent.  This is similar to unlinkFromParent, but permissions are required for the parent system rather than the child system. **WARNING** This cannot be undone. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**parent_system_id** | **String** |  | [required] |
**all** | Option<**bool**> | Unlink all children from the parent. Default is false. |  |[default to false]
**req_unlink_children** | Option<[**ReqUnlinkChildren**](ReqUnlinkChildren.md)> | A JSON object containing information about which systems should be unlinked. |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlink_from_parent

> models::RespChangeCount unlink_from_parent(child_system_id)


Make a child system a standalone system. This will break the connection with it's parent. From this point on, the child system will not be connected to the parent. **WARNING** This cannot be undone. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**child_system_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


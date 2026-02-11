# \SharingApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_share_info**](SharingApi.md#get_share_info) | **GET** /v3/files/share/{systemId}/{path} | Retrieve all sharing information for a path on a system
[**share_path**](SharingApi.md#share_path) | **POST** /v3/files/share/{systemId}/{path} | Share a path on a system with one or more users.
[**share_path_public**](SharingApi.md#share_path_public) | **POST** /v3/files/share_public/{systemId}/{path} | Share a path on a system publicly with all users in the tenant.
[**un_share_path**](SharingApi.md#un_share_path) | **POST** /v3/files/unshare/{systemId}/{path} | Unshare a path on a system with one or more users.
[**un_share_path_all**](SharingApi.md#un_share_path_all) | **POST** /v3/files/unshare_all/{systemId}/{path} | Remove all shares for a path on a system including public access.
[**un_share_path_public**](SharingApi.md#un_share_path_public) | **POST** /v3/files/unshare_public/{systemId}/{path} | Remove public access for a path on a system.



## get_share_info

> models::RespShareInfo get_share_info(system_id, path)
Retrieve all sharing information for a path on a system

Retrieve all sharing information for a path on a system. This includes all users with whom the path has been shared and whether or not the path has been made publicly available. Sharing a path grants users READ access to the path or, in the context of running a job, it grants users READ and MODIFY access to the path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |

### Return type

[**models::RespShareInfo**](RespShareInfo.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_path

> models::RespBasic share_path(system_id, path, req_share_update)
Share a path on a system with one or more users.

Create or update sharing information for a path on a system. The path will be shared with the list of users provided in the request body. Requester must be owner of the system. For LINUX systems path sharing is hierarchical. Sharing a path grants users READ access to the path or, in the context of running a job, it grants users READ and MODIFY access to the path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_path_public

> models::RespBasic share_path_public(system_id, path)
Share a path on a system publicly with all users in the tenant.

Share a path on a system with all users in the tenant. Requester must be owner of the system. Sharing a path grants users READ access to the path or, in the context of running a job, it grants users READ and MODIFY access to the path. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_path

> models::RespBasic un_share_path(system_id, path, req_share_update)
Unshare a path on a system with one or more users.

Create or update sharing information for a path on a system. The path will be unshared with the list of users provided in the request body. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_path_all

> models::RespBasic un_share_path_all(system_id, path, recurse)
Remove all shares for a path on a system including public access.

Remove all shares for a path on a system including public access. This will also be done for all sub-paths. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |
**recurse** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_path_public

> models::RespBasic un_share_path_public(system_id, path)
Remove public access for a path on a system.

Remove public sharing for a path on a system. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**path** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


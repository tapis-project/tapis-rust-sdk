# \SharingApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_share_info**](SharingApi.md#get_share_info) | **GET** /v3/systems/share/{systemId} | Retrieve all sharing information for a system
[**share_system**](SharingApi.md#share_system) | **POST** /v3/systems/share/{systemId} | Share a system with one or more users.
[**share_system_public**](SharingApi.md#share_system_public) | **POST** /v3/systems/share_public/{systemId} | Share a system publicly with all users in the tenant.
[**un_share_system**](SharingApi.md#un_share_system) | **POST** /v3/systems/unshare/{systemId} | Unshare a system with one or more users.
[**un_share_system_public**](SharingApi.md#un_share_system_public) | **POST** /v3/systems/unshare_public/{systemId} | Remove public access for a system.



## get_share_info

> models::RespShareInfo get_share_info(system_id)
Retrieve all sharing information for a system

Retrieve all sharing information for a system. This includes all users with whom the system has been shared and whether or not the system has been made publicly available. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespShareInfo**](RespShareInfo.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_system

> models::RespBasic share_system(system_id, req_share_update)
Share a system with one or more users.

Create or update sharing information for a system. The system will be shared with the list of users provided in the request body. Sharing allows READ and EXECUTE access. When the system has a dynamic *effectiveUserId*, sharing also allows for MODIFY access to all paths for calls made through the Files service. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_system_public

> models::RespBasic share_system_public(system_id)
Share a system publicly with all users in the tenant.

Share a system with all users in the tenant. Sharing allows READ and EXECUTE access. When the system has a dynamic *effectiveUserId*, sharing also allows for MODIFY access to all paths for calls made through the Files service. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_system

> models::RespBasic un_share_system(system_id, req_share_update)
Unshare a system with one or more users.

Create or update sharing information for a system. The system will be unshared with the list of users provided in the request body. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_system_public

> models::RespBasic un_share_system_public(system_id)
Remove public access for a system.

Remove public sharing for a system. Requester must be owner of the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


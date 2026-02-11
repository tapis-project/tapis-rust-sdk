# \SharingApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_share_info**](SharingApi.md#get_share_info) | **GET** /v3/apps/share/{appId} | Retrieve all sharing information for an app
[**share_app**](SharingApi.md#share_app) | **POST** /v3/apps/share/{appId} | Share an app with one or more users.
[**share_app_public**](SharingApi.md#share_app_public) | **POST** /v3/apps/share_public/{appId} | Share an app publicly with all users in the tenant.
[**un_share_app**](SharingApi.md#un_share_app) | **POST** /v3/apps/unshare/{appId} | Unshare an app with one or more users.
[**un_share_app_public**](SharingApi.md#un_share_app_public) | **POST** /v3/apps/unshare_public/{appId} | Remove public access for an app.



## get_share_info

> models::RespShareInfo get_share_info(app_id)
Retrieve all sharing information for an app

Retrieve all sharing information for an app. This includes all users with whom the app has been shared and whether or not the app has been made publicly available. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespShareInfo**](RespShareInfo.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_app

> models::RespBasic share_app(app_id, req_share_update)
Share an app with one or more users.

Create or update sharing information for an app. The app will be shared with the list of users provided in the request body. Requester must be owner of the app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_app_public

> models::RespBasic share_app_public(app_id)
Share an app publicly with all users in the tenant.

Share an app with all users in the tenant. Requester must be owner of the app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_app

> models::RespBasic un_share_app(app_id, req_share_update)
Unshare an app with one or more users.

Create or update sharing information for an app. The app will be unshared with the list of users provided in the request body. Requester must be owner of the app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**req_share_update** | [**ReqShareUpdate**](ReqShareUpdate.md) | A JSON object specifying updated sharing information. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## un_share_app_public

> models::RespBasic un_share_app_public(app_id)
Remove public access for an app.

Remove public sharing for an app. Requester must be owner of the app. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


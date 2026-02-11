# \GroupSecretsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_secret**](GroupSecretsApi.md#add_group_secret) | **POST** /v3/workflows/groups/{group_id}/secrets | Add a user's secret to a group
[**get_group_secret**](GroupSecretsApi.md#get_group_secret) | **GET** /v3/workflows/groups/{group_id}/secrets/{group_secret_id} | Get group secret
[**list_group_secrets**](GroupSecretsApi.md#list_group_secrets) | **GET** /v3/workflows/groups/{group_id}/secrets | List group secrets
[**remove_group_secret**](GroupSecretsApi.md#remove_group_secret) | **DELETE** /v3/workflows/groups/{group_id}/secrets/{group_secret_id} | Remove user from group



## add_group_secret

> models::RespGroupSecret add_group_secret(group_id, req_group_secret)
Add a user's secret to a group

Add a user's secret to a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_group_secret** | [**ReqGroupSecret**](ReqGroupSecret.md) | A JSON object specifying the GroupSecret to add. | [required] |

### Return type

[**models::RespGroupSecret**](RespGroupSecret.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_secret

> models::RespGroupSecret get_group_secret(group_id, group_secret_id)
Get group secret

Get a group secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**group_secret_id** | **String** |  | [required] |

### Return type

[**models::RespGroupSecret**](RespGroupSecret.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_secrets

> models::RespGroupSecretList list_group_secrets(group_id)
List group secrets

List group_secrets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespGroupSecretList**](RespGroupSecretList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_secret

> models::RespBase remove_group_secret(group_id, group_secret_id)
Remove user from group

Remove a user from a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**group_secret_id** | **String** |  | [required] |

### Return type

[**models::RespBase**](RespBase.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


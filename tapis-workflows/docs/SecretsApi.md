# \SecretsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_secret**](SecretsApi.md#create_secret) | **POST** /v3/workflows/secrets | Create a secret
[**delete_secret**](SecretsApi.md#delete_secret) | **DELETE** /v3/workflows/secrets/{secret_id} | Delete a secret
[**get_secret**](SecretsApi.md#get_secret) | **GET** /v3/workflows/secrets/{secret_id} | Retrieve a secret
[**list_secrets**](SecretsApi.md#list_secrets) | **GET** /v3/workflows/secrets | Retrieve secrets



## create_secret

> models::RespSecret create_secret(req_create_secret)
Create a secret

Create a secret. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_create_secret** | [**ReqCreateSecret**](ReqCreateSecret.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespSecret**](RespSecret.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_secret

> models::RespString delete_secret(secret_id)
Delete a secret

Delete a secret and all of the objects that belong to them

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secret

> models::RespSecret get_secret(secret_id)
Retrieve a secret

Retrieve a secret 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |

### Return type

[**models::RespSecret**](RespSecret.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_secrets

> models::RespSecretList list_secrets()
Retrieve secrets

Retrieve all secrets for a user 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespSecretList**](RespSecretList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


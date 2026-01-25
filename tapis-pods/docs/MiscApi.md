# \MiscApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**error_handler**](MiscApi.md#error_handler) | **GET** /error-handler/{status} | error_handler
[**healthcheck**](MiscApi.md#healthcheck) | **GET** /healthcheck | healthcheck
[**traefik_config**](MiscApi.md#traefik_config) | **GET** /traefik-config | traefik_config



## error_handler

> serde_json::Value error_handler(status)
error_handler

Handles all error codes from Traefik.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**status** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## healthcheck

> serde_json::Value healthcheck()
healthcheck

Health check for service. Returns healthy when api is running. Should add database health check, should add kubernetes health check

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## traefik_config

> serde_json::Value traefik_config()
traefik_config

Supplies traefik-config to service. Returns json traefik-config object for traefik to use with the http provider. Dynamic configs don't work well in  Kubernetes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


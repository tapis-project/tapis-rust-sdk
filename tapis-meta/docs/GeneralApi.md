# \GeneralApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheck**](GeneralApi.md#healthcheck) | **GET** /meta/healthcheck | 



## healthcheck

> serde_json::Value healthcheck()


Health check for liveness. No authorization required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


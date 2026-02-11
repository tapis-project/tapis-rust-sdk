# \GeneralApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_health**](GeneralApi.md#check_health) | **GET** /jobs/healthcheck | 
[**readycheck**](GeneralApi.md#readycheck) | **GET** /jobs/readycheck | 



## check_health

> models::RespProbe check_health()


Lightweight health check for liveness. No authorization required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespProbe**](RespProbe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## readycheck

> models::RespProbe readycheck()


Lightweight readiness check. No authorization required.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespProbe**](RespProbe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


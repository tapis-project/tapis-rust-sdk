# \GeneralApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**health_check**](GeneralApi.md#health_check) | **GET** /v3/systems/healthcheck | 
[**ready_check**](GeneralApi.md#ready_check) | **GET** /v3/systems/readycheck | 



## health_check

> models::RespBasic health_check()


Health check. Lightweight non-authenticated check that service is alive.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ready_check

> models::RespBasic ready_check()


Ready check. Non-authenticated check that service is ready to do work.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


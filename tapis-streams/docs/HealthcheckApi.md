# \HealthcheckApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**healthcheck**](HealthcheckApi.md#healthcheck) | **GET** /v3/streams/healthcheck | Healthcheck



## healthcheck

> models::Hello200Response healthcheck(tenant)
Healthcheck

Checks health of meta, kapacitor, influx and chords

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant** | **String** | Tenant id to check healthcheck | [required] |

### Return type

[**models::Hello200Response**](hello_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


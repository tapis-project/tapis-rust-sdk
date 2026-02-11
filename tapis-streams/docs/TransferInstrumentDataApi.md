# \TransferInstrumentDataApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**transfer_data**](TransferInstrumentDataApi.md#transfer_data) | **POST** /v3/streams/transfer | Transfer an instruments data to a system



## transfer_data

> models::TransferData201Response transfer_data(transfer)
Transfer an instruments data to a system

Transfer an instruments data to a system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer** | [**Transfer**](Transfer.md) |  | [required] |

### Return type

[**models::TransferData201Response**](transfer_data_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \EtlApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_etl_pipeline**](EtlApi.md#create_etl_pipeline) | **POST** /v3/workflows/beta/groups/{group_id}/etl | Create an ETL pipeline



## create_etl_pipeline

> models::RespResourceUrl create_etl_pipeline(group_id, req_create_etl_pipeline)
Create an ETL pipeline

Convenience endpoint for create ETL pipelines in the Tapis ecosystem. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_create_etl_pipeline** | [**ReqCreateEtlPipeline**](ReqCreateEtlPipeline.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


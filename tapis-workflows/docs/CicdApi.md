# \CicdApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ci_pipeline**](CicdApi.md#create_ci_pipeline) | **POST** /v3/workflows/groups/{group_id}/ci | Create a CI/CD pipeline



## create_ci_pipeline

> models::RespResourceUrl create_ci_pipeline(group_id, req_ci_pipeline)
Create a CI/CD pipeline

Create a CI/CD pipeline using a request body. Pipeline id must be unique within a group and can be composed of alphanumeric characters and the following special characters [-_.].  Note: When creating pipelines for the CI/CD use case(primarily building images), use this endpoint. It offers a simplified interface for those who want to avoid the complexities of creating a standard workflow. This pipeline's tasks may be modified later. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_ci_pipeline** | [**ReqCiPipeline**](ReqCiPipeline.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


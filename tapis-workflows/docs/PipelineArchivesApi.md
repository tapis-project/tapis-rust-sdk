# \PipelineArchivesApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_pipeline_archives**](PipelineArchivesApi.md#list_pipeline_archives) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/archives | Retrieve pipeline archives



## list_pipeline_archives

> models::RespArchiveList list_pipeline_archives(group_id, pipeline_id)
Retrieve pipeline archives

Retrieve a list of archives attached to a pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespArchiveList**](RespArchiveList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


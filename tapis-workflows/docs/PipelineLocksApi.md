# \PipelineLocksApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pipeline_lock**](PipelineLocksApi.md#get_pipeline_lock) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/locks/{pipeline_lock_uuid} | PipelineLocks
[**list_pipeline_locks**](PipelineLocksApi.md#list_pipeline_locks) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/locks | PipelineLocks
[**release_pipeline_lock**](PipelineLocksApi.md#release_pipeline_lock) | **DELETE** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid}/locks | PipelineLocks



## get_pipeline_lock

> models::RespPipelineLock get_pipeline_lock(group_id, pipeline_id, pipeline_lock_uuid)
PipelineLocks

Get a pipeline lock by its UUID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_lock_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespPipelineLock**](RespPipelineLock.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipeline_locks

> models::RespPipelineLockList list_pipeline_locks(group_id, pipeline_id)
PipelineLocks

List all locks for a given pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespPipelineLockList**](RespPipelineLockList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## release_pipeline_lock

> release_pipeline_lock(group_id, pipeline_id, pipeline_run_uuid)
PipelineLocks

Release a lock on the Pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


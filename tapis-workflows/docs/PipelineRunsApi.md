# \PipelineRunsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**acquire_pipeline_lock**](PipelineRunsApi.md#acquire_pipeline_lock) | **POST** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid}/locks | PipelineRuns
[**get_pipeline_run**](PipelineRunsApi.md#get_pipeline_run) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid} | Pipeline Runs
[**list_pipeline_runs**](PipelineRunsApi.md#list_pipeline_runs) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs | Pipeline Runs
[**terminate_pipeline**](PipelineRunsApi.md#terminate_pipeline) | **POST** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid} | Terminate a running pipeline
[**update_pipeline_run_status**](PipelineRunsApi.md#update_pipeline_run_status) | **PATCH** /v3/workflows/executor/runs/{pipeline_run_uuid}/{status} | Pipeline Runs



## acquire_pipeline_lock

> models::RespPipelineLockAcquisition acquire_pipeline_lock(group_id, pipeline_id, pipeline_run_uuid, req_pipeline_lock)
PipelineRuns

Attempt to acquire a lock on a Pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |
**req_pipeline_lock** | [**ReqPipelineLock**](ReqPipelineLock.md) | Data about the pipeline run attempting to acquire a lock. | [required] |

### Return type

[**models::RespPipelineLockAcquisition**](RespPipelineLockAcquisition.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline_run

> models::RespPipelineRun get_pipeline_run(group_id, pipeline_id, pipeline_run_uuid)
Pipeline Runs

Get a pipeline run 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespPipelineRun**](RespPipelineRun.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipeline_runs

> models::RespPipelineRunList list_pipeline_runs(group_id, pipeline_id)
Pipeline Runs

List runs for a pipeline 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespPipelineRunList**](RespPipelineRunList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terminate_pipeline

> models::RespPipelineRun terminate_pipeline(group_id, pipeline_id, pipeline_run_uuid)
Terminate a running pipeline

Terminate a running pipeline

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **String** |  | [required] |

### Return type

[**models::RespPipelineRun**](RespPipelineRun.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pipeline_run_status

> models::RespString update_pipeline_run_status(x_workflow_executor_token, pipeline_run_uuid, status, req_patch_pipeline_run)
Pipeline Runs

Endpoints that update pipeline runs are only accessible to the Workflow Executor 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_workflow_executor_token** | **String** | an authorization header that contains the token that authroizes the workflow executor to update the pipeline run status  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |
**status** | [**EnumRunStatus**](.md) |  | [required] |
**req_patch_pipeline_run** | Option<[**ReqPatchPipelineRun**](ReqPatchPipelineRun.md)> | Empty JSON object. |  |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


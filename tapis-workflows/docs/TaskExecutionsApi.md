# \TaskExecutionsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task_execution**](TaskExecutionsApi.md#create_task_execution) | **POST** /v3/workflows/executor/runs/{pipeline_run_uuid}/executions | Task Executions
[**get_task_execution**](TaskExecutionsApi.md#get_task_execution) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid}/executions/{task_execution_uuid} | Task Executions
[**list_task_executions**](TaskExecutionsApi.md#list_task_executions) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/runs/{pipeline_run_uuid}/executions | Task Executions
[**update_task_execution_status**](TaskExecutionsApi.md#update_task_execution_status) | **PATCH** /v3/workflows/executor/executions/{task_execution_uuid}/{status} | Task Executions



## create_task_execution

> models::RespResourceUrl create_task_execution(x_workflow_executor_token, pipeline_run_uuid, req_create_task_execution)
Task Executions

Create a task execution 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_workflow_executor_token** | **String** | an authorization header that contains the token that authroizes the workflow executor to create a task execution  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |
**req_create_task_execution** | [**ReqCreateTaskExecution**](ReqCreateTaskExecution.md) | A JSON object for the createTaskExecution operation. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task_execution

> models::RespTaskExecution get_task_execution(group_id, pipeline_id, pipeline_run_uuid, task_execution_uuid)
Task Executions

Get a Task Execution 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |
**task_execution_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespTaskExecution**](RespTaskExecution.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_task_executions

> models::RespTaskExecutionList list_task_executions(group_id, pipeline_id, pipeline_run_uuid)
Task Executions

List Task Executions for a pipeline run 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**pipeline_run_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespTaskExecutionList**](RespTaskExecutionList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_task_execution_status

> models::RespString update_task_execution_status(x_workflow_executor_token, task_execution_uuid, status, req_patch_task_execution)
Task Executions

update a task execution status 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**x_workflow_executor_token** | **String** | an authorization header that contains the token that authroizes the workflow executor to update the pipeline run status  | [required] |
**task_execution_uuid** | **uuid::Uuid** |  | [required] |
**status** | [**EnumRunStatus**](.md) |  | [required] |
**req_patch_task_execution** | Option<[**ReqPatchTaskExecution**](ReqPatchTaskExecution.md)> | Empty JSON object. |  |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


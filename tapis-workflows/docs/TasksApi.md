# \TasksApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_task**](TasksApi.md#create_task) | **POST** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/tasks | Create a task
[**delete_task**](TasksApi.md#delete_task) | **DELETE** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/tasks/{task_id} | Delete a task
[**get_task**](TasksApi.md#get_task) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/tasks/{task_id} | Retrieve task details
[**list_tasks**](TasksApi.md#list_tasks) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/tasks | List tasks
[**patch_task**](TasksApi.md#patch_task) | **PATCH** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/tasks/{task_id} | Update task details



## create_task

> models::RespResourceUrl create_task(group_id, pipeline_id, req_task)
Create a task

Create a task for a pipeline 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**req_task** | [**ReqTask**](ReqTask.md) | A JSON object specifying information for the task to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_task

> models::RespString delete_task(group_id, pipeline_id, task_id)
Delete a task

Delete a task 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_task

> models::RespTask get_task(group_id, pipeline_id, task_id)
Retrieve task details

Retrieve task details for given pipeline id and task id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |

### Return type

[**models::RespTask**](RespTask.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tasks

> models::RespTaskList list_tasks(group_id, pipeline_id)
List tasks

Retrieve all tasks for a given pipeline 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespTaskList**](RespTaskList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_task

> models::RespTask patch_task(group_id, pipeline_id, task_id, task)
Update task details

Update details for a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**task_id** | **String** |  | [required] |
**task** | [**Task**](Task.md) | Request body for the pathTask operation. | [required] |

### Return type

[**models::RespTask**](RespTask.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \PipelinesApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_pipeline_archive**](PipelinesApi.md#add_pipeline_archive) | **POST** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/archives/add | Add an archive to a pipeline
[**change_pipeline_owner**](PipelinesApi.md#change_pipeline_owner) | **PATCH** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/changeOwner/{username} | Change pipeline owner
[**create_pipeline**](PipelinesApi.md#create_pipeline) | **POST** /v3/workflows/groups/{group_id}/pipelines | Create a pipeline
[**delete_pipeline**](PipelinesApi.md#delete_pipeline) | **DELETE** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id} | Delete a pipeline
[**get_pipeline**](PipelinesApi.md#get_pipeline) | **GET** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id} | Retrieve pipeline details
[**list_pipelines**](PipelinesApi.md#list_pipelines) | **GET** /v3/workflows/groups/{group_id}/pipelines | Retrieve pipelines
[**remove_pipeline_archive**](PipelinesApi.md#remove_pipeline_archive) | **DELETE** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/archives/remove | Remove archive to a pipeline
[**run_pipeline**](PipelinesApi.md#run_pipeline) | **POST** /v3/workflows/groups/{group_id}/pipelines/{pipeline_id}/run | Trigger a pipeline run



## add_pipeline_archive

> models::RespBase add_pipeline_archive(group_id, pipeline_id)
Add an archive to a pipeline

Add an archive to a pipeline. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespBase**](RespBase.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## change_pipeline_owner

> models::RespBase change_pipeline_owner(group_id, pipeline_id, username)
Change pipeline owner

Change the owner of a pipeline. Requesting user must be the current owner 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::RespBase**](RespBase.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_pipeline

> models::RespResourceUrl create_pipeline(group_id, req_pipeline)
Create a pipeline

Create a pipeline using a request body. Pipeline id must be unique within a group and can be composed of alphanumeric characters and the following special characters [-_.]. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_pipeline** | [**ReqPipeline**](ReqPipeline.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pipeline

> models::RespString delete_pipeline(group_id, pipeline_id)
Delete a pipeline

Delete a pipeline 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pipeline

> models::RespPipeline get_pipeline(group_id, pipeline_id)
Retrieve pipeline details

Retrieve information for a pipeline given the pipeline id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespPipeline**](RespPipeline.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pipelines

> models::RespPipelineList list_pipelines(group_id)
Retrieve pipelines

Retrieve a list of pipelines for all groups that the requesting user belongs to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespPipelineList**](RespPipelineList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_pipeline_archive

> models::RespBase remove_pipeline_archive(group_id, pipeline_id)
Remove archive to a pipeline

Remove an archive to a pipeline. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |

### Return type

[**models::RespBase**](RespBase.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## run_pipeline

> models::RespPipelineRun run_pipeline(group_id, pipeline_id, req_run_pipeline)
Trigger a pipeline run

Trigger a pipeline run 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**pipeline_id** | **String** |  | [required] |
**req_run_pipeline** | [**ReqRunPipeline**](ReqRunPipeline.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespPipelineRun**](RespPipelineRun.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


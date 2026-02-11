# \ExecutionsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_execution**](ExecutionsApi.md#get_execution) | **GET** /v3/actors/{actor_id}/executions/{execution_id} | get_execution
[**get_execution_logs**](ExecutionsApi.md#get_execution_logs) | **GET** /v3/actors/{actor_id}/executions/{execution_id}/logs | get_execution_logs
[**get_execution_result**](ExecutionsApi.md#get_execution_result) | **GET** /v3/actors/{actor_id}/executions/{execution_id}/results | get_execution_result
[**list_executions**](ExecutionsApi.md#list_executions) | **GET** /v3/actors/{actor_id}/executions | list_executions



## get_execution

> models::GetExecution200Response get_execution(actor_id, execution_id)
get_execution

Get details about an execution.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**execution_id** | **String** | Unique ID of the execution | [required] |

### Return type

[**models::GetExecution200Response**](get_execution_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution_logs

> models::GetExecutionLogs200Response get_execution_logs(actor_id, execution_id)
get_execution_logs

Get an execution's logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**execution_id** | **String** | Unique ID of the execution | [required] |

### Return type

[**models::GetExecutionLogs200Response**](get_execution_logs_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution_result

> std::path::PathBuf get_execution_result(actor_id, execution_id)
get_execution_result

Get an execution's result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**execution_id** | **String** | Unique ID of the execution | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_executions

> models::ListExecutions200Response list_executions(actor_id)
list_executions

List executions for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::ListExecutions200Response**](list_executions_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


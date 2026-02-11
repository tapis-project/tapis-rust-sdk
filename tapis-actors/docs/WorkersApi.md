# \WorkersApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_workers**](WorkersApi.md#list_workers) | **GET** /v3/actors/{actor_id}/workers | list_workers
[**manage_worker_pool_size**](WorkersApi.md#manage_worker_pool_size) | **POST** /v3/actors/{actor_id}/workers | manage_worker_pool_size



## list_workers

> models::ActorWorkerResponse list_workers(actor_id)
list_workers

List workers for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::ActorWorkerResponse**](ActorWorkerResponse.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_worker_pool_size

> models::SendMessage200Response manage_worker_pool_size(actor_id, manage_worker_pool_size_request)
manage_worker_pool_size

Manage number of workers in actor's worker pool. Pool size will not decrease as a result of this action.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**manage_worker_pool_size_request** | [**ManageWorkerPoolSizeRequest**](ManageWorkerPoolSizeRequest.md) |  | [required] |

### Return type

[**models::SendMessage200Response**](send_message_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/octet-stream, text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


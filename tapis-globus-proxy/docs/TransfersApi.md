# \TransfersApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_transfer_task**](TransfersApi.md#cancel_transfer_task) | **DELETE** /v3/globus-proxy/transfers/{client_id}/{task_id} | Cancel a transfer task
[**create_transfer_task**](TransfersApi.md#create_transfer_task) | **POST** /v3/globus-proxy/transfers/{client_id} | Create task to transfer paths from one endpoint to another
[**get_transfer_task**](TransfersApi.md#get_transfer_task) | **GET** /v3/globus-proxy/transfers/{client_id}/{task_id} | Retrieve transfer task



## cancel_transfer_task

> models::RespCancelTask cancel_transfer_task(client_id, task_id, access_token, refresh_token)
Cancel a transfer task

Request that a transfer task be cancelled. Note that even if the response indicates that the task has been cancelled the task still may have succeeded. Task status must be checked. See Globus documentation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**task_id** | **String** |  | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |

### Return type

[**models::RespCancelTask**](RespCancelTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer_task

> models::RespTransferTask create_transfer_task(client_id, access_token, refresh_token, req_create_transfer)
Create task to transfer paths from one endpoint to another

Create a task to transfer files from a source endpoint to a destination endpoint. File paths are relative to the endpoint default directories. Endpoints are activated as needed. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |
**req_create_transfer** | Option<[**ReqCreateTransfer**](ReqCreateTransfer.md)> |  |  |

### Return type

[**models::RespTransferTask**](RespTransferTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer_task

> models::RespTransferTask get_transfer_task(client_id, task_id, access_token, refresh_token)
Retrieve transfer task

Retrieve a transfer task given the task Id. Access token and refresh token must be provided as a query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**task_id** | **String** |  | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |

### Return type

[**models::RespTransferTask**](RespTransferTask.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


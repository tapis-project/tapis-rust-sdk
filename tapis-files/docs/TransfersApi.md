# \TransfersApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_transfer_task**](TransfersApi.md#cancel_transfer_task) | **DELETE** /v3/files/transfers/{transferTaskId} | 
[**create_transfer_task**](TransfersApi.md#create_transfer_task) | **POST** /v3/files/transfers | 
[**get_recent_transfer_tasks**](TransfersApi.md#get_recent_transfer_tasks) | **GET** /v3/files/transfers | 
[**get_transfer_task**](TransfersApi.md#get_transfer_task) | **GET** /v3/files/transfers/{transferTaskId} | 
[**get_transfer_task_details**](TransfersApi.md#get_transfer_task_details) | **GET** /v3/files/transfers/{transferTaskId}/details | 



## cancel_transfer_task

> models::StringResponse cancel_transfer_task(transfer_task_id)


Request that a transfer task be cancelled. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_task_id** | **String** | Transfer task ID | [required] |

### Return type

[**models::StringResponse**](StringResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_transfer_task

> models::TransferTaskResponse create_transfer_task(req_transfer)


Create a background task which will transfer files between systems. Note that not all combinations of system types are supported. For example, transfers involving a GLOBUS system must be GLOBUS to GLOBUS.    Transfers will fail if there are more than a set number of files per directory.  The current limit is  10,000 files, however that could change in the future.  It's recommended that for large numbers of files you build an archive (tar, zip, etc) and transfer that instead. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_transfer** | [**ReqTransfer**](ReqTransfer.md) |  | [required] |

### Return type

[**models::TransferTaskResponse**](TransferTaskResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recent_transfer_tasks

> models::TransferTaskListResponse get_recent_transfer_tasks(limit, offset)


Get a list of transfer tasks starting with the most recent. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | pagination limit |  |[default to 1000]
**offset** | Option<**i32**> | pagination offset |  |[default to 0]

### Return type

[**models::TransferTaskListResponse**](TransferTaskListResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer_task

> models::TransferTaskResponse get_transfer_task(transfer_task_id, include_summary, impersonation_id)


Retrieve a transfer task. By default only the top level attributes are included in the result. The top level attributes are: tenantId, uuid, status, username, tag, created, startTime, endTime, errorMessage. The query parameter *includeSummary* may be set to *true* to also include totalTransfers, completeTransfers and estimatedTotalBytes. The default for *includeSummary* is *false*.  Certain services may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId* for the system.  TransferTask attributes:   - *tenantId*: tenant associated with the transfer task.   - *uuid*: Unique id of the transfer task.   - *tag*: Optional tag provided by user who requested the transfer.   - *username*: Tapis user who requested the transfer.    - *status*: ACCEPTED, STAGED, IN_PROGRESS, COMPLETED, CANCELLED, FAILED, FAILED_OPT, PAUSED   - *created*: Timestamp    - *startTime*: Timestamp   - *endTime*: Timestamp   - *errorMessage*: Error message, if applicable.   - *totalTransfers*: Total number of child transfers requested.   - *completeTransfers*: Number of child transfers completed.   - *estimatedTotalBytes*: Estimate of total number of bytes transferred by all child tasks. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_task_id** | **String** | Transfer task ID | [required] |
**include_summary** | Option<**bool**> | Indicates if summary information such as *estimatedTotalBytes* should be included. |  |
**impersonation_id** | Option<**String**> | Restricted. Only certain services may impersonate a Tapis user. |  |

### Return type

[**models::TransferTaskResponse**](TransferTaskResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer_task_details

> models::TransferTaskResponse get_transfer_task_details(transfer_task_id, impersonation_id)


Retrieve all information for a transfer task, including totalTransfers, completedTransfers, estimatedTotalBytes, list of parents and list of children for each parent.  Certain services may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId* for the system.  For more information on transfer task attributes please see *getTransferTask*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transfer_task_id** | **String** | Transfer task ID | [required] |
**impersonation_id** | Option<**String**> | Restricted. Only certain services may impersonate a Tapis user. |  |

### Return type

[**models::TransferTaskResponse**](TransferTaskResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


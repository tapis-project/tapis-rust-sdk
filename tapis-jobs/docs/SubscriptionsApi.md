# \SubscriptionsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subscriptions**](SubscriptionsApi.md#delete_subscriptions) | **DELETE** /jobs/subscribe/{uuid} | 
[**get_subscriptions**](SubscriptionsApi.md#get_subscriptions) | **GET** /jobs/subscribe/{jobUuid} | 
[**subscribe**](SubscriptionsApi.md#subscribe) | **POST** /jobs/subscribe/{jobUuid} | 



## delete_subscriptions

> models::ResultChangeCount delete_subscriptions(uuid)


Depending on the UUID provide, this API either deletes a single subscription from a job or all subscriptions from a job. To delete single subscription, provide the UUID of that subscription as listed in the subscription retrieval result for the job.  To delete all a job's subscriptions, specify the job UUID.  Like all Job subscription APIs, modifications only affect running jobs and never change the saved job definition. As a consequence, job resubmissions are not affected by runtime subscription changes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**models::ResultChangeCount**](ResultChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> models::RespGetSubscriptions get_subscriptions(job_uuid, limit, skip)


Retrieve a job's subscriptions fom the Notifications service. After subscriptions expire or are deleted by user action they may no longer be listed in Notification service. To inspect the initial set of subscriptions assigned to a job, retrieve the job definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |[default to 100]
**skip** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::RespGetSubscriptions**](RespGetSubscriptions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subscribe

> models::RespResourceUrl subscribe(job_uuid, req_subscribe)


Subcribe to a running job identified by it's UUID. The caller must be the job owner or a tenant administrator.  Like all Job subscription APIs, modifications only affect running jobs and never change the saved job definition. As a consequence, job resubmissions are not affected by runtime subscription changes.  The events to which one can subscribe are:  - JOB_NEW_STATUS - the job has transitioned to a new status - JOB_INPUT_TRANSACTION_ID - a request to stage job input files has been submitted - JOB_ARCHIVE_TRANSACTION_ID - a request to archive job output files has been submitted - JOB_SUBSCRIPTION - a change to the job's subscriptions has occurred - JOB_SHARE_EVENT - a job resource has been shared or unshared - JOB_ERROR_MESSAGE - the job experienced an error - JOB_USER_EVENT - user generated events - ALL - all job event categories

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**req_subscribe** | [**ReqSubscribe**](ReqSubscribe.md) |  | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


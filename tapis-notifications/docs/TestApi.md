# \TestApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**begin_test_sequence**](TestApi.md#begin_test_sequence) | **POST** /v3/notifications/test/begin | 
[**delete_test_sequence**](TestApi.md#delete_test_sequence) | **DELETE** /v3/notifications/test/{name} | 
[**get_test_sequence**](TestApi.md#get_test_sequence) | **GET** /v3/notifications/test/{name} | 
[**record_test_notification**](TestApi.md#record_test_notification) | **POST** /v3/notifications/test/callback/{name} | 



## begin_test_sequence

> models::RespSubscription begin_test_sequence(subscription_ttl, number_of_events, end_series, body)


Start a test sequence by creating a subscription and publishing one or more events matching the subscription. If more than one event requested then events will be published at the rate of 1 every 3 seconds. Only services may perform this operation.  The subscription will have the following properties:   - owner: *api_user*   - name: *subscription_uuid*   - typeFilter: notifications.test.*   - subjectFilter: *subscription_uuid*   - deliveryTarget:     - deliveryMethod: WEBHOOK     - deliveryAddress: *tenant_base_url*_/v3/notifications/test/callback/_*subscription_uuid*   - ttlMinutes: 60  The default TTL of 1 hour may be overridden using the query parameter *subscriptionTTL*.  The initial event will have the following properties:   - source: *tenant_base_url*_/v3/notifications/test   - type: notifications.test.begin   - subject: *subscription_uuid*   - seriesId: *subscription_uuid*  The requested number of events will be published to the main queue and the subscription will be returned to the caller. The sequence of test events may be continued by publishing events that match the test subscription.  Results will be recorded when notifications associated with the test are received via the callback. The first notification should be recorded shortly after the initial event is published. If query parameter *endSeries* is set to true (the default), when the final event of the test sequence is sent it will have the event attribute *endSeries* set to true. This will result in removal of series tracking data. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subscription_ttl** | Option<**i32**> | Subscription TTL in minutes. Default is 60 minutes. A TTL of 0 or less indicates no expiration. |  |[default to 60]
**number_of_events** | Option<**i32**> | Number of events to publish when starting the series. Default is 1. |  |[default to 1]
**end_series** | Option<**bool**> | Flag indicating if tracking data for series should be removed when final event in the test sequence is sent. Default is true. |  |[default to true]
**body** | Option<**serde_json::Value**> | A json request body. Not used but required to correctly generate the client. |  |

### Return type

[**models::RespSubscription**](RespSubscription.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_test_sequence

> models::RespChangeCount delete_test_sequence(name)


Delete all test sequence artifacts including the subscription, results and event series tracking data. Only services may perform this operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_test_sequence

> models::RespTestSequence get_test_sequence(name)


Retrieve status and result history for a test sequence created using the endpoint *test/begin*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::RespTestSequence**](RespTestSequence.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_test_notification

> models::RespBasic record_test_notification(name, notification)


Callback endpoint for receiving a notification associated with a test sequence. Results will be recorded when notifications associated with the test are received via the callback. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**notification** | [**Notification**](Notification.md) | A JSON object with event details. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


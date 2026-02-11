# \EventsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**end_event_series**](EventsApi.md#end_event_series) | **POST** /v3/notifications/events/endSeries | 
[**post_event**](EventsApi.md#post_event) | **POST** /v3/notifications/events | 
[**publish_event**](EventsApi.md#publish_event) | **POST** /v3/notifications/events/publish | 



## end_event_series

> models::RespBasic end_event_series(event_series, tenant)


End an event series. Series tracking data will be deleted. A subsequent new event published with the same tenant, source, subject and seriesId will create a new series with the *seriesSeqCount* starting at 1. Associated event source, subject and seriesId must be provided in the request body.  To specify a tenant other than the oboTenant, please use the query parameter *tenant*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_series** | [**EventSeries**](EventSeries.md) | A JSON object specifying the event series to be ended. | [required] |
**tenant** | Option<**String**> | Highly restricted. Tenant associated with the event. Only services may set the tenant. By default, oboTenant from the JWT is used. |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_event

> models::RespBasic post_event(event, tenant)


(**DEPRECATED** Please use endpoint *notifications/events/publishEvent*)  Post an event to be distributed to all subscribers. Only services may post events.  The attributes *source*, *type* and *timestamp* are required. Note that certain attributes, marked as *attribute* in the list below, are allowed but ignored. These attributes are maintained by Tapis. They are present when the event is part of a delivered notification.  Event attributes:   - source: Context in which event happened: Examples: *Jobs*, *Systems*.   - type: Type of event. Used for routing notifications. A series of 3 fields separated by the dot character. Pattern is *service.category.detail*. Examples: *jobs.new_status.complete*, *systems.system.create*, *files.object.delete*   - subject: Subject of event in the context of the service. Examples: job Id, system Id, file path, role name, etc.   - timestamp: When the event happened.   - data: Optional additional information associated with the event. Data specific to the service associated with the event.   - deleteSubscriptionsMatchingSubject: Boolean indicating that all subscriptions whose *subjectFilter* matches the *subject* of the event should be deleted once all notifications are delivered.   - seriesId: Optional Id that may be used to group events from the same tenant, source and subject. In a series, event order is preserved when sending out notifications.   - *tenant*: Tapis tenant associated with the event.   - *uuid*: Tapis generated unique identifier.   - *user*: Tapis user associated with the event.  Note that events are not persisted by the front end api service. When received they are simply sent to a message broker. The back end dispatch service will persist events temporarily in order to support recovery.  An event is delivered to the delivery target in a Notification object. Notification attributes:   - uuid: Unique identifier for the notification.   - subscriptionName: Name of subscription associated with the event.   - event: All information contained in the event.   - eventUuid: Unique identifier for the event.   - tenant: tenant associated with the event.   - deliveryTarget: the delivery target   - created: When the notification was created.  For details on the schema for a Notification object, please see the request body specification included under the endpoint for *recordTestNotification*, at path *_/v3/notifications/test/callback/{name}*  Note that certain attributes in the request body (such as tenant) are allowed but ignored. These attributes are maintained by Tapis. They are present when the event is part of a delivered notification. The attributes that are allowed but ignored are    - tenant   - uuid   - user 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event** | [**Event**](Event.md) | A JSON object specifying the event details. | [required] |
**tenant** | Option<**String**> | Highly restricted. Tenant associated with the event. Only services may set the tenant. By default, oboTenant from the JWT is used. |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## publish_event

> models::RespBasic publish_event(event, tenant)


Publish an event to be distributed to all subscribers. Only services may publish events.  The attributes *source*, *type* and *timestamp* are required. Note that certain attributes, marked in *italics* in the list below, are allowed but ignored. These attributes are maintained by Tapis. They are present when the event is part of a delivered notification.  A published event may include a value for the attribute *seriesId*. Each series is intended to sequentially track events of various types coming from a given tenant, source and subject. So for each tenant, source and subject the seriesId is considered unique. For example, the Jobs service (the source) sends out events with the jobUuid as the subject and sets the seriesId to the jobUuid. That way, a subscription can be created to follow (in order) all events of various types related to the job. Examples of event types defined in the Jobs service are JOB_NEW_STATUS, JOB_ERROR_MESSAGE, JOB_USER_EVENT.  Tracking data for a series is created automatically when the first event in a series is received. Note that the subject is required. If an event is received without a subject then it will not be part of a series even if the attribute *seriesId* is set. An event publisher may indicate that this is the last event in a series by publishing an event with the attribute *endSeries* to true or by calling the endpoint *notifications/events/endSeries*. When a series is ended, tracking data for that series is deleted. This effectively resets the seriesSeqCount. A subsequent event published with the same seriesId will create a new series with a seriesSeqCount starting at 1.  Event attributes:   - source: Context in which event happened: Examples: *Jobs*, *Systems*.   - type: Type of event. Used for routing notifications. A series of 3 fields separated by the dot character. Pattern is *service.category.detail*. Examples: *jobs.new_status.complete*, *systems.system.create*, *files.object.delete*   - subject: Subject of event in the context of the service. Examples: job Id, system Id, file path, role name, etc.   - timestamp: When the event happened.   - data: Optional additional information associated with the event. Data specific to the service associated with the event.   - deleteSubscriptionsMatchingSubject: Boolean indicating that all subscriptions whose *subjectFilter* exactly matches the *subject* of the event should be deleted once all notifications are delivered. This will also end the series if seriesId is set.   - seriesId: Optional Id that may be used to group events from the same tenant, source and subject. In a series, event order is preserved when sending out notifications.   - endSeries: Boolean indicating that this is the last event in a series. Series tracking data will be deleted.   - *tenant*: Tapis tenant associated with the event.   - *uuid*: Tapis generated unique identifier.   - *user*: Tapis user associated with the event.   - *received*: Tapis generated timestamp for when the event was received by Tapis.   - *seriesSeqCount*: Tapis generated counter for seriesId. Can be used by notification receivers to track expected order. Notifications for events will be sent in order but may not be received in order.  Note that events are not persisted by the front end api service. When received they are simply sent to a message broker. The back end dispatch service will persist events temporarily in order to support recovery.  An event is delivered to the delivery target in a Notification object. Notification attributes:   - uuid: Unique identifier for the notification.   - subscriptionName: Name of subscription associated with the event.   - event: All information contained in the event.   - eventUuid: Unique identifier for the event.   - tenant: tenant associated with the event.   - deliveryTarget: the delivery target   - created: When the notification was created.  For details on the schema for a Notification object, please see the request body specification included under the endpoint for *recordTestNotification*, at path *_/v3/notifications/test/callback/{name}*  Note that certain attributes in the request body (such as tenant) are allowed but ignored. These attributes are maintained by Tapis. They are present when the event is part of a delivered notification. The attributes that are allowed but ignored are    - tenant   - uuid   - user   - received   - seriesSeqCount 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event** | [**Event**](Event.md) | A JSON object specifying the event details. | [required] |
**tenant** | Option<**String**> | Highly restricted. Tenant associated with the event. Only services may set the tenant. By default, oboTenant from the JWT is used. |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


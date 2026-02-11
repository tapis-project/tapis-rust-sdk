# \SubscriptionsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_subscription_by_name**](SubscriptionsApi.md#delete_subscription_by_name) | **DELETE** /v3/notifications/subscriptions/byName/{name} | 
[**delete_subscription_by_uuid**](SubscriptionsApi.md#delete_subscription_by_uuid) | **DELETE** /v3/notifications/subscriptions/byUuid/{uuid} | 
[**delete_subscriptions_by_subject**](SubscriptionsApi.md#delete_subscriptions_by_subject) | **DELETE** /v3/notifications/subscriptions/bySubject/{subject} | 
[**disable_subscription**](SubscriptionsApi.md#disable_subscription) | **POST** /v3/notifications/subscriptions/{name}/disable | 
[**enable_subscription**](SubscriptionsApi.md#enable_subscription) | **POST** /v3/notifications/subscriptions/{name}/enable | 
[**get_subscription_by_name**](SubscriptionsApi.md#get_subscription_by_name) | **GET** /v3/notifications/subscriptions/byName/{name} | 
[**get_subscription_by_uuid**](SubscriptionsApi.md#get_subscription_by_uuid) | **GET** /v3/notifications/subscriptions/byUuid/{uuid} | 
[**get_subscriptions**](SubscriptionsApi.md#get_subscriptions) | **GET** /v3/notifications/subscriptions | 
[**is_enabled**](SubscriptionsApi.md#is_enabled) | **GET** /v3/notifications/subscriptions/{name}/isEnabled | 
[**patch_subscription_by_name**](SubscriptionsApi.md#patch_subscription_by_name) | **PATCH** /v3/notifications/subscriptions/byName/{name} | 
[**post_subscription**](SubscriptionsApi.md#post_subscription) | **POST** /v3/notifications/subscriptions | 
[**search_subscriptions_query_parameters**](SubscriptionsApi.md#search_subscriptions_query_parameters) | **GET** /v3/notifications/subscriptions/search | 
[**search_subscriptions_request_body**](SubscriptionsApi.md#search_subscriptions_request_body) | **POST** /v3/notifications/subscriptions/search | 
[**update_ttl**](SubscriptionsApi.md#update_ttl) | **POST** /v3/notifications/subscriptions/{name}/updateTTL/{ttlMinutes} | 



## delete_subscription_by_name

> models::RespChangeCount delete_subscription_by_name(name, owned_by)


Delete a subscription. Only services may perform this operation.  Events will stop being delivered. Subscription must be re-created to resume event delivery. Query parameter *ownedBy* may be used to delete a subscription owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**owned_by** | Option<**String**> | Delete a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscription_by_uuid

> models::RespChangeCount delete_subscription_by_uuid(uuid)


Delete a subscription whose UUID matches the provided value. Only services may perform this operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_subscriptions_by_subject

> models::RespChangeCount delete_subscriptions_by_subject(subject, owned_by, any_owner)


Delete all subscriptions whose *subjectFilter* exactly matches the specific *subject* provided. Only services may perform this operation.  Query parameter *ownedBy* may be used to delete subscriptions owned by a user other than the requesting user. Ignored if *anyOwner* is set to true. Only services or tenant administrators may set *ownedBy*.  Query parameter *anyOwner* may be used to delete subscriptions owned by any user. Query parameter *anyOwner* takes precedence over parameter *ownedBy*. Only services may set *anyOwner*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subject** | **String** |  | [required] |
**owned_by** | Option<**String**> | Delete subscriptions owned by a user other than the requesting user. Ignored if *anyOwner* is true. Requester must be a service or tenant administrator. |  |
**any_owner** | Option<**bool**> | Delete subscriptions owned by any user. If set to true then *ownedBy* is ignored. Default is false. Requester must be a service. |  |[default to false]

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_subscription

> models::RespChangeCount disable_subscription(name, owned_by)


Update a subscription to be inactive. Only services may perform this operation.  Matching events will stop being delivered. Query parameter *ownedBy* may be used to disable a subscription owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**owned_by** | Option<**String**> | Update a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_subscription

> models::RespChangeCount enable_subscription(name, owned_by)


Update a subscription to be active. Only services may perform this operation.  Matching events will start being delivered. Query parameter *ownedBy* may be used to enable a subscription owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**owned_by** | Option<**String**> | Update a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_by_name

> models::RespSubscription get_subscription_by_name(name, select, owned_by)


Retrieve a subscription owned by the requesting user and whose name matches the provided value. Query parameter *ownedBy* may be used to retrieve a subscription owned by another user. Only services or tenant administrators may set the *ownedBy* query parameter.  Summary of subscription attributes. For details please see *postSubscription*:   - owner: Tapis user who owns the subscription.   - name: Short descriptive name.   - description: Optional more verbose description.   - enabled: Indicates if subscription is currently active.    - typeFilter: Filter to use when matching events. Format is *service.category.detail*. Each field may be a specific type or the wildcard character *.   - subjectFilter: Filter to use when matching events. Filter for subject. May be wildcard character *.   - deliveryTargets: List of delivery targets to be used when delivering a matching event.   - ttlMinutes: Time to live in minutes. 0 indicates no expiration.   - uuid: A UUID generated by the service.   - expiry: Time at which the subscription expires and will be deleted. Maintained by the service.   - created: When created. Maintained by the service.   - updated: When last updated. Maintained by the service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**select** | Option<**String**> | List of attributes to be included as part of the result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=owner,name,typeFilter |  |[default to allAttributes]
**owned_by** | Option<**String**> | Retrieve subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespSubscription**](RespSubscription.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscription_by_uuid

> models::RespSubscription get_subscription_by_uuid(uuid, select)


Retrieve a subscription whose UUID matches the provided value.  Summary of subscription attributes. For details please see *postSubscription*:   - owner: Tapis user who owns the subscription.   - name: Short descriptive name.   - description: Optional more verbose description.   - enabled: Indicates if subscription is currently active.    - typeFilter: Filter to use when matching events. Format is *service.category.detail*. Each field may be a specific type or the wildcard character *.   - subjectFilter: Filter to use when matching events. Filter for subject. May be wildcard character *.   - deliveryTargets: List of delivery targets to be used when delivering a matching event.   - ttlMinutes: Time to live in minutes. 0 indicates no expiration.   - uuid: A UUID generated by the service.   - expiry: Time at which the subscription expires and will be deleted. Maintained by the service.   - created: When created. Maintained by the service.   - updated: When last updated. Maintained by the service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**uuid** | **String** |  | [required] |
**select** | Option<**String**> | List of attributes to be included as part of the result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=owner,name,typeFilter |  |[default to allAttributes]

### Return type

[**models::RespSubscription**](RespSubscription.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_subscriptions

> models::RespSubscriptions get_subscriptions(search, limit, order_by, skip, start_after, compute_total, select, owned_by, any_owner)


Retrieve list of subscriptions owned by the requesting user. Use search and select query parameters to limit results.  Query parameter *ownedBy* may be used to retrieve subscriptions owned by a user other than the requesting user. Ignored if *anyOwner* is set to true. Only services or tenant administrators may set *ownedBy*.  Query parameter *anyOwner* may be used to retrieve all subscriptions owned by any user. Query parameter *anyOwner* takes precedence over parameter *ownedBy*. Only services may set *anyOwner*.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search conditions as a single query parameter. For example search=(name.like.MySubscr*)~(enabled.eq.false) |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=name(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=name(asc)&startAfter=my.sub1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=owner,name,typeFilter. |  |[default to summaryAttributes]
**owned_by** | Option<**String**> | Retrieve subscriptions owned by a user other than the requesting user. Ignored if *anyOwner* is true. Requester must be a service or tenant administrator. |  |
**any_owner** | Option<**bool**> | Include subscriptions owned by any user. If set to true then *ownedBy* is ignored. Default is false. Requester must be a service. |  |[default to false]

### Return type

[**models::RespSubscriptions**](RespSubscriptions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_enabled

> models::RespBoolean is_enabled(name, owned_by)


Check if a subscription owned by the requesting user is currently active. Query parameter *ownedBy* may be used to check a subscription owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**owned_by** | Option<**String**> | Check a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespBoolean**](RespBoolean.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_subscription_by_name

> models::RespResourceUrl patch_subscription_by_name(name, req_patch_subscription, owned_by)


Update selected attributes of a subscription. Only services may perform this operation.  Request body may only contain updatable attributes. Subscription must exist. Query parameter *ownedBy* may be used to patch a subscription owned by another user. Only services or tenant administrators may set *ownedBy*.  Attributes that may not be updated via PATCH are    - name   - owner   - enabled   - ttlMinutes  Note that the attributes *enabled* and *ttlMinutes* may be modified using other endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**req_patch_subscription** | [**ReqPatchSubscription**](ReqPatchSubscription.md) | A JSON object specifying changes to be applied. | [required] |
**owned_by** | Option<**String**> | Patch a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_subscription

> models::RespResourceUrl post_subscription(req_post_subscription)


Create a subscription using a request body. Only services may perform this operation.  For each owner the *name* must be unique and can be composed of alphanumeric characters and the following special characters [-._~]. If attribute *name* is not provided then the service will generate one. The attributes *typeFilter*, *subjectFilter* and *deliveryTargets* are required.  Subscription attributes:   - name: Optional short descriptive name. *owner+name* must be unique. If one is not provided the service will create one.   - description: Optional more verbose description. Maximum length of 2048 characters.   - owner: A specific user set at create time. Default is *${apiUserId}*.   - enabled: Indicates if subscription is active. By default enabled is true.   - typeFilter: Filter to use when matching events. Filter for event type. Has three dot separated parts: *service*, *category* and *detail*. Each field may be a specific type or the wildcard character. Examples are *jobs.job.complete*, *jobs.new_status.\\**.   - subjectFilter: Filter to use when matching events. Filter for subject. This may be a specific subject such as a job Id or the wildcard character *.   - deliveryTargets: List of delivery targets to be used when delivering a matching event. Must have at least one. Each target includes delivery method (EMAIL or WEBHOOK) and delivery address.   - ttlMinutes: Time to live in minutes specified when subscription is created or TTL is updated. Service will compute expiry based on TTL. Default is one week from creation. Value of 0 indicates no expiration. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_post_subscription** | [**ReqPostSubscription**](ReqPostSubscription.md) | A JSON object specifying information for the subscription to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_subscriptions_query_parameters

> models::RespSubscriptions search_subscriptions_query_parameters(free_form_parameter_name, limit, order_by, skip, start_after, compute_total, select, owned_by)


Retrieve details for subscriptions owned by the requesting user. Use query parameters to specify search conditions. For example name.eq=MySubscription. Query parameter *ownedBy* may be used to retrieve subscriptions owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**free_form_parameter_name** | Option<[**std::collections::HashMap<String, String>**](String.md)> | Free form query parameters. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=name(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=name(asc)&startAfter=my.sub2 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=owner,name,typeFilter |  |[default to summaryAttributes]
**owned_by** | Option<**String**> | Retrieve subscriptions owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespSubscriptions**](RespSubscriptions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_subscriptions_request_body

> models::RespSubscriptions search_subscriptions_request_body(req_search_subscriptions, limit, order_by, skip, start_after, compute_total, select, owned_by)


Retrieve details for subscriptions owned by the requesting user. Use request body to specify SQL-like search conditions. Query parameter *ownedBy* may be used to retrieve subscriptions owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_search_subscriptions** | [**ReqSearchSubscriptions**](ReqSearchSubscriptions.md) | A JSON object specifying SQL-like search conditions as an array of strings. Strings are concatenated to form full search query. | [required] |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=name(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=name(asc)&startAfter=my.sub1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=owner,name,typeFilter |  |[default to summaryAttributes]
**owned_by** | Option<**String**> | Retrieve subscriptions owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespSubscriptions**](RespSubscriptions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_ttl

> models::RespChangeCount update_ttl(name, ttl_minutes, owned_by)


Update Time-To-Live (TTL) for a subscription. Only services may perform this operation.  TTL provided as number of minutes. Use 0 to indicate subscription should never expire. Query parameter *ownedBy* may be used to update a subscription owned by another user. Only services or tenant administrators may set *ownedBy*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |
**ttl_minutes** | **i32** |  | [required] |
**owned_by** | Option<**String**> | Update a subscription owned by a user other than the requesting user. Requester must be a service or tenant administrator. |  |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


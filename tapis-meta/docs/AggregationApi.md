# \AggregationApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_aggregation**](AggregationApi.md#add_aggregation) | **PUT** /meta/{db}/{collection}/_aggr/{aggregation} | addAggregation
[**delete_aggregation**](AggregationApi.md#delete_aggregation) | **DELETE** /meta/{db}/{collection}/_aggr/{aggregation} | deleteAggregation
[**submit_large_aggregation**](AggregationApi.md#submit_large_aggregation) | **POST** /meta/{db}/{collection}/_aggr/{aggregation} | submitLargeAggregation
[**use_aggregation**](AggregationApi.md#use_aggregation) | **GET** /meta/{db}/{collection}/_aggr/{aggregation} | useAggregation



## add_aggregation

> add_aggregation(db, collection, aggregation, body)
addAggregation

Create an aggregation that can be executed by users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**aggregation** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> | the aggregation being defined for the collection |  |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_aggregation

> delete_aggregation(db, collection, aggregation)
deleteAggregation

Delete an aggregation defined for a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**aggregation** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_large_aggregation

> serde_json::Value submit_large_aggregation(db, collection, aggregation, page, pagesize, keys, body)
submitLargeAggregation

 This is a POST version of useAggregation on a collection with a avars value to large to submit in a query parameter. If the avars parameter is to large, it may exceed the HTTP header character limit. The HTTP server will throw a query header to large error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**aggregation** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**pagesize** | Option<**i32**> |  |  |[default to 10]
**keys** | Option<[**Vec<String>**](String.md)> |  |  |
**body** | Option<**serde_json::Value**> | the json document for avars query parameter substitution |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## use_aggregation

> use_aggregation(db, collection, aggregation)
useAggregation

Use an aggregation defined for a collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**aggregation** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


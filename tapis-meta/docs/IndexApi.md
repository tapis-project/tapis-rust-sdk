# \IndexApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_index**](IndexApi.md#create_index) | **PUT** /meta/{db}/{collection}/_indexes/{indexName} | createIndex
[**delete_index**](IndexApi.md#delete_index) | **DELETE** /meta/{db}/{collection}/_indexes/{indexName} | deleteIndex
[**list_indexes**](IndexApi.md#list_indexes) | **GET** /meta/{db}/{collection}/_indexes | listIndexes



## create_index

> create_index(db, collection, index_name, body)
createIndex

Create an index on collection in the database.  The request body should hold a json document that defines the index      { \"keys\":  <keys>, \"ops\": <options> }    Example - create an unique, sparse index on property ‘q      {\"keys\": {\"qty\": 1},\"ops\": {\"unique\": true, \"sparse\": true }}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**index_name** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> | the index being added to the collection |  |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_index

> delete_index(db, collection, index_name)
deleteIndex

Delete an index on the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**index_name** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_indexes

> Vec<serde_json::Value> list_indexes(db, collection)
listIndexes

List all indexes in the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \CollectionApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionApi.md#create_collection) | **PUT** /meta/{db}/{collection} | createCollection
[**delete_collection**](CollectionApi.md#delete_collection) | **DELETE** /meta/{db}/{collection} | deleteCollection
[**get_collection_metadata**](CollectionApi.md#get_collection_metadata) | **GET** /meta/{db}/{collection}/_meta | getCollectionMetadata
[**get_collection_size**](CollectionApi.md#get_collection_size) | **GET** /meta/{db}/{collection}/_size | getCollectionSize
[**list_documents**](CollectionApi.md#list_documents) | **GET** /meta/{db}/{collection} | listDocuments
[**submit_large_query**](CollectionApi.md#submit_large_query) | **POST** /meta/{db}/{collection}/_filter | submitLargeQuery



## create_collection

> create_collection(db, collection)
createCollection

Create a new collection in the database.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> delete_collection(if_match, db, collection)
deleteCollection

 Delete a collection in the database. This operation is limit by default to tenant administrators.  An (If-Match) header parameter with the value of the collections etag must be supplied in order for this operations to succeed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**if_match** | **String** |  | [required] |
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_metadata

> serde_json::Value get_collection_metadata(db, collection)
getCollectionMetadata

Get the Metadata for the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection_size

> String get_collection_size(db, collection)
getCollectionSize

Get the size of the collection.  The response will contain the number of documents found in the collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |

### Return type

**String**

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_documents

> Vec<serde_json::Value> list_documents(db, collection, page, pagesize, filter, sort, keys)
listDocuments

List of documents in the collection.  If no query parameters are submitted a default number of documents <pagesize> is returned in default sort order (sort)  as the first page <page> of a document result set. The default sort order is based on the \"_id\" of the document.  A (filter) query parameter value is represented by a valid MongoDb query document. This will allow retrieving documents that meet a desired criteria. When coupled with the (keys) query parameter a projection will limit the fields to return for all matching documents.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**pagesize** | Option<**i32**> |  |  |[default to 10]
**filter** | Option<[**serde_json::Value**](SerdeJson__Value.md)> |  |  |[default to {}]
**sort** | Option<[**serde_json::Value**](SerdeJson__Value.md)> |  |  |[default to {}]
**keys** | Option<[**Vec<String>**](String.md)> |  |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_large_query

> Vec<serde_json::Value> submit_large_query(db, collection, page, pagesize, sort, keys, body)
submitLargeQuery

 This is a POST version of filter on a collection with a filter value to large to submit in a query parameter. If the filter parameter is to large, it may exceed the HTTP header character limit. The HTTP server will throw a query header to large error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**page** | Option<**i32**> |  |  |[default to 1]
**pagesize** | Option<**i32**> |  |  |[default to 10]
**sort** | Option<[**serde_json::Value**](SerdeJson__Value.md)> |  |  |[default to {}]
**keys** | Option<[**Vec<String>**](String.md)> |  |  |
**body** | Option<**serde_json::Value**> | the json document as a MongoDB query document |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \DocumentApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_document**](DocumentApi.md#create_document) | **POST** /meta/{db}/{collection} | createDocument
[**delete_document**](DocumentApi.md#delete_document) | **DELETE** /meta/{db}/{collection}/{docId} | deleteDocument
[**get_document**](DocumentApi.md#get_document) | **GET** /meta/{db}/{collection}/{docId} | getDocument
[**modify_document**](DocumentApi.md#modify_document) | **PATCH** /meta/{db}/{collection}/{docId} | modifyDocument
[**replace_document**](DocumentApi.md#replace_document) | **PUT** /meta/{db}/{collection}/{docId} | replaceDocument



## create_document

> serde_json::Value create_document(db, collection, basic, body)
createDocument

 Create a new document in collection.  A document request body with out the field <_id> gets an auto generated id  A document request body with out the field <_id> writes a document with that id unless a duplicate is encountered.  A batch of document creations is possible by submitting an array of documents in the request body. All those documents  will be added to to the collection in bulk.  The addition of the (basic) query parameter set to true will return a response for a single document creation as a   Tapis basic response with the newly created <_id> for later reference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**basic** | Option<**bool**> |  |  |[default to false]
**body** | Option<**serde_json::Value**> | the json document being added to the collection or array of documents added in bulk operation |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_document

> delete_document(db, collection, doc_id)
deleteDocument

Delete a document in the collection by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**doc_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_document

> serde_json::Value get_document(db, collection, doc_id)
getDocument

Get a document form the collection by its _id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**doc_id** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## modify_document

> modify_document(db, collection, doc_id, np, body)
modifyDocument

Modify a document in the collection with _id. The fields submitted in the json of the request body will replace the same named fields in the current document.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**doc_id** | **String** |  | [required] |
**np** | Option<**bool**> |  |  |[default to false]
**body** | Option<**serde_json::Value**> | the json document being added to the collection |  |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## replace_document

> replace_document(db, collection, doc_id, body)
replaceDocument

Replace a document in the collection with the _id.  Replaces the document with the json document submitted in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**db** | **String** |  | [required] |
**collection** | **String** |  | [required] |
**doc_id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> | the json document being replaced in the collection |  |

### Return type

 (empty response body)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


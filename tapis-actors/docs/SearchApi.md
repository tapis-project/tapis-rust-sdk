# \SearchApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_database**](SearchApi.md#search_database) | **GET** /actors/search/{search_type} | search_database



## search_database

> models::BasicResponse search_database(search_type, search, limit, skip)
search_database

Return db records that match query

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_type** | **String** | The Abaco database to search | [required] |
**search** | Option<**String**> | The query to perform when using DB search. |  |
**limit** | Option<**i32**> | limit the number of search records returned. |  |
**skip** | Option<**i32**> | index (skip) to start list. |  |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


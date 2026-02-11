# \SearchAcrossAllStreamsEndPointsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search**](SearchAcrossAllStreamsEndPointsApi.md#search) | **GET** /v3/streams/search/{resource_type} | Search projects, sites, intruments and variables owned by a user



## search

> models::Search200Response search(resource_type, list_type, skip, compute_total)
Search projects, sites, intruments and variables owned by a user

Search endpoint for streams resources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**resource_type** | **String** |  | [required] |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**skip** | Option<**i32**> |  |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]

### Return type

[**models::Search200Response**](search_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


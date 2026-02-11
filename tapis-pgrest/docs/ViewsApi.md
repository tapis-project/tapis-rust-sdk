# \ViewsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_view**](ViewsApi.md#get_view) | **GET** /v3/pgrest/views/{view_name} | get_view



## get_view

> models::GetTableRows200Response get_view(view_name, limit, offset)
get_view

List objects in the view with root_url {view_name}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_name** | **String** | The root_url parameter of the view. | [required] |
**limit** | Option<**i32**> | limit the number of records returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::GetTableRows200Response**](get_table_rows_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \UnitsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_units**](UnitsApi.md#list_units) | **GET** /units | List units.



## list_units

> models::ListUnits200Response list_units(query, limit, skip)
List units.

List units.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | a formated query string for units. |  |
**limit** | Option<**i32**> | limit the number of records returned. |  |
**skip** | Option<**i32**> | index (skip) to start list. |  |

### Return type

[**models::ListUnits200Response**](list_units_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


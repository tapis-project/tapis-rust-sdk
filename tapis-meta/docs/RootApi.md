# \RootApi

All URIs are relative to *http://localhost:8080/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_db_names**](RootApi.md#list_db_names) | **GET** /meta/ | listDBNames



## list_db_names

> Vec<String> list_db_names()
listDBNames

List the names of all Dbs available. This operation is limited to Service admins.

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ManageTablesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_table**](ManageTablesApi.md#create_table) | **POST** /v3/pgrest/manage/tables | create_table
[**delete_table**](ManageTablesApi.md#delete_table) | **DELETE** /v3/pgrest/manage/tables/{table_id} | delete_table
[**get_table**](ManageTablesApi.md#get_table) | **GET** /v3/pgrest/manage/tables/{table_id} | get_table
[**list_tables**](ManageTablesApi.md#list_tables) | **GET** /v3/pgrest/manage/tables | list_tables
[**update_table**](ManageTablesApi.md#update_table) | **PUT** /v3/pgrest/manage/tables/{table_id} | update_table



## create_table

> models::CreateTable201Response create_table(new_table)
create_table

Create a table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_table** | [**NewTable**](NewTable.md) |  | [required] |

### Return type

[**models::CreateTable201Response**](create_table_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_table

> models::BasicResponse delete_table(table_id)
delete_table

Delete a specific table and all associted data. WARNING -- this action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_id** | **String** | The id of the table to delete. | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table

> models::CreateTable201Response get_table(table_id, details)
get_table

Get details about a specific table.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_id** | **String** | The id of the table. | [required] |
**details** | Option<**bool**> | Get additional details about the table |  |

### Return type

[**models::CreateTable201Response**](create_table_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tables

> models::ListTables200Response list_tables()
list_tables

List tables in the tenant's schema.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListTables200Response**](list_tables_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_table

> models::CreateTable201Response update_table(table_id, update_table)
update_table

Update a table definition with specific table operations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**table_id** | **String** | The id of the table. | [required] |
**update_table** | [**UpdateTable**](UpdateTable.md) |  | [required] |

### Return type

[**models::CreateTable201Response**](create_table_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


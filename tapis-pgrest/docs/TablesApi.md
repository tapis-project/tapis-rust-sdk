# \TablesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_table_row**](TablesApi.md#add_table_row) | **POST** /v3/pgrest/data/{root_url} | add_table_row
[**add_table_rows**](TablesApi.md#add_table_rows) | **POST** /v3/pgrest/data//{root_url} | add_table_rows
[**delete_table_row**](TablesApi.md#delete_table_row) | **DELETE** /v3/pgrest/data/{root_url}/{item} | delete_table_row
[**get_table_row**](TablesApi.md#get_table_row) | **GET** /v3/pgrest/data/{root_url}/{item} | get_table_row
[**get_table_rows**](TablesApi.md#get_table_rows) | **GET** /v3/pgrest/data/{root_url} | get_table_rows
[**update_table_row**](TablesApi.md#update_table_row) | **PUT** /v3/pgrest/data/{root_url}/{item} | update_table_row
[**update_table_rows**](TablesApi.md#update_table_rows) | **PUT** /v3/pgrest/data/{root_url} | update_table_rows



## add_table_row

> models::AddTableRow201Response add_table_row(root_url, new_table_row)
add_table_row

Create a new object in the table with specified root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**new_table_row** | [**NewTableRow**](NewTableRow.md) |  | [required] |

### Return type

[**models::AddTableRow201Response**](add_table_row_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_table_rows

> models::GetTableRows200Response add_table_rows(root_url, new_table_rows)
add_table_rows

Create new objects in the table with specified root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**new_table_rows** | [**NewTableRows**](NewTableRows.md) |  | [required] |

### Return type

[**models::GetTableRows200Response**](get_table_rows_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_table_row

> models::BasicResponse delete_table_row(root_url, item)
delete_table_row

Delete a specific object with id {item} from the table with root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**item** | **String** | The id of an item (i.e., a row) on the table whose root_url is given by {root_url}. | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table_row

> models::AddTableRow201Response get_table_row(root_url, item)
get_table_row

Get details about the specific object with id {item} on the table with root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**item** | **String** | The id of an item (i.e., a row) on the table whose root_url is given by {root_url}. | [required] |

### Return type

[**models::AddTableRow201Response**](add_table_row_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_table_rows

> models::GetTableRows200Response get_table_rows(root_url, limit, offset)
get_table_rows

List objects in the table with specified root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
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


## update_table_row

> models::AddTableRow201Response update_table_row(root_url, item, body)
update_table_row

Update a specific object with id {item} from the table with root_url {root_url}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**item** | **String** | The id of an item (i.e., a row) on the table whose root_url is given by {root_url}. | [required] |
**body** | **serde_json::Value** |  | [required] |

### Return type

[**models::AddTableRow201Response**](add_table_row_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_table_rows

> models::BasicResponse update_table_rows(root_url, update_multiple_table_rows)
update_table_rows

Update multiple rows in a table atomically based on filter, if no filter, update all.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**root_url** | **String** | The root_url parameter of the table. | [required] |
**update_multiple_table_rows** | [**UpdateMultipleTableRows**](UpdateMultipleTableRows.md) |  | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


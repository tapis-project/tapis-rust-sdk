# \ClientsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_client**](ClientsApi.md#create_client) | **POST** /v3/oauth2/clients | 
[**delete_client**](ClientsApi.md#delete_client) | **DELETE** /v3/oauth2/clients/{client_id} | Permanently set a client to inactive.
[**get_client**](ClientsApi.md#get_client) | **GET** /v3/oauth2/clients/{client_id} | Get client details
[**list_clients**](ClientsApi.md#list_clients) | **GET** /v3/oauth2/clients | 
[**update_client**](ClientsApi.md#update_client) | **PUT** /v3/oauth2/clients/{client_id} | Update client details



## create_client

> models::CreateClient201Response create_client(new_client)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_client** | [**NewClient**](NewClient.md) |  | [required] |

### Return type

[**models::CreateClient201Response**](create_client_201_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client

> models::DeleteClient200Response delete_client(client_id)
Permanently set a client to inactive.

Permanently set a client to inactive. Once set to inactive clients cannot be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Unique ID of the client | [required] |

### Return type

[**models::DeleteClient200Response**](delete_client_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client

> models::CreateClient201Response get_client(client_id)
Get client details

Get details of a specific client by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Unique ID of the client | [required] |

### Return type

[**models::CreateClient201Response**](create_client_201_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clients

> models::ListClients200Response list_clients(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of clients returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListClients200Response**](list_clients_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client

> models::CreateClient201Response update_client(client_id, update_client)
Update client details

Update details of a specific client by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Unique ID of the client | [required] |
**update_client** | [**UpdateClient**](UpdateClient.md) |  | [required] |

### Return type

[**models::CreateClient201Response**](create_client_201_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \AliasesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_alias**](AliasesApi.md#create_alias) | **POST** /v3/actors/aliases | create_alias
[**delete_alias**](AliasesApi.md#delete_alias) | **DELETE** /v3/actors/aliases/{alias} | delete_alias
[**get_alias**](AliasesApi.md#get_alias) | **GET** /v3/actors/aliases/{alias} | get_alias
[**list_aliases**](AliasesApi.md#list_aliases) | **GET** /v3/actors/aliases | list_aliases
[**update_actor_alias**](AliasesApi.md#update_actor_alias) | **PUT** /v3/actors/aliases/{alias} | update_actor_alias



## create_alias

> models::CreateAlias201Response create_alias(new_alias)
create_alias

Register an actor alias

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_alias** | [**NewAlias**](NewAlias.md) |  | [required] |

### Return type

[**models::CreateAlias201Response**](create_alias_201_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_alias

> models::DeleteActor200Response delete_alias(alias)
delete_alias

Permenantly delete an actor alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | Unique alias of the actor | [required] |

### Return type

[**models::DeleteActor200Response**](delete_actor_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_alias

> models::CreateAlias201Response get_alias(alias)
get_alias

Get details of a specific actor alias.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | Unique alias of the actor | [required] |

### Return type

[**models::CreateAlias201Response**](create_alias_201_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_aliases

> models::ListAliases200Response list_aliases(limit, offset)
list_aliases

List all actor aliases available to user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of actors returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListAliases200Response**](list_aliases_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_actor_alias

> models::CreateAlias201Response update_actor_alias(alias, new_alias)
update_actor_alias

Update an alias definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**alias** | **String** | Unique alias of the actor | [required] |
**new_alias** | [**NewAlias**](NewAlias.md) |  | [required] |

### Return type

[**models::CreateAlias201Response**](create_alias_201_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


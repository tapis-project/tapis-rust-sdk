# \ActorsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_actor**](ActorsApi.md#create_actor) | **POST** /v3/actors | create_actor
[**delete_actor**](ActorsApi.md#delete_actor) | **DELETE** /v3/actors/{actor_id} | delete_actor
[**get_actor**](ActorsApi.md#get_actor) | **GET** /v3/actors/{actor_id} | get_actor
[**get_execution_result**](ActorsApi.md#get_execution_result) | **GET** /v3/actors/{actor_id}/executions/{execution_id}/results | get_execution_result
[**list_actors**](ActorsApi.md#list_actors) | **GET** /v3/actors | list_actors
[**update_actor**](ActorsApi.md#update_actor) | **PUT** /v3/actors/{actor_id} | update_actor



## create_actor

> models::CreateActor201Response create_actor(new_actor)
create_actor

Register an actor

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_actor** | [**NewActor**](NewActor.md) |  | [required] |

### Return type

[**models::CreateActor201Response**](create_actor_201_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_actor

> models::DeleteActor200Response delete_actor(actor_id)
delete_actor

Permenantly delete an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::DeleteActor200Response**](delete_actor_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_actor

> models::GetActor200Response get_actor(actor_id)
get_actor

Get details of a specific actor by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::GetActor200Response**](get_actor_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_execution_result

> std::path::PathBuf get_execution_result(actor_id, execution_id)
get_execution_result

Get an execution's result.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**execution_id** | **String** | Unique ID of the execution | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_actors

> models::ListActors200Response list_actors(limit, offset)
list_actors

List summary of all actors owned by user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of actors returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListActors200Response**](list_actors_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_actor

> models::GetActor200Response update_actor(actor_id, new_actor)
update_actor

Update an actor's definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**new_actor** | [**NewActor**](NewActor.md) |  | [required] |

### Return type

[**models::GetActor200Response**](get_actor_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


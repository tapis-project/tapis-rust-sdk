# \NoncesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_nonce**](NoncesApi.md#create_nonce) | **POST** /v3/actors/{actor_id}/nonces | create_nonce
[**delete_nonce**](NoncesApi.md#delete_nonce) | **DELETE** /v3/actors/{actor_id}/nonces/{nonce_id} | delete_nonce
[**get_nonce**](NoncesApi.md#get_nonce) | **GET** /v3/actors/{actor_id}/nonces/{nonce_id} | get_nonce
[**list_nonces**](NoncesApi.md#list_nonces) | **GET** /v3/actors/{actor_id}/nonces | list_nonces



## create_nonce

> models::ListNonces200Response create_nonce(actor_id, new_actor_nonce)
create_nonce

Create a nonce for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**new_actor_nonce** | [**NewActorNonce**](NewActorNonce.md) |  | [required] |

### Return type

[**models::ListNonces200Response**](list_nonces_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_nonce

> models::DeleteActor200Response delete_nonce(actor_id, nonce_id)
delete_nonce

Delete a nonce.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**nonce_id** | **String** | Unique ID of the nonce | [required] |

### Return type

[**models::DeleteActor200Response**](delete_actor_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_nonce

> models::GetNonce200Response get_nonce(actor_id, nonce_id)
get_nonce

Get details about a nonce for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**nonce_id** | **String** | Unique ID of the nonce | [required] |

### Return type

[**models::GetNonce200Response**](get_nonce_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_nonces

> models::ListNonces200Response list_nonces(actor_id)
list_nonces

List nonces for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::ListNonces200Response**](list_nonces_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


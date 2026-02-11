# \MessagesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_pending_messages**](MessagesApi.md#delete_pending_messages) | **DELETE** /v3/actors/{actor_id}/messages | delete_pending_messages
[**get_messages**](MessagesApi.md#get_messages) | **GET** /v3/actors/{actor_id}/messages | get_messages
[**send_binary_message**](MessagesApi.md#send_binary_message) | **POST** /actors/{actor_id}//messages | send_binary_message
[**send_json_message**](MessagesApi.md#send_json_message) | **POST** /actors/{actor_id}///messages | send_json_message
[**send_message**](MessagesApi.md#send_message) | **POST** /v3/actors/{actor_id}/messages | send_message



## delete_pending_messages

> models::DeleteActor200Response delete_pending_messages(actor_id)
delete_pending_messages

Delete all pending messages actor's inbox.

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


## get_messages

> models::GetMessages200Response get_messages(actor_id)
get_messages

Get number of pending messages for an actor.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |

### Return type

[**models::GetMessages200Response**](get_messages_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_binary_message

> models::SendMessage200Response send_binary_message(actor_id, binary_message, _abaco_synchronous)
send_binary_message

Send an actor a binary message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**binary_message** | [**BinaryMessage**](BinaryMessage.md) |  | [required] |
**_abaco_synchronous** | Option<**String**> | Whether to use a synchronous execution |  |

### Return type

[**models::SendMessage200Response**](send_message_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_json_message

> models::SendMessage200Response send_json_message(actor_id, json_message, _abaco_synchronous)
send_json_message

Send an actor a JSON message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**json_message** | [**JsonMessage**](JsonMessage.md) |  | [required] |
**_abaco_synchronous** | Option<**String**> | Whether to use a synchronous execution |  |

### Return type

[**models::SendMessage200Response**](send_message_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_message

> models::SendMessage200Response send_message(actor_id, json_message, _abaco_synchronous)
send_message

Send an actor a string message

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**actor_id** | **String** | Unique ID of the actor | [required] |
**json_message** | [**JsonMessage**](JsonMessage.md) |  | [required] |
**_abaco_synchronous** | Option<**String**> | Whether to use a synchronous execution |  |

### Return type

[**models::SendMessage200Response**](send_message_200_response.md)

### Authorization

[wso2jwtDevAuth](../README.md#wso2jwtDevAuth), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json, application/octet-stream, application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


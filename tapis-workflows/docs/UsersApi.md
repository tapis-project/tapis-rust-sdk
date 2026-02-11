# \UsersApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_user**](UsersApi.md#add_group_user) | **POST** /v3/workflows/groups/{group_id}/users | Add a user to a group
[**get_group_user**](UsersApi.md#get_group_user) | **GET** /v3/workflows/groups/{group_id}/users/{username} | Get group user
[**list_group_users**](UsersApi.md#list_group_users) | **GET** /v3/workflows/groups/{group_id}/users | List users
[**remove_group_user**](UsersApi.md#remove_group_user) | **DELETE** /v3/workflows/groups/{group_id}/users/{username} | Remove user from group
[**update_group_user**](UsersApi.md#update_group_user) | **PATCH** /v3/workflows/groups/{group_id}/users/{username} | Update group user



## add_group_user

> models::RespResourceUrl add_group_user(group_id, req_group_user)
Add a user to a group

Add a user to a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_group_user** | [**ReqGroupUser**](ReqGroupUser.md) | A JSON object specifying the group user to add. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_user

> models::RespGroupUser get_group_user(group_id, username)
Get group user

Get a user from a group 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::RespGroupUser**](RespGroupUser.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_users

> models::RespGroupUserList list_group_users(group_id)
List users

List users. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespGroupUserList**](RespGroupUserList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_user

> models::RespGroupUser remove_group_user(group_id, username)
Remove user from group

Remove a user from a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**username** | **String** |  | [required] |

### Return type

[**models::RespGroupUser**](RespGroupUser.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_user

> models::RespGroupUser update_group_user(group_id, username, req_update_group_user)
Update group user

Update a user for a specified group. Only group admins can perform this operation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**username** | **String** |  | [required] |
**req_update_group_user** | [**ReqUpdateGroupUser**](ReqUpdateGroupUser.md) | A JSON object | [required] |

### Return type

[**models::RespGroupUser**](RespGroupUser.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


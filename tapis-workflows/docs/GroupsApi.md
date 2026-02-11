# \GroupsApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group**](GroupsApi.md#create_group) | **POST** /v3/workflows/groups | Create a group
[**delete_group**](GroupsApi.md#delete_group) | **DELETE** /v3/workflows/groups/{group_id} | Delete a group
[**get_group**](GroupsApi.md#get_group) | **GET** /v3/workflows/groups/{group_id} | Retrieve group details
[**list_groups**](GroupsApi.md#list_groups) | **GET** /v3/workflows/groups | Retrieve groups



## create_group

> models::RespResourceUrl create_group(req_group)
Create a group

Create a group that perform CRUD operations on workflow resources.  The owner of the group will be made an admin by default. If you want to set other users as admins, you must use the '' endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_group** | [**ReqGroup**](ReqGroup.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group

> models::RespString delete_group(group_id)
Delete a group

Delete a group and all of the objects that belong to them

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> models::RespGroupDetail get_group(group_id)
Retrieve group details

Retrieve details for a given group id 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespGroupDetail**](RespGroupDetail.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> models::RespGroupList list_groups()
Retrieve groups

Retrieve all groups to which the user belongs 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespGroupList**](RespGroupList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


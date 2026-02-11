# \OwnersApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_owner**](OwnersApi.md#create_owner) | **POST** /v3/tenants/owners | Create an owner
[**delete_owner**](OwnersApi.md#delete_owner) | **DELETE** /v3/tenants/owners/{email} | Delete owner
[**get_owner**](OwnersApi.md#get_owner) | **GET** /v3/tenants/owners/{email} | Get owner details
[**list_owners**](OwnersApi.md#list_owners) | **GET** /v3/tenants/owners | List owners



## create_owner

> models::CreateOwner201Response create_owner(owner)
Create an owner

Create an owner

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner** | [**Owner**](Owner.md) |  | [required] |

### Return type

[**models::CreateOwner201Response**](create_owner_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_owner

> models::DeleteSite200Response delete_owner(email)
Delete owner

Permenantly delete an owner.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email addres of the owner | [required] |

### Return type

[**models::DeleteSite200Response**](delete_site_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_owner

> models::CreateOwner201Response get_owner(email)
Get owner details

Get details of a specific owner by its email address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of the owner | [required] |

### Return type

[**models::CreateOwner201Response**](create_owner_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_owners

> models::ListOwners200Response list_owners(limit, offset)
List owners

List owners

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of records returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListOwners200Response**](list_owners_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \LdapsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ldap**](LdapsApi.md#create_ldap) | **POST** /v3/tenants/ldaps | Create an ldap
[**delete_ldap**](LdapsApi.md#delete_ldap) | **DELETE** /v3/tenants/ldaps/{ldap_id} | Delete ldap
[**get_ldap**](LdapsApi.md#get_ldap) | **GET** /v3/tenants/ldaps/{ldap_id} | Get ldap details
[**list_ldaps**](LdapsApi.md#list_ldaps) | **GET** /v3/tenants/ldaps | List ldaps



## create_ldap

> models::CreateLdap201Response create_ldap(new_ldap)
Create an ldap

Create an ldap

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_ldap** | [**NewLdap**](NewLdap.md) |  | [required] |

### Return type

[**models::CreateLdap201Response**](create_ldap_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_ldap

> models::DeleteSite200Response delete_ldap(ldap_id)
Delete ldap

Permenantly delete an ldap.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ldap_id** | **String** | Unique ID of the ldap | [required] |

### Return type

[**models::DeleteSite200Response**](delete_site_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ldap

> models::CreateLdap201Response get_ldap(ldap_id)
Get ldap details

Get details of a specific ldap by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ldap_id** | **String** | Unique ID of the ldap | [required] |

### Return type

[**models::CreateLdap201Response**](create_ldap_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ldaps

> models::ListLdaps200Response list_ldaps(limit, offset)
List ldaps

List ldaps

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of records returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListLdaps200Response**](list_ldaps_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \TenantsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ldap**](TenantsApi.md#create_ldap) | **POST** /v3/tenants/ldaps | Create an ldap
[**create_owner**](TenantsApi.md#create_owner) | **POST** /v3/tenants/owners | Create an owner
[**create_tenant**](TenantsApi.md#create_tenant) | **POST** /v3/tenants | Create a tenant.
[**delete_ldap**](TenantsApi.md#delete_ldap) | **DELETE** /v3/tenants/ldaps/{ldap_id} | Delete ldap
[**delete_owner**](TenantsApi.md#delete_owner) | **DELETE** /v3/tenants/owners/{email} | Delete owner
[**get_ldap**](TenantsApi.md#get_ldap) | **GET** /v3/tenants/ldaps/{ldap_id} | Get ldap details
[**get_owner**](TenantsApi.md#get_owner) | **GET** /v3/tenants/owners/{email} | Get owner details
[**get_tenant**](TenantsApi.md#get_tenant) | **GET** /v3/tenants/{tenant_id} | Get tenant details
[**get_tenant_history**](TenantsApi.md#get_tenant_history) | **GET** /v3/tenants/{tenant_id}/history | Get tenant history
[**list_ldaps**](TenantsApi.md#list_ldaps) | **GET** /v3/tenants/ldaps | List ldaps
[**list_owners**](TenantsApi.md#list_owners) | **GET** /v3/tenants/owners | List owners
[**list_tenants**](TenantsApi.md#list_tenants) | **GET** /v3/tenants | List tenants.
[**update_tenant**](TenantsApi.md#update_tenant) | **PUT** /v3/tenants/{tenant_id} | Update a tenant



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


## create_tenant

> models::CreateTenant201Response create_tenant(new_tenant)
Create a tenant.

Create a tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tenant** | [**NewTenant**](NewTenant.md) |  | [required] |

### Return type

[**models::CreateTenant201Response**](create_tenant_201_response.md)

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


## get_tenant

> models::CreateTenant201Response get_tenant(tenant_id)
Get tenant details

Get details of a specific tenant by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Unique ID of the tenant | [required] |

### Return type

[**models::CreateTenant201Response**](create_tenant_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_history

> models::GetTenantHistory200Response get_tenant_history(tenant_id)
Get tenant history

Get history log for a specific tenant by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Unique ID of the tenant | [required] |

### Return type

[**models::GetTenantHistory200Response**](get_tenant_history_200_response.md)

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


## list_tenants

> models::ListTenants200Response list_tenants(limit, offset)
List tenants.

List tenants.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of records returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListTenants200Response**](list_tenants_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tenant

> models::CreateTenant201Response update_tenant(tenant_id, update_tenant)
Update a tenant

Update certain fields on an exsiting tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tenant_id** | **String** | Unique ID of the tenant | [required] |
**update_tenant** | [**UpdateTenant**](UpdateTenant.md) |  | [required] |

### Return type

[**models::CreateTenant201Response**](create_tenant_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


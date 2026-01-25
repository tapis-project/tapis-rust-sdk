# \AdminApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_config**](AdminApi.md#get_config) | **GET** /v3/oauth2/admin/config | 
[**update_config**](AdminApi.md#update_config) | **PUT** /v3/oauth2/admin/config | 



## get_config

> models::GetConfig200Response get_config()


Get the authenticator configuraion for the tenant; restricted to Tenant admins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetConfig200Response**](get_config_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config

> models::GetConfig200Response update_config(new_tenant_config)


Update the authenticator configuraion for the tenant; restricted to Tenant admins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tenant_config** | [**NewTenantConfig**](NewTenantConfig.md) |  | [required] |

### Return type

[**models::GetConfig200Response**](get_config_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


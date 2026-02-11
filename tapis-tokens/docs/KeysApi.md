# \KeysApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**update_keys**](KeysApi.md#update_keys) | **PUT** /v3/tokens/keys | Update the signing key pair for a tenant.



## update_keys

> models::UpdateKeys201Response update_keys(new_signing_keys_request)
Update the signing key pair for a tenant.

Generates a new public/private key pair for token signatures and updates the tenant definition accordingly. Returns the public key.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_signing_keys_request** | [**NewSigningKeysRequest**](NewSigningKeysRequest.md) |  | [required] |

### Return type

[**models::UpdateKeys201Response**](update_keys_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


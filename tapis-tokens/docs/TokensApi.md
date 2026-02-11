# \TokensApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_token**](TokensApi.md#create_token) | **POST** /v3/tokens | Generate a token.
[**refresh_token**](TokensApi.md#refresh_token) | **PUT** /v3/tokens | Generate a new token from a refresh token.
[**revoke_token**](TokensApi.md#revoke_token) | **POST** /v3/tokens/revoke | Revoke a token.
[**update_keys**](TokensApi.md#update_keys) | **PUT** /v3/tokens/keys | Update the signing key pair for a tenant.



## create_token

> models::RefreshToken201Response create_token(new_token_request)
Generate a token.

Generate a token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_token_request** | [**NewTokenRequest**](NewTokenRequest.md) |  | [required] |

### Return type

[**models::RefreshToken201Response**](refresh_token_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> models::RefreshToken201Response refresh_token(refresh_token_request)
Generate a new token from a refresh token.

Generate a new token from a refresh token.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**refresh_token_request** | [**RefreshTokenRequest**](RefreshTokenRequest.md) |  | [required] |

### Return type

[**models::RefreshToken201Response**](refresh_token_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_token

> models::BasicResponse revoke_token(revoke_token_request)
Revoke a token.

Revoke a Tapis JWT. Pass the token to revoke in the body of the request. Once revoked, a token cannot be unrevoked.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revoke_token_request** | [**RevokeTokenRequest**](RevokeTokenRequest.md) |  | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


# \AuthApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_tokens**](AuthApi.md#check_tokens) | **GET** /v3/globus-proxy/auth/check_tokens/{client_id}/{endpoint_id} | Check token pair and refresh as needed
[**get_auth_info**](AuthApi.md#get_auth_info) | **GET** /v3/globus-proxy/auth/url/{client_id}/{endpoint_id} | Return authorization URL given client Id
[**get_tokens**](AuthApi.md#get_tokens) | **GET** /v3/globus-proxy/auth/tokens/{client_id}/{session_id}/{auth_code} | Exchange authorization code for access and refresh tokens



## check_tokens

> models::RespAuthTokens check_tokens(client_id, endpoint_id, access_token, refresh_token)
Check token pair and refresh as needed

Given an endpoint and a pair of tokens refresh the pair as needed. Return the refreshed token pair which may be the same as the provided pair. Access and refresh tokens must be provided as query parameters. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Endpoint Id | [required] |
**access_token** | **String** | Globus transfer access token | [required] |
**refresh_token** | **String** | Globus transfer refresh token | [required] |

### Return type

[**models::RespAuthTokens**](RespAuthTokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_auth_info

> models::RespGlobusAuthInfo get_auth_info(client_id, endpoint_id)
Return authorization URL given client Id

Given a Globus Client Id return the authorization URL that can be used by an end-user to authenticate and obtain a *Native App Authorization Code*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | Globus client associated with the request. | [required] |
**endpoint_id** | **String** | Globus endpoint associated with the request. | [required] |

### Return type

[**models::RespGlobusAuthInfo**](RespGlobusAuthInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tokens

> models::RespAuthTokens get_tokens(client_id, session_id, auth_code)
Exchange authorization code for access and refresh tokens

Exchange a Globus *Native App Authorization Code* for a pair of access and refresh tokens. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** |  | [required] |
**session_id** | **String** | Tapis session Id tracking the OAuth2 flow. | [required] |
**auth_code** | **String** |  | [required] |

### Return type

[**models::RespAuthTokens**](RespAuthTokens.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


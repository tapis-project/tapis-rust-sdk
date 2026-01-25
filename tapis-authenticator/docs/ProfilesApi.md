# \ProfilesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_profile**](ProfilesApi.md#get_profile) | **GET** /v3/oauth2/profiles/{username} | 
[**get_userinfo**](ProfilesApi.md#get_userinfo) | **GET** /v3/oauth2/userinfo | 
[**list_profiles**](ProfilesApi.md#list_profiles) | **GET** /v3/oauth2/profiles | 



## get_profile

> models::GetUserinfo200Response get_profile(username)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | **String** | Username to retrieve. | [required] |

### Return type

[**models::GetUserinfo200Response**](get_userinfo_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_userinfo

> models::GetUserinfo200Response get_userinfo()


Return the user profile associated with the Tapis Token. Also can be used to validate the token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetUserinfo200Response**](get_userinfo_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_profiles

> models::ListProfiles200Response list_profiles(limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of profiles returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListProfiles200Response**](list_profiles_200_response.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


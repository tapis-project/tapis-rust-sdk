# \IdentitiesApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_identity**](IdentitiesApi.md#create_identity) | **POST** /v3/workflows/identities | Create an identity
[**delete_identity**](IdentitiesApi.md#delete_identity) | **DELETE** /v3/workflows/identities/{identity_uuid} | Delete an identity
[**get_identity**](IdentitiesApi.md#get_identity) | **GET** /v3/workflows/identities/{identity_uuid} | Get identity
[**list_identities**](IdentitiesApi.md#list_identities) | **GET** /v3/workflows/identities | List user identities for a given user



## create_identity

> models::RespResourceUrl create_identity(req_identity)
Create an identity

Create a mapping between a group user and an external identity.  Users can only create identities for themselves. i.e., group admins and owners cannot create identities for any user but themselves. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_identity** | [**ReqIdentity**](ReqIdentity.md) | A JSON object specifying information for the pipeline to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_identity

> models::RespString delete_identity(identity_uuid)
Delete an identity

Delete an identitiy 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespString**](RespString.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_identity

> models::RespIdentity get_identity(identity_uuid)
Get identity

Get a specific identity by its UUID 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**identity_uuid** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespIdentity**](RespIdentity.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_identities

> models::RespIdentityList list_identities()
List user identities for a given user

Retrieve identities for the requesting user 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespIdentityList**](RespIdentityList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \SchedulerProfilesApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_scheduler_profile**](SchedulerProfilesApi.md#create_scheduler_profile) | **POST** /v3/systems/schedulerProfile | 
[**delete_scheduler_profile**](SchedulerProfilesApi.md#delete_scheduler_profile) | **DELETE** /v3/systems/schedulerProfile/{name} | 
[**get_scheduler_profile**](SchedulerProfilesApi.md#get_scheduler_profile) | **GET** /v3/systems/schedulerProfile/{name} | 
[**get_scheduler_profiles**](SchedulerProfilesApi.md#get_scheduler_profiles) | **GET** /v3/systems/schedulerProfile | 



## create_scheduler_profile

> models::RespResourceUrl create_scheduler_profile(req_post_scheduler_profile)


Create a scheduler profile using a request body. Name must be unique within a tenant and can be composed of alphanumeric characters and the following special characters [-._~]. Name must begin with an alphabetic character and can be no more than 80 characters in length. Description is optional with a maximum length of 2048 characters.  Note that certain attributes (such as *tenant*) are allowed but ignored so that the JSON result returned by a GET may be modified and used when making a POST request to create a profile. The attributes that are allowed but ignored are    - tenant   - uuid   - created   - updated 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_post_scheduler_profile** | [**ReqPostSchedulerProfile**](ReqPostSchedulerProfile.md) | A JSON object specifying information for the profile to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_scheduler_profile

> models::RespChangeCount delete_scheduler_profile(name)


Remove a scheduler profile given the profile name. Requester must be owner of the profile. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduler_profile

> models::RespSchedulerProfile get_scheduler_profile(name)


Retrieve information for a scheduler profile given the profile name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** |  | [required] |

### Return type

[**models::RespSchedulerProfile**](RespSchedulerProfile.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_scheduler_profiles

> models::RespSchedulerProfiles get_scheduler_profiles()


Retrieve list of scheduler profiles.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespSchedulerProfiles**](RespSchedulerProfiles.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


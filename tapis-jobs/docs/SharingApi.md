# \SharingApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_job_share**](SharingApi.md#delete_job_share) | **DELETE** /jobs/{jobUuid}/share/{user} | 
[**get_job_share**](SharingApi.md#get_job_share) | **GET** /jobs/{jobUuid}/share | 
[**share_job**](SharingApi.md#share_job) | **POST** /jobs/{jobUuid}/share | 



## delete_job_share

> models::RespUnShareJob delete_job_share(job_uuid, user)


Delete all share information of a previously shared job for a specific user  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**user** | **String** |  | [required] |

### Return type

[**models::RespUnShareJob**](RespUnShareJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_share

> models::RespGetJobShareList get_job_share(job_uuid, limit, skip)


Retrieve share information of a job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**models::RespGetJobShareList**](RespGetJobShareList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_job

> models::RespShareJob share_job(job_uuid, req_share_job)


Share a job with a user of the tenant. The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**req_share_job** | [**ReqShareJob**](ReqShareJob.md) |  | [required] |

### Return type

[**models::RespShareJob**](RespShareJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


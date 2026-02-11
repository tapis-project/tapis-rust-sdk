# \ArchivesApi

All URIs are relative to *http://localhost:8000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_archive**](ArchivesApi.md#create_archive) | **POST** /v3/workflows/groups/{group_id}/archives | Create an archive
[**get_archive**](ArchivesApi.md#get_archive) | **GET** /v3/workflows/groups/{group_id}/archives/{archive_id} | Retrieve an archive
[**list_archives**](ArchivesApi.md#list_archives) | **GET** /v3/workflows/groups/{group_id}/archives | Retrieve archives



## create_archive

> models::RespResourceUrl create_archive(group_id, req_archive)
Create an archive

Create an Archive. Archives are used to persist the results of a pipeline run 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**req_archive** | [**ReqArchive**](ReqArchive.md) | A JSON object specifying information for the archive to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceURL.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_archive

> models::RespArchive get_archive(group_id, archive_id)
Retrieve an archive

Retrieve an archive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |
**archive_id** | **String** |  | [required] |

### Return type

[**models::RespArchive**](RespArchive.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_archives

> models::RespArchiveList list_archives(group_id)
Retrieve archives

Retrieve a list of archives for in group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** |  | [required] |

### Return type

[**models::RespArchiveList**](RespArchiveList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


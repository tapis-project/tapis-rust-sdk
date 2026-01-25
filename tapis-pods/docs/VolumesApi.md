# \VolumesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_volume**](VolumesApi.md#create_volume) | **POST** /pods/volumes | create_volume
[**delete_volume**](VolumesApi.md#delete_volume) | **DELETE** /pods/volumes/{volume_id} | delete_volume
[**get_volume**](VolumesApi.md#get_volume) | **GET** /pods/volumes/{volume_id} | get_volume
[**get_volume_contents**](VolumesApi.md#get_volume_contents) | **GET** /pods/volumes/{volume_id}/contents/{path} | get_volume_contents
[**list_volume_files**](VolumesApi.md#list_volume_files) | **GET** /pods/volumes/{volume_id}/list | list_volume_files
[**list_volumes**](VolumesApi.md#list_volumes) | **GET** /pods/volumes | list_volumes
[**update_volume**](VolumesApi.md#update_volume) | **PUT** /pods/volumes/{volume_id} | update_volume
[**upload_to_volume**](VolumesApi.md#upload_to_volume) | **POST** /pods/volumes/{volume_id}/upload/{path} | upload_to_volume



## create_volume

> models::VolumeResponse create_volume(new_volume)
create_volume

Create a volume with inputted information.  Notes: - Author will be given ADMIN level permissions to the volume. - status_requested defaults to \"ON\". So volume will immediately begin creation.  Returns new volume object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_volume** | [**NewVolume**](NewVolume.md) |  | [required] |

### Return type

[**models::VolumeResponse**](VolumeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_volume

> models::DeleteVolumeResponse delete_volume(volume_id)
delete_volume

Delete a volume.  Returns \"\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |

### Return type

[**models::DeleteVolumeResponse**](DeleteVolumeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume

> models::VolumeResponse get_volume(volume_id)
get_volume

Get a volume.  Returns retrieved volume object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |

### Return type

[**models::VolumeResponse**](VolumeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_volume_contents

> serde_json::Value get_volume_contents(volume_id, path, zip)
get_volume_contents

Get file or directory contents as a stream of data from a Tapis Volume.  Use the **zip** query parameter to request directories as a zip archive. This is not allowed if path would result in all files in the volume being included. Please download individual directories, files or objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Unique identifier for the volume. | [required] |
**path** | **String** | Path relative to the volume's root directory. Cannot be empty or /. | [required] |
**zip** | Option<**bool**> | If true, directory contents are compressed using ZIP format. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream, application/zip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volume_files

> models::FilesListResponse list_volume_files(volume_id)
list_volume_files

List files in volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |

### Return type

[**models::FilesListResponse**](FilesListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_volumes

> models::VolumesResponse list_volumes()
list_volumes

Get all volumes in your respective tenant and site that you have READ or higher access to.  Returns a list of volumes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::VolumesResponse**](VolumesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_volume

> models::VolumeResponse update_volume(volume_id, update_volume)
update_volume

Update a volume.  Note: - Fields that change volume source or sink are not modifiable. Please recreate your volume in that case.  Returns updated volume object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** |  | [required] |
**update_volume** | [**UpdateVolume**](UpdateVolume.md) |  | [required] |

### Return type

[**models::VolumeResponse**](VolumeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_to_volume

> models::FilesUploadResponse upload_to_volume(volume_id, path, file)
upload_to_volume

Upload to volume.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**volume_id** | **String** | Unique identifier for the volume. | [required] |
**path** | **String** | Path within the volume where the file will be uploaded. Cannot be empty or /. | [required] |
**file** | **std::path::PathBuf** | The file to upload. | [required] |

### Return type

[**models::FilesUploadResponse**](FilesUploadResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


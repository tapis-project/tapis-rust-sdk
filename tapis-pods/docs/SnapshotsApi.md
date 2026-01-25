# \SnapshotsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_snapshot**](SnapshotsApi.md#create_snapshot) | **POST** /pods/snapshots | create_snapshot
[**delete_snapshot**](SnapshotsApi.md#delete_snapshot) | **DELETE** /pods/snapshots/{snapshot_id} | delete_snapshot
[**get_snapshot**](SnapshotsApi.md#get_snapshot) | **GET** /pods/snapshots/{snapshot_id} | get_snapshot
[**get_snapshot_contents**](SnapshotsApi.md#get_snapshot_contents) | **GET** /pods/snapshots/{snapshot_id}/contents/{path} | get_snapshot_contents
[**list_snapshot_files**](SnapshotsApi.md#list_snapshot_files) | **GET** /pods/snapshots/{snapshot_id}/list | list_snapshot_files
[**list_snapshots**](SnapshotsApi.md#list_snapshots) | **GET** /pods/snapshots | list_snapshots
[**update_snapshot**](SnapshotsApi.md#update_snapshot) | **PUT** /pods/snapshots/{snapshot_id} | update_snapshot



## create_snapshot

> models::SnapshotResponse create_snapshot(new_snapshot)
create_snapshot

Create a snapshot with inputted information.  Notes: - Author will be given ADMIN level permissions to the snapshot.  Returns new snapshot object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_snapshot** | [**NewSnapshot**](NewSnapshot.md) |  | [required] |

### Return type

[**models::SnapshotResponse**](SnapshotResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_snapshot

> models::DeleteSnapshotResponse delete_snapshot(snapshot_id)
delete_snapshot

Delete a snapshot.  Returns \"\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

[**models::DeleteSnapshotResponse**](DeleteSnapshotResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot

> models::SnapshotResponse get_snapshot(snapshot_id)
get_snapshot

Get a snapshot.  Returns retrieved snapshot object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

[**models::SnapshotResponse**](SnapshotResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_snapshot_contents

> serde_json::Value get_snapshot_contents(snapshot_id, path, zip)
get_snapshot_contents

Get file or directory contents as a stream of data from a Tapis Snapshot.  Use the **zip** query parameter to request directories as a zip archive. This is not allowed if path would result in all files in the snapshot being included. Please download individual directories, files or objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** | Unique identifier for the snapshot. | [required] |
**path** | **String** | Path relative to the snapshot's root directory. Cannot be empty or /. | [required] |
**zip** | Option<**bool**> | If true, directory contents are compressed using ZIP format. |  |[default to false]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/octet-stream, application/zip

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshot_files

> models::FilesListResponse list_snapshot_files(snapshot_id)
list_snapshot_files

List files in snapshot.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |

### Return type

[**models::FilesListResponse**](FilesListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_snapshots

> models::SnapshotsResponse list_snapshots()
list_snapshots

Get all snapshots in your respective tenant and site that you have READ or higher access to.  Returns a list of snapshots.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SnapshotsResponse**](SnapshotsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_snapshot

> models::SnapshotResponse update_snapshot(snapshot_id, update_snapshot)
update_snapshot

Update a snapshot.  Note: - Fields that change snapshot source or sink are not modifiable. Please recreate your snapshot in that case.  Returns updated snapshot object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**snapshot_id** | **String** |  | [required] |
**update_snapshot** | [**UpdateSnapshot**](UpdateSnapshot.md) |  | [required] |

### Return type

[**models::SnapshotResponse**](SnapshotResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


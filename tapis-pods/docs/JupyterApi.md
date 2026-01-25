# \JupyterApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ensure_jupyter_pod**](JupyterApi.md#ensure_jupyter_pod) | **GET** /pods/jupyter/ensure | Ensure user has a running Jupyter pod, useful for starting up coding environment
[**upload_to_jupyter**](JupyterApi.md#upload_to_jupyter) | **POST** /pods/jupyter/{pod_id}/upload | Upload a document to the user's Jupyter pod



## ensure_jupyter_pod

> serde_json::Value ensure_jupyter_pod()
Ensure user has a running Jupyter pod, useful for starting up coding environment

Ensure the current user has a running Jupyter pod. If not, create a new one named '{username}jupyter' from the base Jupyter template. Returns pod name and URL.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_to_jupyter

> serde_json::Value upload_to_jupyter(pod_id, file, path)
Upload a document to the user's Jupyter pod

Upload a document to the user's running Jupyter pod using the Jupyter API. Input: multipart form (file), and 'path' (destination in Jupyter).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**path** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


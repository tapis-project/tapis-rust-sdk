# \ManageViewsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_view**](ManageViewsApi.md#create_view) | **POST** /v3/pgrest/manage/views | create_view
[**delete_view**](ManageViewsApi.md#delete_view) | **DELETE** /v3/pgrest/manage/views/{view_name} | delete_view
[**get_manage_view**](ManageViewsApi.md#get_manage_view) | **GET** /v3/pgrest/manage/views/{view_name} | get_manage_view
[**list_views**](ManageViewsApi.md#list_views) | **GET** /v3/pgrest/manage/views | list_views
[**refresh_materialized_view**](ManageViewsApi.md#refresh_materialized_view) | **GET** /v3/pgrest/manage/views/{view_name}/refresh | refresh_materialized_view



## create_view

> models::CreateView201Response create_view(new_view)
create_view

Create a view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_view** | [**NewView**](NewView.md) |  | [required] |

### Return type

[**models::CreateView201Response**](create_view_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_view

> models::BasicResponse delete_view(view_name)
delete_view

Delete a specific view. WARNING -- this action cannot be undone.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_name** | **String** | The name of the view to delete. | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_manage_view

> models::CreateView201Response get_manage_view(view_name, details)
get_manage_view

Get details of a specific view.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_name** | **String** | The id of the view. | [required] |
**details** | Option<**bool**> | Get additional details about the view |  |

### Return type

[**models::CreateView201Response**](create_view_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_views

> models::ListViews200Response list_views()
list_views

List views in the tenant's schema.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ListViews200Response**](list_views_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_materialized_view

> models::BasicResponse refresh_materialized_view(view_name)
refresh_materialized_view

Refresh materialized view (views created with the materialized_view_raw_sql attribute).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**view_name** | **String** | The id of the view. | [required] |

### Return type

[**models::BasicResponse**](BasicResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \SitesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_site**](SitesApi.md#create_site) | **POST** /v3/sites | Create a site.
[**delete_site**](SitesApi.md#delete_site) | **DELETE** /v3/sites/{site_id} | Delete a site
[**get_site**](SitesApi.md#get_site) | **GET** /v3/sites/{site_id} | Get site details
[**list_sites**](SitesApi.md#list_sites) | **GET** /v3/sites | List sites.



## create_site

> models::CreateSite201Response create_site(new_site)
Create a site.

Create a site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_site** | [**NewSite**](NewSite.md) |  | [required] |

### Return type

[**models::CreateSite201Response**](create_site_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_site

> models::DeleteSite200Response delete_site(site_id)
Delete a site

Permenantly delete a site.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Unique ID of the site | [required] |

### Return type

[**models::DeleteSite200Response**](delete_site_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_site

> models::CreateSite201Response get_site(site_id)
Get site details

Get details of a specific site by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**site_id** | **String** | Unique ID of the site | [required] |

### Return type

[**models::CreateSite201Response**](create_site_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_sites

> models::ListSites200Response list_sites(limit, offset)
List sites.

List sites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | limit the number of records returned. |  |
**offset** | Option<**i32**> | index (offset) to start list. |  |

### Return type

[**models::ListSites200Response**](list_sites_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


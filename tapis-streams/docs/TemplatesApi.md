# \TemplatesApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_template**](TemplatesApi.md#create_template) | **POST** /v3/streams/templates | Create template.
[**get_template**](TemplatesApi.md#get_template) | **GET** /v3/streams/templates/{template_id} | Get templates.
[**list_templates**](TemplatesApi.md#list_templates) | **GET** /v3/streams/templates | List templates.
[**update_template**](TemplatesApi.md#update_template) | **PUT** /v3/streams/templates/{template_id} | Update template.



## create_template

> models::ListTemplates200Response create_template(new_template)
Create template.

Create template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_template** | [**NewTemplate**](NewTemplate.md) |  | [required] |

### Return type

[**models::ListTemplates200Response**](list_templates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template

> models::ListTemplates200Response get_template(template_id)
Get templates.

Get template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | Unique template id | [required] |

### Return type

[**models::ListTemplates200Response**](list_templates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> models::ListTemplates200Response list_templates(query, limit, skip)
List templates.

List templates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | a formated query string for channel. |  |
**limit** | Option<**i32**> | limit the number of records returned. |  |
**skip** | Option<**i32**> | index (skip) to start list. |  |

### Return type

[**models::ListTemplates200Response**](list_templates_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_template

> models::UpdateTemplate201Response update_template(template_id, new_template)
Update template.

Update template.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** | Unique template id | [required] |
**new_template** | [**NewTemplate**](NewTemplate.md) |  | [required] |

### Return type

[**models::UpdateTemplate201Response**](update_template_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \TemplatesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_template**](TemplatesApi.md#add_template) | **POST** /pods/templates | add_template
[**add_template_tag**](TemplatesApi.md#add_template_tag) | **POST** /pods/templates/{template_id}/tags | add_template_tag
[**delete_template**](TemplatesApi.md#delete_template) | **DELETE** /pods/templates/{template_id} | delete_template
[**get_template**](TemplatesApi.md#get_template) | **GET** /pods/templates/{template_id} | get_template
[**get_template_tag**](TemplatesApi.md#get_template_tag) | **GET** /pods/templates/{template_id}/tags/{tag_id} | get_template_tag
[**list_template_tags**](TemplatesApi.md#list_template_tags) | **GET** /pods/templates/{template_id}/tags | list_template_tags
[**list_templates**](TemplatesApi.md#list_templates) | **GET** /pods/templates | list_templates
[**list_templates_and_tags**](TemplatesApi.md#list_templates_and_tags) | **GET** /pods/templates/tags | list_templates_and_tags
[**update_template**](TemplatesApi.md#update_template) | **PUT** /pods/templates/{template_id} | update_template



## add_template

> models::TemplateResponse add_template(new_template)
add_template

Add a template with inputted information.  Returns new template object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_template** | [**NewTemplate**](NewTemplate.md) |  | [required] |

### Return type

[**models::TemplateResponse**](TemplateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_template_tag

> models::TemplateTagResponse add_template_tag(template_id, new_template_tag)
add_template_tag

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**new_template_tag** | [**NewTemplateTag**](NewTemplateTag.md) |  | [required] |

### Return type

[**models::TemplateTagResponse**](TemplateTagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_template

> models::TemplateDeleteResponse delete_template(template_id)
delete_template

Delete a template.  Returns \"\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**models::TemplateDeleteResponse**](TemplateDeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template

> models::TemplateResponse get_template(template_id)
get_template

Get a templates.  Returns retrieved templates object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |

### Return type

[**models::TemplateResponse**](TemplateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_template_tag

> models::TemplateTagsResponse get_template_tag(template_id, tag_id)
get_template_tag

Get a specific tag entry the template has  Returns the tag entry

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**tag_id** | **String** |  | [required] |

### Return type

[**models::TemplateTagsResponse**](TemplateTagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_template_tags

> models::TemplateTagsResponse list_template_tags(template_id, full)
list_template_tags

List tag entries the template has  Returns the ledger of template tags

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**full** | Option<**bool**> | Return pod_definition in tag when full=true |  |[default to true]

### Return type

[**models::TemplateTagsResponse**](TemplateTagsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates

> models::TemplatesResponse list_templates()
list_templates

Get all templates allowed globally + in respective tenant + for specific user. Returns a list of templates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::TemplatesResponse**](TemplatesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_templates_and_tags

> serde_json::Value list_templates_and_tags(full)
list_templates_and_tags

Get all templates and their tags for the user. Returns a dictionary with templates and their tags.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**full** | Option<**bool**> | Returns tag pod_definition with tag when full=true |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_template

> models::TemplateResponse update_template(template_id, update_template)
update_template

Update a template.  Note: - Fields that change template id cannot be modified. Please recreate your template in that case.  Returns updated template object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_id** | **String** |  | [required] |
**update_template** | [**UpdateTemplate**](UpdateTemplate.md) |  | [required] |

### Return type

[**models::TemplateResponse**](TemplateResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


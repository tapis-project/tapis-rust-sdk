# \ImagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_image**](ImagesApi.md#add_image) | **POST** /pods/images | add_image
[**add_images**](ImagesApi.md#add_images) | **POST** /pods/images/bulk | add_images
[**delete_image**](ImagesApi.md#delete_image) | **DELETE** /pods/images/{image_id} | delete_image
[**get_image**](ImagesApi.md#get_image) | **GET** /pods/images/{image_id} | get_image
[**get_images**](ImagesApi.md#get_images) | **GET** /pods/images | get_images



## add_image

> models::ImageResponse add_image(new_image)
add_image

Add a image with inputted information.  Returns new image object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_image** | [**NewImage**](NewImage.md) |  | [required] |

### Return type

[**models::ImageResponse**](ImageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_images

> models::ImagesResponse add_images(new_image, skip_duplicates)
add_images

Add multiple images with inputted information.  Returns new image objects.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_image** | [**Vec<models::NewImage>**](NewImage.md) |  | [required] |
**skip_duplicates** | Option<**bool**> | Whether to skip duplicates or fail on duplicates. |  |[default to false]

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_image

> models::ImageDeleteResponse delete_image(image_id)
delete_image

Delete an image.  Returns \"\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** |  | [required] |

### Return type

[**models::ImageDeleteResponse**](ImageDeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_image

> models::ResponseGetImage get_image(image_id)
get_image

Get an image.  Returns retrieved image object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**image_id** | **String** |  | [required] |

### Return type

[**models::ResponseGetImage**](Response_Get_Image.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_images

> models::ImagesResponse get_images()
get_images

Get all images allowed globally + in respective tenant.  Returns a list of images.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ImagesResponse**](ImagesResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


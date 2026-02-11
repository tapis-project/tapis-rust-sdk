# \PostItsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_post_it**](PostItsApi.md#create_post_it) | **POST** /v3/files/postits/{systemId}/{path} | Create a PostIt
[**delete_post_it**](PostItsApi.md#delete_post_it) | **DELETE** /v3/files/postits/{postitId} | Delete a PostIt
[**get_post_it**](PostItsApi.md#get_post_it) | **GET** /v3/files/postits/{postitId} | Get PostIt.
[**list_post_its**](PostItsApi.md#list_post_its) | **GET** /v3/files/postits | List PostIts.
[**redeem_post_it**](PostItsApi.md#redeem_post_it) | **GET** /v3/files/postits/redeem/{postitId} | Redeem PostIt.
[**update_post_it**](PostItsApi.md#update_post_it) | **PATCH** /v3/files/postits/{postitId} | Modify a PostIt



## create_post_it

> models::PostItResponse create_post_it(system_id, path, create_post_it_request)
Create a PostIt

Create a PostIt.  The PostIt will grant access to a file url. The newly created PostIt can be redeemed by anyone without  further authorization.  This will nearly identical to calling the files service getContents endpoint. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** | The name of the system to create the PostIt for. | [required] |
**path** | **String** | Path relative to the system *rootDir* | [required] |
**create_post_it_request** | [**CreatePostItRequest**](CreatePostItRequest.md) | A JSON document describing the PostIt to be created. | [required] |

### Return type

[**models::PostItResponse**](PostItResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_post_it

> models::RespChangeCount delete_post_it(postit_id)
Delete a PostIt

Delete a PostIt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**postit_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_post_it

> models::PostItResponse get_post_it(postit_id)
Get PostIt.

Get a single PostIt.  This does not redeem the PostIt.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**postit_id** | **uuid::Uuid** |  | [required] |

### Return type

[**models::PostItResponse**](PostItResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_post_its

> models::PostItListResponse list_post_its(list_type, limit, order_by, skip, start_after, select)
List PostIts.

Retrieve a list of all PostIts.  Use *listType* and *select* query  parameters to limit results. Query parameter *listType* allows for filtering  results based on authorization. Options for *listType* are   - *OWNED* Include only items owned by requester (Default)   - *ALL* Include all items requester is authorized to view. (Tenant admins can view all PostIts in their tenant). 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type** | Option<[**ListTypeEnum**](ListTypeEnum.md)> |  |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=&lt;postitid&gt; |  |
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,owner,path |  |[default to summaryAttributes]

### Return type

[**models::PostItListResponse**](PostItListResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## redeem_post_it

> redeem_post_it(postit_id, zip, download)
Redeem PostIt.

Redeem a PostIt.  This will return the file that is pointed to by the PostIt.  No authentication is required. If the *zip* query param is provided it controls if the content is zipped or not.  If zip is not provided, it defaults to false unless the path pointed to by the PostIt is a directory.  In the case of a directory, the default is zip=true. Directories must by redeemed in zipped format, so either accept the default, or specify zip=true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**postit_id** | **uuid::Uuid** |  | [required] |
**zip** | Option<**bool**> | Indicates a zip output stream should be provided.  If zip is not provided it defaults to false unless the path is a directory.  In the case of a directory the content will be zipped. |  |
**download** | Option<**bool**> | If set to true, this will force a browser to initiate a file download.  If set to false, the content-disposition header will be set to inline causing the browser to render the document.  If download is not provided it defaults to false. |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_post_it

> models::PostItResponse update_post_it(postit_id, update_post_it_request)
Modify a PostIt

Update selected fields of a PostIt.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**postit_id** | **uuid::Uuid** |  | [required] |
**update_post_it_request** | [**UpdatePostItRequest**](UpdatePostItRequest.md) | A JSON document describing the PostIt to be updated. | [required] |

### Return type

[**models::PostItResponse**](PostItResponse.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \ApplicationsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_app_owner**](ApplicationsApi.md#change_app_owner) | **POST** /v3/apps/{appId}/changeOwner/{userName} | 
[**create_app_version**](ApplicationsApi.md#create_app_version) | **POST** /v3/apps | 
[**delete_app**](ApplicationsApi.md#delete_app) | **POST** /v3/apps/{appId}/delete | 
[**disable_app**](ApplicationsApi.md#disable_app) | **POST** /v3/apps/{appId}/disable | 
[**disable_app_version**](ApplicationsApi.md#disable_app_version) | **POST** /v3/apps/{appId}/{appVersion}/disable | 
[**enable_app**](ApplicationsApi.md#enable_app) | **POST** /v3/apps/{appId}/enable | 
[**enable_app_version**](ApplicationsApi.md#enable_app_version) | **POST** /v3/apps/{appId}/{appVersion}/enable | 
[**get_app**](ApplicationsApi.md#get_app) | **GET** /v3/apps/{appId}/{appVersion} | 
[**get_app_latest_version**](ApplicationsApi.md#get_app_latest_version) | **GET** /v3/apps/{appId} | 
[**get_apps**](ApplicationsApi.md#get_apps) | **GET** /v3/apps | 
[**get_history**](ApplicationsApi.md#get_history) | **GET** /v3/apps/{appId}/history | 
[**is_enabled**](ApplicationsApi.md#is_enabled) | **GET** /v3/apps/{appId}/isEnabled | 
[**lock_app**](ApplicationsApi.md#lock_app) | **POST** /v3/apps/{appId}/{appVersion}/lock | 
[**patch_app**](ApplicationsApi.md#patch_app) | **PATCH** /v3/apps/{appId}/{appVersion} | 
[**put_app**](ApplicationsApi.md#put_app) | **PUT** /v3/apps/{appId}/{appVersion} | 
[**search_apps_query_parameters**](ApplicationsApi.md#search_apps_query_parameters) | **GET** /v3/apps/search | 
[**search_apps_request_body**](ApplicationsApi.md#search_apps_request_body) | **POST** /v3/apps/search | 
[**undelete_app**](ApplicationsApi.md#undelete_app) | **POST** /v3/apps/{appId}/undelete | 
[**unlock_app**](ApplicationsApi.md#unlock_app) | **POST** /v3/apps/{appId}/{appVersion}/unlock | 



## change_app_owner

> models::RespChangeCount change_app_owner(app_id, user_name)


Change owner of an application. Applies to all versions. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_app_version

> models::RespResourceUrl create_app_version(req_post_app)


Create an application using a request body. App id+version must be unique within tenant and can be composed of alphanumeric characters and the following special characters [-._~]. Id must begin with an alphanumeric character and can be no more than 80 characters in length.  Note that certain attributes (such as tenant) are allowed but ignored so that the JSON result returned by a GET may be modified and used when making a POST request to create an application. The attributes that are allowed but ignored are    - tenant   - uuid   - isPublic   - sharedWithUsers   - sharedAppCtx   - deleted   - created   - updated 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_post_app** | [**ReqPostApp**](ReqPostApp.md) | A JSON object specifying information for the app to be created. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app

> models::RespChangeCount delete_app(app_id)


Mark an application as deleted. Application will not appear in queries unless explicitly requested. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_app

> models::RespChangeCount disable_app(app_id)


Mark an application unavailable for use. Applies to all versions. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_app_version

> models::RespChangeCount disable_app_version(app_id, app_version)


Mark a specific version of an application unavailable for use. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_app

> models::RespChangeCount enable_app(app_id)


Mark an application available for use. Applies to all versions. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_app_version

> models::RespChangeCount enable_app_version(app_id, app_version)


Mark a specific version of an application available for use. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app

> models::RespApp get_app(app_id, app_version, require_exec_perm, impersonation_id, select, resource_tenant)


Retrieve information for an application given the application Id and version. In the result the attribute *sharedAppCtx* indicates if the application is available to the user because it has been shared with the user. The value of *sharedAppCtx* will be the grantor, the Tapis user who shared the application.  Certain Tapis services or a tenant administrator may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |
**require_exec_perm** | Option<**bool**> | check for EXECUTE permission as well as READ permission. |  |[default to false]
**impersonation_id** | Option<**String**> | Restricted. Only certain services may impersonate a Tapis user. |  |
**select** | Option<**String**> | List of attributes to be included as part of the result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,version,owner |  |[default to allAttributes]
**resource_tenant** | Option<**String**> | Restricted. May be used by Tapis services to set the tenant associated with the requested resource. |  |

### Return type

[**models::RespApp**](RespApp.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_latest_version

> models::RespApp get_app_latest_version(app_id, require_exec_perm, select, resource_tenant, impersonation_id)


Retrieve latest version of an application. In the result the attribute *sharedAppCtx* indicates if the application is available to the user because it has been shared with the user. The value of *sharedAppCtx* will be the grantor, the Tapis user who shared the application.  Certain Tapis services or a tenant administrator may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**require_exec_perm** | Option<**bool**> | check for EXECUTE permission as well as READ permission. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of the result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,version,owner |  |[default to allAttributes]
**resource_tenant** | Option<**String**> | Restricted. May be used by Tapis services to set the tenant associated with the requested resource. |  |
**impersonation_id** | Option<**String**> | Restricted. Only certain Tapis services or a tenant administrator may impersonate a Tapis user. |  |

### Return type

[**models::RespApp**](RespApp.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apps

> models::RespApps get_apps(search, list_type, limit, order_by, skip, start_after, compute_total, select, show_deleted, impersonation_id)


Retrieve list of applications.  Use *listType*, *search* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Options for *listType* are    - *OWNED* Include only items owned by requester (Default)   - *SHARED_PUBLIC* Include only items shared publicly   - *SHARED_DIRECT* Include only items shared directly with requester. Exclude publicly shared items.   - *READ_PERM* Include only items for which requester was granter READ or MODIFY permission.   - *MINE* Include items owned or shared directly with requester. Exclude publicly shared items.   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission.  Certain Tapis services or a tenant administrator may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search conditions as a single query parameter. For example search=(id.like.MyApp*)~(enabled.eq.true) |  |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.app1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,version,owner |  |[default to summaryAttributes]
**show_deleted** | Option<**bool**> | Indicates if Applications marked as deleted should be shown in the results. Default is false. |  |[default to false]
**impersonation_id** | Option<**String**> | Restricted. Only certain Tapis services or a tenant administrator may impersonate a Tapis user. |  |

### Return type

[**models::RespApps**](RespApps.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history

> models::RespAppHistory get_history(app_id)


Retrieve history of changes for a given appId. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespAppHistory**](RespAppHistory.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_enabled

> models::RespBoolean is_enabled(app_id, version)


Check if an application is currently enabled, i.e. available for use. If the query parameter *version* is specified then both the top level attribute *enabled* and the version specific attribute *versionEnabled* are checked. Both must be set to *true* for the application to be considered *enabled*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**version** | Option<**String**> | Include specified application version in the check. |  |

### Return type

[**models::RespBoolean**](RespBoolean.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lock_app

> models::RespChangeCount lock_app(app_id, app_version)


Lock a version of an application to prevent updates via PUT or PATCH. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_app

> models::RespResourceUrl patch_app(app_id, app_version, req_patch_app)


Update selected attributes of an existing version of an application. Request body may only contain updatable attributes. Application must exist.  Attributes that may not be updated via PATCH are    - id   - owner   - enabled   - versionEnabled   - locked   - deleted  Note that the attributes owner, enabled, versionEnabled, locked and deleted may be modified using other endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |
**req_patch_app** | [**ReqPatchApp**](ReqPatchApp.md) | A JSON object specifying changes to be applied. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_app

> models::RespResourceUrl put_app(app_id, app_version, req_put_app)


Update all updatable attributes of an application using a request body identical to POST. Application must exist.  Note that certain attributes (such as tenant) are allowed but ignored so that the JSON result returned by a GET may be modified and used when making a PUT request to update.  The attributes that are allowed but ignored for both PUT and POST are    - tenant   - uuid   - isPublic   - sharedWithUsers   - sharedAppCtx   - deleted   - created   - updated  In addition, for a PUT operation, the following non-updatable attributes are allowed but ignored    - id   - version   - owner   - enabled   - versionEnabled   - locked  Note that the attributes owner, enabled, versionEnabled, locked and deleted may be modified using other endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |
**req_put_app** | [**ReqPutApp**](ReqPutApp.md) | A JSON object specifying changes to be applied. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_apps_query_parameters

> models::RespApps search_apps_query_parameters(list_type, limit, order_by, skip, start_after, compute_total, select)


Retrieve details for applications. Use query parameters to specify search conditions. For example ?owner.eq=jdoe&enabled.eq=false Use *listType* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Options for *listType* are    - *OWNED* Include only items owned by requester (Default)   - *SHARED_PUBLIC* Include only items shared publicly   - *SHARED_DIRECT* Include only items shared directly with requester. Exclude publicly shared items.   - *READ_PERM* Include only items for which requester was granter READ or MODIFY permission.   - *MINE* Include items owned or shared directly with requester. Exclude publicly shared items.   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.app2 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,version,owner |  |[default to summaryAttributes]

### Return type

[**models::RespApps**](RespApps.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_apps_request_body

> models::RespApps search_apps_request_body(req_search_apps, list_type, limit, order_by, skip, start_after, compute_total, select)


Retrieve details for applications. Use request body to specify SQL-like search conditions. Use *listType* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Options for *listType* are    - *OWNED* Include only items owned by requester (Default)   - *SHARED_PUBLIC* Include only items shared publicly   - *SHARED_DIRECT* Include only items shared directly with requester. Exclude publicly shared items.   - *READ_PERM* Include only items for which requester was granter READ or MODIFY permission.   - *MINE* Include items owned or shared directly with requester. Exclude publicly shared items.   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_search_apps** | [**ReqSearchApps**](ReqSearchApps.md) | A JSON object specifying SQL-like search conditions as an array of strings. Strings are concatenated to form full search query. | [required] |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.app1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,version,owner |  |[default to summaryAttributes]

### Return type

[**models::RespApps**](RespApps.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_app

> models::RespChangeCount undelete_app(app_id)


Mark an application as not deleted. Application will appear in queries. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unlock_app

> models::RespChangeCount unlock_app(app_id, app_version)


Unlock a version of an application to allow updates via PUT and PATCH. Requester must be owner of the app or a tenant administrator. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**app_id** | **String** |  | [required] |
**app_version** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


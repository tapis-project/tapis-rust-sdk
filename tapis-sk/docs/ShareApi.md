# \ShareApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_share**](ShareApi.md#delete_share) | **DELETE** /security/share | 
[**delete_share_by_id**](ShareApi.md#delete_share_by_id) | **DELETE** /security/share/{id} | 
[**get_share**](ShareApi.md#get_share) | **GET** /security/share/{id} | 
[**get_shares**](ShareApi.md#get_shares) | **GET** /security/share | 
[**has_privilege**](ShareApi.md#has_privilege) | **GET** /security/share/hasPrivilege | 
[**share_resource**](ShareApi.md#share_resource) | **POST** /security/share | 



## delete_share

> models::RespChangeCount delete_share(grantor, grantee, tenant, resource_type, resource_id1, resource_id2, privilege)


Delete a single shared resource by unique attribute selection. The *grantor*, *grantee*, *tenant*, *resourceType*, *resourceId1* and *privilege* parameters are mandatory; *resourceId2* is optional and assumed to be NULL if not provided.  The shared resource is deleted only if it's in the tenant specified in the required *tenant* query parameter. The calling service must also be the same as the orginal service that granted the share.  This call is idempotent. If no share satisfies the above constraints, a success response code is returned and the indicated number of changes is set to zero. When a share is deleted, the indicated number of changes is one.  For the request to be authorized, the requestor must be the Tapis service that originally granted the share. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grantor** | Option<**String**> |  |  |[default to ]
**grantee** | Option<**String**> |  |  |[default to ]
**tenant** | Option<**String**> |  |  |[default to ]
**resource_type** | Option<**String**> |  |  |[default to ]
**resource_id1** | Option<**String**> |  |  |[default to ]
**resource_id2** | Option<**String**> |  |  |[default to ]
**privilege** | Option<**String**> |  |  |[default to ]

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_share_by_id

> models::RespChangeCount delete_share_by_id(id, tenant)


Delete a shared resource by ID. The shared resource is deleted only if it's in the tenant specified in the required *tenant* query parameter. The calling service must also be the same as the orginal service that created the share.  This call is idempotent. If no share satisfies the above constraints, a success response code is returned and the indicated number of changes is set to zero. When a share is deleted, the indicated number of changes is one.  For the request to be authorized, the requestor must be the Tapis service that originally granted the share. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**tenant** | Option<**String**> |  |  |[default to ]

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_share

> models::RespShare get_share(id, tenant)


Get a shared resource by ID. The shared resource is deleted only if it's in the tenant specified in the required *tenant* query parameter.  For the request to be authorized, the requestor must be a Tapis service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**tenant** | Option<**String**> |  |  |[default to ]

### Return type

[**models::RespShare**](RespShare.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_shares

> models::RespShareList get_shares(grantor, grantee, tenant, resource_type, resource_id1, resource_id2, privilege, created_by, created_by_tenant, include_public_grantees, require_null_id2, id)


Get a filtered list of shared resources. Query parameters are used to restrict the returned shares. The *grantor*, *grantee*, *tenant*, *resourceType*, *resourceId1*, *resourceId2*, *privilege*, *createdBy* and *createdByTenant* parameters are used to match values in shared resource objects. Other query parameters are used to control how matching is performed. The *tenant* parameter is required.  If resourceId1 or resourceId2 end with a percent sign (%) wildcard then the search results will include all shares with IDs that begin with the same prefix string. Percent signs embedded elsewhere in the string are *not* recognized as wildcards.  Specifying the *id* parameter causes the other filtering parameters to be ignored. The result list will contain at most one entry.  The *includePublicGrantees* flag, true by default, controls whether resources granted to **~public** and **~public_no_authn** are also considered for inclusion in the result list.  The *requireNullId2* flag, true by default, applies only when no *resourceId2* value is provided. When set, only shared resources that do not specify a *resourceId2* value are considered for inclusion in the result list. By setting this flag to false the caller indicates a \"don't care\" designation on the *resourceId2* value, allowing shares with any *resourceId2* value to be considered for inclusion in the result list.  For the request to be authorized, the requestor must be a Tapis service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grantor** | Option<**String**> |  |  |[default to ]
**grantee** | Option<**String**> |  |  |[default to ]
**tenant** | Option<**String**> |  |  |[default to ]
**resource_type** | Option<**String**> |  |  |[default to ]
**resource_id1** | Option<**String**> |  |  |[default to ]
**resource_id2** | Option<**String**> |  |  |[default to ]
**privilege** | Option<**String**> |  |  |[default to ]
**created_by** | Option<**String**> |  |  |[default to ]
**created_by_tenant** | Option<**String**> |  |  |[default to ]
**include_public_grantees** | Option<**bool**> |  |  |[default to true]
**require_null_id2** | Option<**bool**> |  |  |[default to true]
**id** | Option<**i32**> |  |  |[default to 0]

### Return type

[**models::RespShareList**](RespShareList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## has_privilege

> models::RespBoolean has_privilege(grantee, tenant, resource_type, resource_id1, resource_id2, privilege, exclude_public, exclude_public_no_authn)


Determine if a user has been granted a specific privilege on a specific resource. The *grantee*, *tenant*, *resourceType*, *resourceId1* and *privilege* parameters are mandatory; *resourceId2* is optional and assumed to be NULL if not provided. Privilege matching is performed for the grantee and tenant specified in the query parameters.  True is returned if the user has been granted the privilege, false otherwise.  By default, both authenticated and unauthenticated public privileges are included in the calculation. For example, if a privilege on a resource has been granted to all authenticated users in a tenant (~public), then true will be returned for all users in the tenant.  The *excludePublic* and *excludePublicNoAuthn* parameters can be used to change the default handling of public grants. Either or both types of public grants can be excluded.  For the request to be authorized, the requestor must be a Tapis service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grantee** | Option<**String**> |  |  |[default to ]
**tenant** | Option<**String**> |  |  |[default to ]
**resource_type** | Option<**String**> |  |  |[default to ]
**resource_id1** | Option<**String**> |  |  |[default to ]
**resource_id2** | Option<**String**> |  |  |[default to ]
**privilege** | Option<**String**> |  |  |[default to ]
**exclude_public** | Option<**bool**> |  |  |[default to false]
**exclude_public_no_authn** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::RespBoolean**](RespBoolean.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## share_resource

> models::RespResourceUrl share_resource(req_share_resource)


Share a Tapis resource using a request body. Shared resources allow services to indicate that other services should relax their Tapis authorization checking in certain, well-defined contexts.  Grantees can be given shared access to a resource on an individual basis or by using the public granting mechanism. Grants to the distinguished **~public** and **~public_no_authn** pseudo-grantees allow access to a resource to authenticated users or to any user, respectively.  The payload for this request includes these values, with all except *resourceId2* required:     - grantor    - grantee    - tenant    - resourceType    - resourceId1    - resourceId2    - privilege  If the share already exists, then this call has no effect. For the request to be authorized, the requestor must be a Tapis service. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_share_resource** | [**ReqShareResource**](ReqShareResource.md) |  | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


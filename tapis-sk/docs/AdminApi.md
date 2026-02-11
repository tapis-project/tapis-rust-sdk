# \AdminApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**reinitialize_site**](AdminApi.md#reinitialize_site) | **GET** /security/admin/reinitialize | 



## reinitialize_site

> models::RespAuthorized reinitialize_site()


Request to reinitialize tenants for the site.  This should be called if tenants are added to the site.  When called any site initialization will be run again.  At present this is mainly just asking for a fresh list of the tenants from the tenant service, and reprocessing each tenant (this might include role creation or assignment for tenant admins).  There is code that prevents running this \"too frequently\".  The current default value is 10 minutes, meaning that if you make this request and try to do it again before 10 minutes elapses, the request is ignored.  Only site admins can make this request. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


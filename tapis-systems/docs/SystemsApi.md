# \SystemsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**change_system_owner**](SystemsApi.md#change_system_owner) | **POST** /v3/systems/{systemId}/changeOwner/{userName} | 
[**create_system**](SystemsApi.md#create_system) | **POST** /v3/systems | 
[**delete_system**](SystemsApi.md#delete_system) | **POST** /v3/systems/{systemId}/delete | 
[**disable_system**](SystemsApi.md#disable_system) | **POST** /v3/systems/{systemId}/disable | 
[**enable_system**](SystemsApi.md#enable_system) | **POST** /v3/systems/{systemId}/enable | 
[**get_history**](SystemsApi.md#get_history) | **GET** /v3/systems/{systemId}/history | 
[**get_system**](SystemsApi.md#get_system) | **GET** /v3/systems/{systemId} | 
[**get_systems**](SystemsApi.md#get_systems) | **GET** /v3/systems | 
[**host_eval**](SystemsApi.md#host_eval) | **GET** /v3/systems/{systemId}/hostEval/{envVarName} | 
[**is_enabled**](SystemsApi.md#is_enabled) | **GET** /v3/systems/{systemId}/isEnabled | 
[**match_constraints**](SystemsApi.md#match_constraints) | **POST** /v3/systems/match/constraints | 
[**patch_system**](SystemsApi.md#patch_system) | **PATCH** /v3/systems/{systemId} | 
[**put_system**](SystemsApi.md#put_system) | **PUT** /v3/systems/{systemId} | 
[**search_systems_query_parameters**](SystemsApi.md#search_systems_query_parameters) | **GET** /v3/systems/search | 
[**search_systems_request_body**](SystemsApi.md#search_systems_request_body) | **POST** /v3/systems/search | 
[**undelete_system**](SystemsApi.md#undelete_system) | **POST** /v3/systems/{systemId}/undelete | 



## change_system_owner

> models::RespChangeCount change_system_owner(system_id, user_name)


Change owner of a system.  Please note that existing permission grants and shares will remain.  **WARNING** Note also that no credentials are deleted during this process. So, for example, after the change a system with a static *effectiveUserId* will retain the credentials and host access of the static *effectiveUser*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_system

> models::RespResourceUrl create_system(req_post_system, skip_credential_check)


Create a system using a request body. System name must be unique within a tenant and can be composed of alphanumeric characters and the following special characters [-._~]. Name must begin with an alphanumeric character and can be no more than 80 characters in length. Description is optional with a maximum length of 2048 characters.  The attribute *host* represents a host name, IP address, Globus Endpoint Id or Globus Collection Id.  The attribute *effectiveUserId* determines the host login user, the user used to access the underlying host. The attribute can be set to a static string indicating a specific user (such as a service account) or dynamically specified as *${apiUserId}*. For the case of *${apiUserId}*, the service resolves the variable by extracting the identity from the request to the service (i.e. the JWT) and applying a mapping to a host login user if such a mapping has been provided. If no mapping is provided, then the extracted identity is taken to be the host login user.  If the *effectiveUserId* is static (i.e. not *${apiUserId}*) then credentials may optionally be provided in the *authnCredential* attribute of the request body. Please note that if there is a *loginUser* field in the credential request body, TAPIS will reject the request because the static effective user is always the login user. The Systems service does not store the secrets in its database, they are persisted in the Security Kernel.  By default for LINUX and S3 type systems credentials provided are verified. Use query parameter skipCredentialCheck=true to bypass initial verification of credentials.  The attribute *rootDir* serves as an effective root directory when operating on files through the Tapis Files service. All paths are relative to this directory when using Files to list, copy, move, mkdir, etc. Required for systems of type LINUX or IRODS. Supports the following variables which are resolved at create time: *${apiUserId}*, *${tenant}* and *${owner}*. May not be updated. Contact support to request a change.  There is also a special macro available for *rootDir* that may be used under certain conditions when a system is first created. The macro name is HOST_EVAL. The syntax for the macro is HOST_EVAL($var), where *var* is the environment variable to be evaluated on the system host when the create request is made. Note that the $ character preceding the environment variable name is optional. If after resolution the final path does not have the required leading slash (/) to make it an absolute path, then one will be prepended. The following conditions must be met in order to use the macro   - System must be of type LINUX   - Credentials must be provided when system is created.   - Macro HOST_EVAL() must only appear once and must be the first element of the path. Including a leading slash is optional.   - The *effectiveUserId* for the system must be static. Note that *effectiveUserId* may be set to *${owner}*.  Here are some examples    - HOST_EVAL($SCRATCH)   - HOST_EVAL($HOME)   - /HOST_EVAL(MY_ROOT_DIR)/scratch   - /HOST_EVAL($PROJECT_HOME)/projects/${tenant}/${owner}  Note that certain attributes in the request body (such as tenant) are allowed but ignored so that the JSON result returned by a GET may be modified and used when making a POST request to create a system. The attributes that are allowed but ignored are    - tenant   - uuid   - deleted   - created   - updated 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_post_system** | [**ReqPostSystem**](ReqPostSystem.md) | A JSON object specifying information for the system to be created. | [required] |
**skip_credential_check** | Option<**bool**> | Bypass initial credential validation (for LINUX and S3). Default is false. |  |[default to false]

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_system

> models::RespChangeCount delete_system(system_id)


Mark a system as deleted. System will not appear in queries unless explicitly requested. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_system

> models::RespChangeCount disable_system(system_id)


Mark a system unavailable for use. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_system

> models::RespChangeCount enable_system(system_id)


Mark a system available for use. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_history

> models::RespSystemHistory get_history(system_id)


Retrieve history of changes for a given systemId. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespSystemHistory**](RespSystemHistory.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system

> models::RespSystem get_system(system_id, authn_method, require_exec_perm, select, return_credentials, impersonation_id, shared_app_ctx, resource_tenant)


Retrieve information for a system given the system Id.  Use query parameter *authnMethod* to override the default authentication method.  Certain Tapis services or a tenant administrator may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId*.  Certain Tapis services may use the query parameter *sharedAppCtx* to indicate that the request is in a shared application context. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**authn_method** | Option<**String**> | Desired authentication method to use when fetching credentials, default method used if this is null. |  |[default to ]
**require_exec_perm** | Option<**bool**> | Check for EXECUTE permission as well as READ permission. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of the result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,owner,host |  |[default to allAttributes]
**return_credentials** | Option<**bool**> | Restricted. Only certain Tapis services are authorized to get credentials. |  |[default to false]
**impersonation_id** | Option<**String**> | Restricted. Only certain Tapis services or a tenant administrator may impersonate a Tapis user. |  |
**shared_app_ctx** | Option<**String**> | Restricted. Only certain Tapis services may indicate that the request is in a shared context. Must be set to the grantor who shared the application. |  |
**resource_tenant** | Option<**String**> | Restricted. May be used by Tapis services to set the tenant associated with the requested resource. |  |

### Return type

[**models::RespSystem**](RespSystem.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_systems

> models::RespSystems get_systems(search, list_type, limit, order_by, skip, start_after, compute_total, select, show_deleted, impersonation_id, has_credentials)


Retrieve list of systems.  Use *listType*, *search* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Please note that using *ALL* may have a significant negative impact on performance. Options for *listType* are    - *OWNED* Include only items owned by requester (Default).   - *SHARED_PUBLIC* Include only items shared publicly.   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission. May impact performance.  Use *hasCredentials* boolean query parameter to limit results based on the presence or absence of registered credentials for a system. Filtering is based on registered credentials for the current *defaultAuthnMethod* of a system. Credentials for other authentication method types are ignored. For a dynamic *effectiveUserId* filtering is based on the Tapis user making the request. If the query parameter is absent then no filtering is done.  Certain Tapis services or a tenant administrator may use the query parameter *impersonationId* to be used in place of the requesting Tapis user. Tapis will use this user Id when performing authorization and resolving the *effectiveUserId*. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search conditions as a single query parameter. For example search=(id.like.MySys*)~(enabled.eq.true) |  |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only show systems owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.sys1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,owner,host |  |[default to summaryAttributes]
**show_deleted** | Option<**bool**> | Indicates if Systems marked as deleted should be shown in the results. Default is false. |  |[default to false]
**impersonation_id** | Option<**String**> | Restricted. Only certain Tapis services or a tenant administrator may impersonate a Tapis user. |  |
**has_credentials** | Option<**bool**> | Determines additional filtering based on credentials. If absent no filtering is done. |  |

### Return type

[**models::RespSystems**](RespSystems.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## host_eval

> models::RespName host_eval(system_id, env_var_name)


Resolve environment variable by connecting to host associated with system. Note that this is done for the current *effectiveUserId*. Appropriate valid credentials must have already been registered. The system must be of type LINUX. The provided *{envVarName}* must start with an alphabetic character or underscore followed by 0 or more alphanumeric characters or underscores. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**env_var_name** | **String** |  | [required] |

### Return type

[**models::RespName**](RespName.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## is_enabled

> models::RespBoolean is_enabled(system_id)


Check if a system is currently enabled, i.e. available for use. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespBoolean**](RespBoolean.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## match_constraints

> models::RespSystems match_constraints(req_match_constraints)


**WARNING** *Capability constraint matching is not yet supported.*  Retrieve details for systems. Use request body to specify constraint conditions as an SQL-like WHERE clause. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_match_constraints** | [**ReqMatchConstraints**](ReqMatchConstraints.md) | A JSON object specifying SQL-like constraint conditions as an array of strings. Strings are concatenated to form full query. | [required] |

### Return type

[**models::RespSystems**](RespSystems.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_system

> models::RespResourceUrl patch_system(system_id, req_patch_system)


Update selected attributes of a system. Request body may only contain updatable attributes. System must exist.  Attributes that may not be updated via PATCH are    - id   - systemType   - owner   - enabled   - bucketName   - rootDir   - canExec  Note that the attributes owner and enabled may be modified using other endpoints. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**req_patch_system** | [**ReqPatchSystem**](ReqPatchSystem.md) | A JSON object specifying changes to be applied. | [required] |

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_system

> models::RespResourceUrl put_system(system_id, req_put_system, skip_credential_check)


Update all updatable attributes of a system using a request body identical to POST. System must exist.  Note that certain attributes in the request body (such as tenant) are allowed but ignored so that the JSON result returned by a GET may be modified and used when making a PUT request to update a system.  The attributes that are allowed but ignored for both PUT and POST are   - tenant   - uuid   - deleted   - created   - updated  In addition for a PUT operation the following non-updatable attributes are allowed but ignored   - id   - systemType   - owner   - effectiveUserId   - authnCredential   - enabled   - bucketName   - rootDir   - canExec  Note that the attributes *owner*, *enabled* and *authnCredential* may be modified using other endpoints. Attribute *effectiveUserId* may be updated using the endpoint **patchSystem**. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**req_put_system** | [**ReqPutSystem**](ReqPutSystem.md) | A JSON object specifying changes to be applied. | [required] |
**skip_credential_check** | Option<**bool**> | Bypass initial credential validation (for LINUX and S3). Default is false. |  |[default to false]

### Return type

[**models::RespResourceUrl**](RespResourceUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_systems_query_parameters

> models::RespSystems search_systems_query_parameters(free_form_parameter_name, list_type, limit, order_by, skip, start_after, compute_total, select, has_credentials)


Retrieve details for systems. Use query parameters to specify search conditions. For example owner.eq=jdoe&port.gt=1024 Use *listType* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Please note that using *ALL* may have a significant negative impact on performance. Options for *listType* are    - *OWNED* Include only items owned by requester (Default)   - *SHARED_PUBLIC* Include only items shared publicly   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission. May impact performance.  Use *hasCredentials* boolean query parameter to limit results based on the presence or absence of registered credentials for a system. Filtering is based on registered credentials for the current *defaultAuthnMethod* of a system. Credentials for other authentication method types are ignored. For a dynamic *effectiveUserId* filtering is based on the Tapis user making the request. If the query parameter is absent then no filtering is done. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**free_form_parameter_name** | Option<[**std::collections::HashMap<String, String>**](String.md)> | Free form query parameters. |  |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.sys2 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,owner,host |  |[default to summaryAttributes]
**has_credentials** | Option<**bool**> | Determines additional filtering based on credentials. If absent no filtering is done. |  |

### Return type

[**models::RespSystems**](RespSystems.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_systems_request_body

> models::RespSystems search_systems_request_body(req_search_systems, list_type, limit, order_by, skip, start_after, compute_total, select, has_credentials)


Retrieve details for systems. Use request body to specify SQL-like search conditions. Use *listType* and *select* query parameters to limit results. Query parameter *listType* allows for filtering results based on authorization. Please note that using *ALL* may have a significant negative impact on performance. Options for *listType* are    - *OWNED* Include only items owned by requester (Default)   - *SHARED_PUBLIC* Include only items shared publicly   - *ALL* Include all items requester is authorized to view. Includes check for READ or MODIFY permission. May impact performance.  Use *hasCredentials* boolean query parameter to limit results based on the presence or absence of registered credentials for a system. Filtering is based on registered credentials for the current *defaultAuthnMethod* of a system. Credentials for other authentication method types are ignored. For a dynamic *effectiveUserId* filtering is based on the Tapis user making the request. If the query parameter is absent then no filtering is done. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_search_systems** | [**ReqSearchSystems**](ReqSearchSystems.md) | A JSON object specifying SQL-like search conditions as an array of strings. Strings are concatenated to form full search query. | [required] |
**list_type** | Option<[**ListTypeEnum**](.md)> | Determines additional filtering of results based on ownership, permissions and sharing. Default is to only see items owned by requester. |  |
**limit** | Option<**i32**> | Limit number of items returned. For example limit=10. Use -1 for unlimited. Default is 100. |  |[default to 100]
**order_by** | Option<**String**> | Attribute for sorting. Direction may be included. For example orderBy=id(desc). Default direction is (asc). |  |
**skip** | Option<**i32**> | Number of items to skip. Use one of skip or startAfter. For example skip=10. Default is 0. |  |
**start_after** | Option<**String**> | Where to start when sorting. Use one of skip or startAfter. Must also specify orderBy. For example, limit=10&orderBy=id(asc)&startAfter=my.sys1 |  |
**compute_total** | Option<**bool**> | Compute total number of results that would have been returned if unlimited. Default is false. |  |[default to false]
**select** | Option<**String**> | List of attributes to be included as part of each result item. Keywords *allAttributes* and *summaryAttributes* are supported. For example select=id,owner,host |  |[default to summaryAttributes]
**has_credentials** | Option<**bool**> | Determines additional filtering based on credentials. If absent no filtering is done. |  |

### Return type

[**models::RespSystems**](RespSystems.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_system

> models::RespChangeCount undelete_system(system_id)


Mark a system as not deleted. System will appear in queries. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespChangeCount**](RespChangeCount.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


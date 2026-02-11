# \CredentialsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**check_user_credential**](CredentialsApi.md#check_user_credential) | **POST** /v3/systems/credential/{systemId}/user/{userName}/check | 
[**create_user_credential**](CredentialsApi.md#create_user_credential) | **POST** /v3/systems/credential/{systemId}/user/{userName} | 
[**generate_globus_tokens**](CredentialsApi.md#generate_globus_tokens) | **POST** /v3/systems/credential/{systemId}/user/{userName}/globus/tokens/{authCode}/{sessionId} | Use a Globus authorization code + Tapis session Id to generate tokens
[**get_globus_auth_url**](CredentialsApi.md#get_globus_auth_url) | **GET** /v3/systems/credential/{systemId}/globus/authUrl | Retrieve a Globus URL that can be used to generate an authorization code for an OAuth2 flow.
[**get_user_credential**](CredentialsApi.md#get_user_credential) | **GET** /v3/systems/credential/{systemId}/user/{userName} | 
[**remove_user_credential**](CredentialsApi.md#remove_user_credential) | **DELETE** /v3/systems/credential/{systemId}/user/{userName} | 



## check_user_credential

> models::RespBasic check_user_credential(system_id, user_name, authn_method)


Check user credentials by connecting to the system host. Not supported for all system types. Currently supported for LINUX and S3 type systems.  If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then *{userName}* is interpreted as a Tapis user and a search is made for credentials associated with *{userName}*. Note that the Tapis user *{userName}* may have a mapping to a host *loginUser* in which case the *loginUser* will be used when verifying the credentials. Note that what we call the *Tapis user* comes from the username claim in the Tapis JWT.  If the *effectiveUserId* for the system is static (i.e. not *${apiUserId}*) then *{userName}* is interpreted as the login user to be used when accessing the host. Note that this would typically be the current *effectiveUserId* defined for the system, but that is not a requirement. This allows for registering and checking credentials for a login user prior to updating the system definition.  Operation is allowed if requester is the system owner or a tenant administrator. If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then the operation is allowed if *{userName}* is the Tapis user making the request.  Desired authentication method may be specified using query parameter *authnMethod*. If not specified, then credentials for the system's default authentication method are verified. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**authn_method** | Option<**String**> | Desired authentication method to use when verifying, system default method used if not provided. |  |[default to ]

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_user_credential

> models::RespBasic create_user_credential(system_id, user_name, req_update_credential, create_tms_keys, skip_credential_check)


Create or update credentials in the Security Kernel for given system and target *userName* using a request body. Credentials for multiple authentication methods may be provided. Note that there is support for only one set of credentials per user per system. Updating credentials overwrites previously registered data.  The Systems service does not store the secrets in its database, they are persisted in the Security Kernel.  If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then *{userName}* is interpreted as a Tapis user and the request body may contain the optional attribute *loginUser* which will be used to map the Tapis user to a username to be used when accessing the system. If the login user is not provided then there is no mapping and the Tapis user is always used when accessing the system. Note that what we call the *Tapis user* comes from the username claim in the Tapis JWT.  If the *effectiveUserId* for the system is static (i.e. not *${apiUserId}*) then *{userName}* is interpreted as the login user to be used when accessing the host. Please note that if there is a *loginUser* field in the request body, TAPIS will reject the request because the static effective user is always the login user.  Operation is allowed if requester is the system owner or a tenant administrator. If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then the operation is allowed if *{userName}* is the Tapis user making the request.  Use query parameter *createTmsKeys=true* to have the service call a Trust Manager System (TMS) server to create and store an ssh keypair. Default is *false*. Please note that the following restrictions apply:   - Tapis installation for your site must be configured to support the Trust Manager System (TMS).   - The host for the system must have the sshd configuration set up to use TMS.   - The *effectiveUserId* must be dynamic.   - Mapping of user using *loginUser* is not supported.  By default credentials for LINUX and S3 type systems are verified. Use query parameter *skipCredentialCheck=true* to bypass initial credential validation. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**req_update_credential** | [**ReqUpdateCredential**](ReqUpdateCredential.md) | A JSON object specifying credentials. | [required] |
**create_tms_keys** | Option<**bool**> | Use TMS to generate and store an ssh keypair. Default is false. |  |[default to false]
**skip_credential_check** | Option<**bool**> | Bypass initial credential validation. Default is false. |  |[default to false]

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_globus_tokens

> models::RespBasic generate_globus_tokens(system_id, user_name, auth_code, session_id)
Use a Globus authorization code + Tapis session Id to generate tokens

Use a Globus *Native App Authorization Code* and a Tapis session Id to generate a pair of access and refresh tokens. System must be of type GLOBUS. The Systems service will use the Tapis Security Kernel to store the tokens for the given system and user.  If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then *{userName}* must be set to the Tapis user. Note that what we call the *Tapis user* comes from the username claim in the Tapis JWT.  If the *effectiveUserId* for the system is static (i.e. not *${apiUserId}*) then *{userName}* must be set to the static *effectiveUserId*.  The session Id is a Tapis Id that is used to track the oauth2 flow that is started when a call to the getGlobusAuthUrl endpoint is made. The authorization code, as per Globus documentation, is valid for 10 minutes. Please note that the Tapis installation for your site must be configured by the site administrator to support systems of type GLOBUS. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**auth_code** | **String** | Authorization code to be exchanged for tokens. | [required] |
**session_id** | **String** | Tapis session Id tracking the OAuth2 flow. | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_globus_auth_url

> models::RespGlobusAuthUrl get_globus_auth_url(system_id)
Retrieve a Globus URL that can be used to generate an authorization code for an OAuth2 flow.

Retrieve a Globus URL + Session Id that can be used to generate an oauth2 authorization code associated with the given system. System must be of type GLOBUS. In Globus, the code is referred to as a *Native App Authorization Code*. The host property of the system is used as the Globus Endpoint Id or Globus Collection Id. Once a user has obtained an authorization code, the corresponding Systems endpoint for generating Globus tokens should be called to exchange the code + sessionId for a pair of access and refresh tokens. The session Id is a Tapis Id that is used to track the oauth2 flow that is started when this call is made. The authorization code, as per Globus documentation, is valid for 10 minutes. Please note that the Tapis installation for your site must be configured by the site administrator to support systems of type GLOBUS. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |

### Return type

[**models::RespGlobusAuthUrl**](RespGlobusAuthUrl.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_credential

> models::RespCredential get_user_credential(system_id, user_name, authn_method)


Restricted. Only certain Tapis services authorized.  Retrieve credentials for given system, target *userName* and authentication method.  If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then *{userName}* is interpreted as a Tapis user. Note that there may me a mapping of the Tapis user to a host *loginUser* and what we call the *Tapis user* comes from the username claim in the Tapis JWT.  If the *effectiveUserId* for the system is static (i.e. not *${apiUserId}*) then *{userName}* is interpreted as the host *loginUser* that is used when accessing the host.  Desired authentication method may be specified using query parameter *authnMethod*. If desired authentication method not specified then credentials for the system's default authentication method are returned.  The result includes the attribute *authnMethod* indicating the authentication method associated with the returned credentials. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |
**authn_method** | Option<**String**> | Desired authentication method to use when fetching credentials, default method used if not provided. |  |[default to ]

### Return type

[**models::RespCredential**](RespCredential.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_user_credential

> models::RespBasic remove_user_credential(system_id, user_name)


Remove credentials from the Security Kernel for given system and *target user*. Requester must be owner of the system.  Operation is allowed if requester is the system owner or a tenant administrator. If the *effectiveUserId* for the system is dynamic (i.e. equal to *${apiUserId}*) then the operation is allowed if *{userName}* is the Tapis user making the request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **String** |  | [required] |
**user_name** | **String** |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


# \VaultApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_secret**](VaultApi.md#delete_secret) | **POST** /security/vault/secret/delete/{secretType}/{secretName} | 
[**destroy_secret**](VaultApi.md#destroy_secret) | **POST** /security/vault/secret/destroy/{secretType}/{secretName} | 
[**destroy_secret_meta**](VaultApi.md#destroy_secret_meta) | **DELETE** /security/vault/secret/destroy/meta/{secretType}/{secretName} | 
[**list_secret_meta**](VaultApi.md#list_secret_meta) | **GET** /security/vault/secret/list/meta/{secretType} | 
[**read_secret**](VaultApi.md#read_secret) | **GET** /security/vault/secret/{secretType}/{secretName} | 
[**read_secret_meta**](VaultApi.md#read_secret_meta) | **GET** /security/vault/secret/read/meta/{secretType}/{secretName} | 
[**undelete_secret**](VaultApi.md#undelete_secret) | **POST** /security/vault/secret/undelete/{secretType}/{secretName} | 
[**validate_service_password**](VaultApi.md#validate_service_password) | **POST** /security/vault/secret/validateServicePassword/{secretName} | 
[**validate_site_admin_password**](VaultApi.md#validate_site_admin_password) | **POST** /security/vault/secret/validateSiteAdminPassword/{secretName} | 
[**write_secret**](VaultApi.md#write_secret) | **POST** /security/vault/secret/{secretType}/{secretName} | 



## delete_secret

> models::RespVersions delete_secret(secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Soft delete one or more versions of a secret. Each version can be deleted individually or as part of a group specified in the input array. Deletion can be reversed using the *secret/undelete/{secretName}* endpoint, which make this a _soft_ deletion operation.  The input versions array is interpreted as follows:     * [-] - empty = delete all versions    * [0] - zero = delete only the latest version    * [1, 3, ...] - list = delete the specified versions  A valid tenant and user must also be specified in the body.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**req_versions** | [**ReqVersions**](ReqVersions.md) |  | [required] |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespVersions**](RespVersions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_secret

> models::RespVersions destroy_secret(secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Destroy one or more versions of a secret. Destroy implements a hard delete which delete that cannot be undone. It does not, however, remove any metadata associated with the secret.  The input versions array is interpreted as follows:     * [-] - empty = destroy all versions    * [0] - zero = destroy only the latest version    * [1, 3, ...] - list = destroy the specified versions  A valid tenant and user must be specified in the body.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**req_versions** | [**ReqVersions**](ReqVersions.md) |  | [required] |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespVersions**](RespVersions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## destroy_secret_meta

> models::RespBasic destroy_secret_meta(secret_type, secret_name, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Erase all traces of a secret: its key, all versions of its value and all its metadata. Specifying a folder erases all secrets in that folder.  A valid tenant and user must be specified as query parameters.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_secret_meta

> models::RespSecretList list_secret_meta(secret_type, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice)


List the secret names at the specified path. The path must represent a folder, not an actual secret name. If the path does not have a trailing slash one will be inserted. Secret names should not encode private information.  A valid tenant and user must be specified as query parameters.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the secret name. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* path parameter and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespSecretList**](RespSecretList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_secret

> models::RespSecret read_secret(secret_type, secret_name, tenant, user, version, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Read a versioned secret. By default, the latest version of the secret is read. If the *version* query parameter is specified then that version of the secret is read. The *version* parameter should be passed as an integer with zero indicating the latest version of the secret. A NOT FOUND status code is returned if the secret version does not exist or if it's deleted or destroyed.  The response object includes the map of zero or more key/value pairs and metadata that describes the secret. The metadata includes which version of the secret was returned.  A valid tenant and user must be specified as query parameters.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**version** | Option<**i32**> |  |  |[default to 0]
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespSecret**](RespSecret.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## read_secret_meta

> models::RespSecretVersionMetadata read_secret_meta(secret_type, secret_name, tenant, user, sysid, sysuser, keytype, dbhost, dbname, dbservice)


List a secret's metadata including its version information. The input parameter must be a secret name, not a folder. The result includes which version of the secret is the latest.  A valid tenant and user must be specified as query parameters.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**tenant** | Option<**String**> |  |  |
**user** | Option<**String**> |  |  |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespSecretVersionMetadata**](RespSecretVersionMetadata.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## undelete_secret

> models::RespVersions undelete_secret(secret_type, secret_name, req_versions, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Restore one or more versions of a secret that have previously been deleted. This endpoint undoes soft deletions performed using the *secret/delete/{secretType}/{secretName}* endpoint.  The input versions array is interpreted as follows:     * [-] - empty = undelete all versions    * [0] - zero = undelete only the latest version    * [1, 3, ...] - list = undelete the specified versions  A valid tenant and user must be specified in the body.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**req_versions** | [**ReqVersions**](ReqVersions.md) |  | [required] |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespVersions**](RespVersions.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_service_password

> models::RespAuthorized validate_service_password(secret_name, req_validate_pwd)


Validate a service's password. The JSON payload contains the password that needs to be validated against the password stored in the vault for the service specifie din the X-Tapis-User header. The secret name is the path under which the password was stored.  A valid tenant and user must also be specified in the payload.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  Only services can make this request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** |  | [required] |
**req_validate_pwd** | [**ReqValidatePwd**](ReqValidatePwd.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_site_admin_password

> models::RespAuthorized validate_site_admin_password(secret_name, req_validate_pwd)


Validate a Site Admin's password. The JSON payload contains the password that needs to be validated against the password stored in the vault for the site admin specified in the X-Tapis-User header. The secret name is the path under which the password was stored.  A valid tenant and user must also be specified in the payload.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  Only services can make this request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_name** | **String** |  | [required] |
**req_validate_pwd** | [**ReqValidatePwd**](ReqValidatePwd.md) |  | [required] |

### Return type

[**models::RespAuthorized**](RespAuthorized.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## write_secret

> models::RespSecretMeta write_secret(secret_type, secret_name, req_write_secret, sysid, sysuser, keytype, dbhost, dbname, dbservice)


Create or update a secret. The JSON payload contains a required *data* object and an optional *options* object. It also contains the required tenant and user fields.  The *data* object is a JSON object that contains one or more key/value pairs in which both the key and value are strings. These are the individual secrets that are saved under the path name. The secrets are automatically versioned, which allows a pre-configured number of past secret values to be accessible even after new values are assigned. See the various GET operations for details on how to access different aspects of secrets.  NOTE: The *cas* option is currently ignored but documented here for future reference.  The *options* object can contain a *cas* key and with an integer value that represents a secret version. CAS stands for check-and-set and will check an existing secret's version before updating. If cas is not set the write will be always be allowed. If set to 0, a write will only be allowed if the key doesn’t exist. If the index is greater than zero, then the write will only be allowed if the key’s current version matches the version specified in the cas parameter.  ### Naming Secrets Secrets can be arranged hierarchically by using the \"+\" characters in the *secretName*. These characters will be converted to slashes upon receipt, allowing secrets to be arranged in folders.  A secret is assigned a path name constructed from the *secretType* and *secretName* path parameters and, optionally, from query parameters determined by the *secretType*. Each *secretType* determines a specific transformation from the url path to a path in the vault. The *secretType* may require certain query parameters to be present on the request in order to construct the vault path. See the next section for details.  ### Secret Types The list below documents each *secretType* and their applicable query parameters. Highlighted parameter names indicate required parameters. When present, default values are listed first and also highlighted.    - **system**     - *sysid*: the unique system id     - *sysuser*: the accessing user (except when keytype=cert)     - keytype: *sshkey* | password | accesskey | token | tmskey | cert   - **dbcred**     - *dbhost*:  the DBMS hostname, IP address or alias     - *dbname*:  the database name or alias     - *dbservice*: service name   - **jwtsigning** - *no query parameters*   - **user** - *no query parameters*   - **service** - *no query parameters*  ### Authorization Requestors are authorized based on the secret type specified in the URL path. The following authorizations are enforced:  - system: limited to the systems service - dbcred: any service - jwtsigning: limited to the tokens service - user: any user - service: any service  ### Generating Secrets Passwords and public/private key pairs appropriate for Tapis use can be generated as part of this secret write call. To direct SK to create a secret, assign the special value `<generate-secret>` to a key. When SK detects this value, it generates a password or key pair depending on context, and replaces the `<generate-secret>` text with the generated secret. In the case of a key pair, both the public and private keys are saved.  Key pairs are always generated for secrets of type JWTSigning, while passwords are generated for all other secret types unless the key is named *privateKey*.  To generate a key pair, insert the following key/value pair into the payload's data map:      key=\"privateKey\", value=\"<generate-secret>\"  When the key pair is generated, the above key/value item is replaced by these two key/value pairs:      key=\"privateKey\", value=<private key in pem format>     key=\"publicKey\",  value=<public key in pem format>  In non-JWTSigning secret types, passwords are generated whenever the following key/value pair is encountered in the payload's data map:      key=<name other than privateKey>, value=\"<generate-secret>\"  The generated password simply replaces the item's value and the key name is left unchanged. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_type** | **String** |  | [required] |
**secret_name** | **String** |  | [required] |
**req_write_secret** | [**ReqWriteSecret**](ReqWriteSecret.md) |  | [required] |
**sysid** | Option<**String**> |  |  |
**sysuser** | Option<**String**> |  |  |
**keytype** | Option<**String**> |  |  |[default to sshkey]
**dbhost** | Option<**String**> |  |  |
**dbname** | Option<**String**> |  |  |
**dbservice** | Option<**String**> |  |  |

### Return type

[**models::RespSecretMeta**](RespSecretMeta.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


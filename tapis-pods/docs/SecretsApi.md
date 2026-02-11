# \SecretsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_secret**](SecretsApi.md#create_secret) | **POST** /pods/secrets | create_secret
[**delete_secret**](SecretsApi.md#delete_secret) | **DELETE** /pods/secrets/{secret_id} | delete_secret
[**get_secret**](SecretsApi.md#get_secret) | **GET** /pods/secrets/{secret_id} | get_secret
[**get_secret_value**](SecretsApi.md#get_secret_value) | **GET** /pods/secrets/{secret_id}/value | get_secret_value
[**list_secrets**](SecretsApi.md#list_secrets) | **GET** /pods/secrets | list_secrets
[**update_secret**](SecretsApi.md#update_secret) | **PUT** /pods/secrets/{secret_id} | update_secret



## create_secret

> models::SecretResponse create_secret(new_secret)
create_secret

Create a secret with inputted information.  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Notes: - This endpoint creates new secrets only. Returns 409 Conflict if secret_id already exists. - To update an existing secret, use PUT /pods/secrets/{secret_id}. - Secrets are stored securely in Tapis Security Kernel (SK). - Secret names are automatically namespaced: ``pods_{tenant}_user_{username}_{name}`` - Only initial ADMIN can use this secret in pods unless you grant permissions. - Use ``secret_map`` in pod definitions to inject as environment variables. - All secret operations are logged  Request Body Fields: - **secret_id** (required): Alphanumeric name with underscores/dashes allowed (max 100 chars) - **secret_value** (required): The actual secret value to store securely - **description** (optional): ASCII description (max 500 chars) - **scope** (optional): ``user`` (default) or ``pod`` - determines secret visibility - **pod_id** (optional): Required if scope is ``pod``, must be omitted if scope is ``user`` - **readable** (optional): ``true`` (default) or ``false`` - controls if value can be retrieved via API - **writable** (optional): ``true`` (default) or ``false`` - controls if value can be updated via PUT  Access Mode Combinations: - ``readable=true, writable=true`` (default): Full access - value can be read and updated - ``readable=true, writable=false``: Read-only - value can be read but not updated (write-once) - ``readable=false, writable=true``: Write-only - value can be updated but not read via API - ``readable=false, writable=false``: Locked - value cannot be read or updated, only used via pod injection  Note: Pod injection via ``secret_map`` always works regardless of ``readable`` setting.  Usage in Pods: After creating a secret, reference it in your pod's ``secret_map``; after that, reference it with ${} in the fields that support secrets::      {         \"secret_map\": {             \"DB_PASSWORD\": \"${secret:my_db_secret}\"         },         \"environment_variables\": {             \"DATABASE_URL\": \"postgres://user:${pods:secrets:DB_PASSWORD}@localhost/db\"         }     }  Returns new secret object (without the secret value).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_secret** | [**NewSecret**](NewSecret.md) |  | [required] |

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_secret

> models::SecretDeleteResponse delete_secret(secret_id)
delete_secret

Delete a secret from both the database and Security Kernel (SK).  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Note: - Requires ADMIN permission on the secret (only the creator has this by default). - This operation is permanent and cannot be undone. - Pods currently using this secret will fail to start until the secret reference is removed or a new secret with the same name is created by the pod owner. - All delete operations are logged.  Returns the deleted secret name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |

### Return type

[**models::SecretDeleteResponse**](SecretDeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secret

> models::SecretResponse get_secret(secret_id)
get_secret

Get secret metadata (not the secret value itself).  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Note: - Returns metadata only. Use GET /secrets/{name}/value to retrieve the actual value. - Requires READ permission on the secret.  Returns secret metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secret_value

> models::SecretValueResponse get_secret_value(secret_id)
get_secret_value

Get the actual secret value from SK.  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Note: - Requires USER permission on the secret (higher than READ). - Secrets with ``readable=False`` cannot have their values retrieved via this endpoint. - Pod injection via ``secret_map`` always works regardless of ``readable`` setting. - This operation is logged.  Returns the secret value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |

### Return type

[**models::SecretValueResponse**](SecretValueResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_secrets

> models::SecretsResponse list_secrets()
list_secrets

Get all secrets you have permission to access.  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Note: - Returns metadata only, not secret values (use GET /secrets/{name}/value). - Filter shows only secrets where you have READ+ permission.  Returns a list of secrets (without secret values).

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SecretsResponse**](SecretsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_secret

> models::SecretResponse update_secret(secret_id, update_secret)
update_secret

Update a secret's description and/or value.  25Q4 Feature: Pods Secrets allow secure injection of credentials into pods.  Note: - Requires USER permission on the secret. - Secrets with ``writable=False`` cannot have their values updated (write-once secrets). - Description updates are always allowed regardless of ``writable`` setting. - Updates are logged. - Pods using this secret will get the new value on next start/restart.  Returns updated secret metadata.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**secret_id** | **String** |  | [required] |
**update_secret** | [**UpdateSecret**](UpdateSecret.md) |  | [required] |

### Return type

[**models::SecretResponse**](SecretResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


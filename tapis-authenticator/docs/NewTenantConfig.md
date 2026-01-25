# NewTenantConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allowable_grant_types** | Option<**Vec<String>**> | JSON-serializable list of allowable grant types | [optional]
**use_ldap** | Option<**bool**> | whether to use the LDAP configured in the Tenants API for this tenant. | [optional]
**use_token_webapp** | Option<**bool**> | whether to make the Authenticator token web app available | [optional]
**default_access_token_ttl** | Option<**i32**> | The access token TTL, in seconds, for standard grant types, such as authorization code. | [optional]
**default_refresh_token_ttl** | Option<**i32**> | The refresh token TTL, in seconds, for standard grant types, such as authorization code. | [optional]
**max_access_token_ttl** | Option<**i32**> | The maxiumum access token TTL, in seconds, for grant types that allow the caller to specify the TTL. | [optional]
**max_refresh_token_ttl** | Option<**i32**> | The maxiumum refresh token TTL, in seconds, for grant types that allow the caller to specify the TTL. | [optional]
**mfa_config** | Option<[**serde_json::Value**](.md)> | JSON-serializable object which includes various details such as which MFA system to use (e.g., TACC MFA or another MFA) and configurations for it. | [optional]
**custom_idp_configuration** | Option<[**serde_json::Value**](.md)> | Configuration for customizing the IdP integration, including custom ldap search filters and alternative IdPs like github OAuth of Custos; must be a JSON-serializable object. | [optional]
**token_url** | Option<**String**> | Token URL endpoint for generating v2 token | [optional]
**impers_oauth_client_id** | Option<**String**> | Client ID for impersonation | [optional]
**impers_oauth_client_secret** | Option<**String**> | Client secret for impersonation | [optional]
**impersadmin_username** | Option<**String**> | Impersonation username for impersonation | [optional]
**impersadmin_password** | Option<**String**> | Impersonation password for impersonation | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



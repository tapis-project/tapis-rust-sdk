# NewTokenRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_type** | **String** | The type of account (user or service) represented by the token. | 
**token_tenant_id** | **String** | The tenant associated with the token to be generated. | 
**token_username** | **String** | The username associated with the token to be generated. | 
**target_site_id** | Option<**String**> | The site_id for the site where this token will be used. Only used when account_type is \"service\". If not provided, the site_id for the token_tenant_id will be used. | [optional]
**delegation_token** | Option<**bool**> | Whether the generated token should be a delegation token. | [optional]
**delegation_sub_tenant_id** | Option<**String**> | The tenant_id associated with the subject who used a delegation authority in creating the delegation token. | [optional]
**delegation_sub_username** | Option<**String**> | The username associated with the subject who used a delegation authority in creating the delegation token. | [optional]
**access_token_ttl** | Option<**i32**> | The TTL, in seconds, for the generated token. | [optional]
**generate_refresh_token** | Option<**bool**> | Whether to also generate a refresh token. | [optional]
**refresh_token_ttl** | Option<**i32**> | The TTL, in seconds, for the refresh token (if generated). | [optional]
**claims** | Option<[**serde_json::Value**](.md)> | JSON object of additional claims to add to the standard claims issued with the token. Note - standard claims cannot be modified through this parameter. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# OAuth2Metadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**issuer** | Option<**String**> | The authorization server's issuer identifier. | [optional]
**authorization_endpoint** | Option<**String**> | URL of the authorization server's authorization endpoint. | [optional]
**token_endpoint** | Option<**String**> | URL of the authorization server's token endpoint. | [optional]
**jwks_uri** | Option<**String**> | URL to the public key used to check signatures for the tokens issued by this server. | [optional]
**registration_endpoint** | Option<**String**> | URL of the authorization server's OAuth 2.0 Dynamic Client Registration endpoint | [optional]
**grant_types_supported** | Option<**Vec<String>**> | JSON-serializable list of grant types supported by this server. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



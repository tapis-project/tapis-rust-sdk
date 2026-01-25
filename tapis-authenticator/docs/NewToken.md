# NewToken

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**username** | Option<**String**> | The username being authenticated (for password grant). | [optional]
**password** | Option<**String**> | The password assoicated with the username being authenticated (for password grant). | [optional]
**client_id** | Option<**String**> | The client_id being authenticated (for device_code grant). | [optional]
**client_key** | Option<**String**> | The client_key being authenticated (optional for authorization_code grant). | [optional]
**grant_type** | Option<**String**> | The OAuth2 grant type being used; either password, authorization_code or refresh_token. | [optional]
**redirect_uri** | Option<**String**> | The client's redirect URI (for authorization_code grant). | [optional]
**code** | Option<**String**> | The authorization code associated with the request (for authorization_code grant). | [optional]
**device_code** | Option<**String**> | The device code associated with the request (for device_code grant) | [optional]
**refresh_token** | Option<**String**> | The refresh token associated with the request (for refresh_token grant). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



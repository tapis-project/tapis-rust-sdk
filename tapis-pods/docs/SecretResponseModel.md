# SecretResponseModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**secret_id** | **String** | Name of the secret. | 
**scope** | Option<**String**> | Scope of secret: 'user' or 'pod' | [optional][default to user]
**pod_id** | Option<**String**> |  | [optional]
**description** | Option<**String**> | Description of this secret. | [optional][default to ]
**readable** | Option<**bool**> | If True, secret value can be retrieved via GET /secrets/{id}/value. Pod injection always works regardless of this setting. | [optional][default to true]
**writable** | Option<**bool**> | If True, secret value can be updated via PUT or POST recreation. If False, secret is write-once. | [optional][default to true]
**sk_secret_name** | Option<**String**> | Full secret name used in SK (prefixed). | [optional][default to ]
**creation_ts** | Option<**String**> |  | [optional]
**added_by** | Option<**String**> | User who added this secret. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



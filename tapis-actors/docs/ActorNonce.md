# ActorNonce

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | the id of the nonce | [optional]
**level** | Option<**String**> | The permission level associated with the nonce. | [optional]
**max_uses** | Option<**i32**> | Max uses for the nonce; set to -1 for unlimited uses. | [optional]
**actor_id** | Option<**String**> | the id of the actor | [optional]
**api_server** | Option<**String**> | the base URL for this Abaco instance. | [optional]
**owner** | Option<**String**> | The user who owns the associated actor. | [optional]
**create_time** | Option<**String**> | Time (in UTC) the nonce was created. | [optional]
**remaining_uses** | Option<**i32**> | Number of uses remaining for the nonce; set to -1 for unlimited uses. | [optional]
**roles** | Option<**Vec<String>**> | The roles associated with the nonce. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



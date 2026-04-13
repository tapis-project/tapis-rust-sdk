# ActorExecution

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**actor_id** | Option<**String**> | the id of the actor | [optional]
**api_server** | Option<**String**> | the base URL for this Abaco instance. | [optional]
**owner** | Option<**String**> | The user who owns the associated actor. | [optional]
**cpu** | Option<**i32**> |  | [optional]
**id** | Option<**String**> | the id of the executions | [optional]
**start_time** | Option<**String**> | the time the execution started. | [optional]
**status** | Option<**Status**> | the status of the execution. (enum: SUBMITTED, RUNNING, COMPLETE) | [optional]
**message_received_time** | Option<**String**> | the time the message was originally received by Abaco. | [optional]
**io** | Option<**i32**> |  | [optional]
**runtime** | Option<**i32**> |  | [optional]
**worker_id** | Option<**String**> | the id of the Abaco worker that supervised the execution. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



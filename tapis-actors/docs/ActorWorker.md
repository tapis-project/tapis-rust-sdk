# ActorWorker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ch_name** | Option<**String**> | Identifier for the worker channel; used to send management commands to the worker. | [optional]
**cid** | Option<**String**> | The container id for the worker. | [optional]
**create_time** | Option<**String**> | The time (UTC) the worker was created. | [optional]
**id** | Option<**String**> | unique id for the worker. | [optional]
**image** | Option<**String**> | The docker image for the actor assigned to the worker. | [optional]
**last_execution_time** | Option<**String**> | The last ime (UTC) the worker started an execution. | [optional]
**last_health_check_time** | Option<**String**> | The last ime (UTC) the worker responded to a health check. | [optional]
**location** | Option<**String**> | The location of the docker daemon used by this worker. | [optional]
**status** | Option<**Status**> | Status of the worker. (enum: REQUESTED, SPAWNER SETUP, PULLING IMAGE, CREATING CONTAINER, UPDATING STORE, READY, BUSY, ERROR) | [optional]
**tenant** | Option<**String**> | tenant assoicated with the worker. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



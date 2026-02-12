# Actor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The unique identifier of the actor. | [optional]
**create_time** | Option<**String**> | Time (in UTC) the actor was created. | [optional]
**image** | Option<**String**> | The Docker image associated with the actor. | [optional]
**default_environment** | Option<**serde_json::Value**> | The default environment variables and values for the actor. | [optional]
**description** | Option<**String**> | User provided description of the actor. | [optional]
**last_update_time** | Option<**String**> | Time (in UTC) the actor was last updated. | [optional]
**link** | Option<**String**> | Actor identifier of actor to link this actor's events too. May be an actor id or an alias. Cycles not permitted. | [optional]
**mounts** | Option<[**Vec<models::ActorMount>**](ActorMount.md)> |  | [optional]
**owner** | Option<**String**> | The user who created this actor. | [optional]
**privileged** | Option<**bool**> | Whether the actor runs in privileged mode. | [optional]
**queue** | Option<**String**> | The command channel that this actor uses. | [optional]
**state** | Option<**serde_json::Value**> | Current state for the actor. | [optional]
**stateless** | Option<**bool**> | Whether the actor stores private state between executions. | [optional]
**token** | Option<**bool**> | Whether this actor requires an OAuth token. | [optional]
**status** | Option<**String**> | Current status of the actor. | [optional]
**status_message** | Option<**String**> | Explanation of status. | [optional]
**cron_schedule** | Option<**String**> | String of 'yyyy-mm-dd hh + <number> <unit of time>' format. Turns on cron feature for actor so that at the specified times the actor with execute with a generic static message. | [optional]
**cron_on** | Option<**bool**> | Variable to turn on or off the cron schedule feature for the actor. | [optional]
**cron_next_ex** | Option<**String**> | Next cron execution time for the actor. Only used when `cronOn` is set to True. | [optional]
**r#type** | Option<**Type**> | Return type (none, bin, json) for this actor. Default is none. (enum: none, bin, json) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



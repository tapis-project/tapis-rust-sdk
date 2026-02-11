# NewActor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | **String** | The Docker image associated with the actor. | 
**token** | Option<**bool**> | Whether this actor requires an OAuth token. | [optional][default to false]
**default_environment** | Option<**serde_json::Value**> | The default environment variables and values for the actor. | [optional]
**description** | Option<**String**> | User provided description of the actor. | [optional]
**privileged** | Option<**bool**> | Whether the actor runs in privileged mode (requires admin role). | [optional][default to false]
**queue** | Option<**String**> | The command channel that this actor uses. | [optional]
**stateless** | Option<**bool**> | Whether the actor stores private state between executions. | [optional][default to true]
**link** | Option<**String**> | Actor identifier of actor to link this actor's events too. May be an actor id or an alias. Cycles not permitted. | [optional]
**cron_schedule** | Option<**String**> | String of 'yyyy-mm-dd hh + <number> <unit of time>' format. Turns on cron feature for actor so that at the specified times the actor with execute with a generic static message. | [optional]
**cron_on** | Option<**bool**> | Variable to turn on or off the cron schedule feature for the actor. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



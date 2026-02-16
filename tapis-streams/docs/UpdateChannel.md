# UpdateChannel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**channel_name** | **String** | User defined channel name. | 
**template_id** | Option<**String**> | Template id. (public templates include default_threshold or default_deadman) | [optional]
**r#type** | Option<**Type**> | Type of Checks (threshold or deadman) (enum: threshold, deadman) | [optional][default to Threshold]
**triggers_with_actions** | Option<[**Vec<models::NewChannelTriggersWithActionsInner>**](NewChannelTriggersWithActionsInner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



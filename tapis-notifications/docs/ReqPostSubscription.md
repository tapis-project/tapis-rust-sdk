# ReqPostSubscription

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional][default to ${apiUserId}]
**enabled** | Option<**bool**> |  | [optional][default to true]
**type_filter** | **String** |  | 
**subject_filter** | **String** |  | 
**delivery_targets** | [**Vec<models::DeliveryTarget>**](DeliveryTarget.md) |  | 
**ttl_minutes** | Option<**i32**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



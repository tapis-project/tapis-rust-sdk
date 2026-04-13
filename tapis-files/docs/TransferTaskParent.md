# TransferTaskParent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant_id** | Option<**String**> |  | [optional]
**username** | Option<**String**> |  | [optional]
**source_uri** | Option<**String**> |  | [optional]
**destination_uri** | Option<**String**> |  | [optional]
**total_bytes** | Option<**i64**> |  | [optional]
**bytes_transferred** | Option<**i64**> |  | [optional]
**task_id** | Option<**i32**> |  | [optional]
**children** | Option<[**Vec<models::TransferTaskChild>**](TransferTaskChild.md)> |  | [optional]
**error_message** | Option<**String**> |  | [optional]
**uuid** | Option<**uuid::Uuid**> | Unique ID of the task. | [optional]
**status** | Option<[**models::TransferStatusEnum**](TransferStatusEnum.md)> |  | [optional]
**created** | Option<**String**> |  | [optional]
**start_time** | Option<**String**> |  | [optional]
**end_time** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



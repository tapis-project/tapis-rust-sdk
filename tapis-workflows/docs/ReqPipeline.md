# ReqPipeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**uses** | Option<[**models::Uses**](Uses.md)> |  | [optional]
**archive_ids** | Option<**Vec<String>**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**env** | Option<[**std::collections::HashMap<String, models::EnvSpec>**](EnvSpec.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**params** | Option<[**std::collections::HashMap<String, models::Spec>**](Spec.md)> |  | [optional]
**r#type** | [**models::EnumPipelineType**](EnumPipelineType.md) |  | 
**execution_profile** | Option<[**models::ExecutionProfile**](ExecutionProfile.md)> |  | [optional]
**tasks** | Option<[**Vec<models::ReqTask>**](ReqTask.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



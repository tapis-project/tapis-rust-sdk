# Pipeline

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**uses** | Option<[**models::Uses**](Uses.md)> |  | [optional]
**env** | Option<[**std::collections::HashMap<String, models::EnvSpec>**](EnvSpec.md)> |  | [optional]
**enabled** | Option<**bool**> |  | [optional]
**params** | Option<[**std::collections::HashMap<String, models::Spec>**](Spec.md)> |  | [optional]
**group** | Option<**uuid::Uuid**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**uuid** | Option<**uuid::Uuid**> |  | [optional]
**execution_profile** | Option<[**models::PipelineExecutionProfile**](PipelineExecutionProfile.md)> |  | [optional]
**current_run** | Option<**uuid::Uuid**> |  | [optional]
**last_run** | Option<**uuid::Uuid**> |  | [optional]
**tasks** | Option<[**Vec<models::Task>**](Task.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



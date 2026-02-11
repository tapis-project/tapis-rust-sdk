# BaseTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> |  | [optional]
**r#type** | Option<[**models::EnumTaskType**](EnumTaskType.md)> |  | [optional]
**depends_on** | Option<[**Vec<models::TaskDependency>**](TaskDependency.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**execution_profile** | Option<[**models::TaskExecutionProfile**](TaskExecutionProfile.md)> |  | [optional]
**input** | Option<[**std::collections::HashMap<String, models::SpecWithValue>**](SpecWithValue.md)> |  | [optional]
**output** | Option<[**std::collections::HashMap<String, models::OutputValue>**](Output_value.md)> |  | [optional]
**conditions** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



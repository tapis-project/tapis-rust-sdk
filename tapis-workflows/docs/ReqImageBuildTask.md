# ReqImageBuildTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**r#type** | [**models::EnumTaskType**](EnumTaskType.md) |  | 
**depends_on** | Option<[**Vec<models::TaskDependency>**](TaskDependency.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**execution_profile** | Option<[**models::TaskExecutionProfile**](TaskExecutionProfile.md)> |  | [optional]
**input** | Option<[**std::collections::HashMap<String, models::SpecWithValue>**](SpecWithValue.md)> |  | [optional]
**output** | Option<[**std::collections::HashMap<String, models::OutputValue>**](OutputValue.md)> |  | [optional]
**conditions** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional]
**builder** | [**models::EnumBuilder**](EnumBuilder.md) |  | 
**cache** | Option<**bool**> |  | [optional][default to true]
**context** | [**models::ReqContext**](ReqContext.md) |  | 
**destination** | [**models::ReqDestination**](ReqDestination.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



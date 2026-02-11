# ReqRequestTask

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**r#type** | [**models::EnumTaskType**](EnumTaskType.md) |  | 
**depends_on** | Option<[**Vec<models::TaskDependency>**](TaskDependency.md)> |  | [optional]
**description** | Option<**String**> |  | [optional]
**execution_profile** | Option<[**models::TaskExecutionProfile**](TaskExecutionProfile.md)> |  | [optional]
**input** | Option<[**std::collections::HashMap<String, models::SpecWithValue>**](SpecWithValue.md)> |  | [optional]
**output** | Option<[**std::collections::HashMap<String, models::OutputValue>**](Output_value.md)> |  | [optional]
**conditions** | Option<[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)> |  | [optional]
**auth** | Option<[**serde_json::Value**](.md)> |  | [optional]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**http_method** | [**models::EnumHttpMethod**](EnumHTTPMethod.md) |  | 
**query_params** | Option<[**serde_json::Value**](.md)> |  | [optional]
**url** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



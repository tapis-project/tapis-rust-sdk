# ReqPatchTask

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
**builder** | Option<[**models::EnumBuilder**](EnumBuilder.md)> |  | [optional]
**cache** | Option<**bool**> |  | [optional][default to true]
**context** | Option<[**models::Context**](Context.md)> |  | [optional]
**destination** | Option<[**models::Destination**](Destination.md)> |  | [optional]
**auth** | Option<[**serde_json::Value**](.md)> |  | [optional]
**data** | Option<[**serde_json::Value**](.md)> |  | [optional]
**headers** | Option<[**serde_json::Value**](.md)> |  | [optional]
**http_method** | Option<[**models::EnumHttpMethod**](EnumHTTPMethod.md)> |  | [optional]
**query_params** | Option<[**serde_json::Value**](.md)> |  | [optional]
**url** | Option<**String**> |  | [optional]
**image** | Option<**String**> |  | [optional]
**poll** | Option<**bool**> |  | [optional][default to true]
**tapis_job_def** | Option<[**serde_json::Value**](.md)> |  | [optional]
**tapis_actor_id** | Option<**String**> |  | [optional]
**tapis_actor_message** | Option<**String**> |  | [optional]
**git_repositories** | Option<[**Vec<models::GitCloneDetails>**](GitCloneDetails.md)> |  | [optional]
**runtime** | Option<[**models::EnumRuntimeEnvironment**](EnumRuntimeEnvironment.md)> |  | [optional]
**installer** | Option<[**models::EnumInstaller**](EnumInstaller.md)> |  | [optional]
**command** | Option<**String**> |  | [optional]
**entrypoint** | Option<**String**> |  | [optional]
**code** | Option<**String**> |  | [optional]
**packages** | Option<**Vec<String>**> |  | [optional]
**uses** | Option<[**models::Uses**](Uses.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



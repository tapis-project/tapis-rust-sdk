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
**output** | Option<[**std::collections::HashMap<String, models::OutputValue>**](OutputValue.md)> |  | [optional]
**conditions** | Option<**Vec<std::collections::HashMap<String, serde_json::Value>>**> |  | [optional]
**builder** | Option<[**models::EnumBuilder**](EnumBuilder.md)> |  | [optional]
**cache** | Option<**bool**> |  | [optional][default to true]
**context** | Option<[**models::Context**](Context.md)> |  | [optional]
**destination** | Option<[**models::Destination**](Destination.md)> |  | [optional]
**auth** | Option<**serde_json::Value**> |  | [optional]
**data** | Option<**serde_json::Value**> |  | [optional]
**headers** | Option<**serde_json::Value**> |  | [optional]
**http_method** | Option<[**models::EnumHttpMethod**](EnumHTTPMethod.md)> |  | [optional]
**query_params** | Option<**serde_json::Value**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**image** | Option<**String**> |  | [optional]
**poll** | Option<**bool**> |  | [optional][default to true]
**tapis_job_def** | Option<**serde_json::Value**> |  | [optional]
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



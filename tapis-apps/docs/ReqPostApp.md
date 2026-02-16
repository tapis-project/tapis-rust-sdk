# ReqPostApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** |  | 
**version** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional][default to ${apiUserId}]
**enabled** | Option<**bool**> |  | [optional][default to true]
**version_enabled** | Option<**bool**> |  | [optional][default to true]
**locked** | Option<**bool**> |  | [optional][default to false]
**runtime** | Option<[**models::RuntimeEnum**](RuntimeEnum.md)> |  | [optional]
**runtime_version** | Option<**String**> |  | [optional]
**runtime_options** | Option<[**Vec<models::RuntimeOptionEnum>**](RuntimeOptionEnum.md)> |  | [optional]
**container_image** | **String** |  | 
**job_type** | Option<[**models::JobTypeEnum**](JobTypeEnum.md)> |  | [optional]
**max_jobs** | Option<**i32**> |  | [optional][default to -1]
**max_jobs_per_user** | Option<**i32**> |  | [optional][default to -1]
**strict_file_inputs** | Option<**bool**> |  | [optional][default to false]
**job_attributes** | Option<[**models::JobAttributes**](JobAttributes.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**notes** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



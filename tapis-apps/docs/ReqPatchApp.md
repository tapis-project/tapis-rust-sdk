# ReqPatchApp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**runtime** | Option<[**models::RuntimeEnum**](RuntimeEnum.md)> |  | [optional]
**runtime_version** | Option<**String**> |  | [optional]
**runtime_options** | Option<[**Vec<models::RuntimeOptionEnum>**](RuntimeOptionEnum.md)> |  | [optional]
**container_image** | Option<**String**> |  | [optional]
**job_type** | Option<[**models::JobTypeEnum**](JobTypeEnum.md)> |  | [optional]
**max_jobs** | Option<**i32**> |  | [optional][default to -1]
**max_jobs_per_user** | Option<**i32**> |  | [optional][default to -1]
**strict_file_inputs** | Option<**bool**> |  | [optional][default to false]
**job_attributes** | Option<[**models::JobAttributes**](JobAttributes.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**notes** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



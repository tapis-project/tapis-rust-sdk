# TapisSystem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant** | Option<**String**> |  | [optional]
**id** | Option<**String**> | Short descriptive name for the system that is unique within the tenant. | [optional]
**description** | Option<**String**> | Optional more verbose description. | [optional]
**system_type** | Option<[**models::SystemTypeEnum**](SystemTypeEnum.md)> |  | [optional]
**owner** | Option<**String**> |  | [optional][default to ${apiUserId}]
**host** | Option<**String**> | FQDN, IP address, Globus endpoint ID or Globus collection ID. | [optional]
**enabled** | Option<**bool**> | Indicates if system is currently considered active and available for use. | [optional][default to true]
**effective_user_id** | Option<**String**> | Username to use when accessing the system. A specific user (such as a service account) or the dynamic user ``${apiUserId}``. | [optional][default to ${apiUserId}]
**default_authn_method** | Option<[**models::AuthnEnum**](AuthnEnum.md)> |  | [optional]
**authn_credential** | Option<[**models::Credential**](Credential.md)> |  | [optional]
**bucket_name** | Option<**String**> | Name of the bucket for an S3 type system. | [optional]
**root_dir** | Option<**String**> | Effective root directory to be used when listing files or moving files to and from the system. | [optional]
**port** | Option<**i32**> |  | [optional]
**use_proxy** | Option<**bool**> |  | [optional][default to false]
**proxy_host** | Option<**String**> |  | [optional]
**proxy_port** | Option<**i32**> |  | [optional]
**dtn_system_id** | Option<**String**> | An alternate system to use as a Data Transfer Node (DTN) during job execution. | [optional]
**can_exec** | Option<**bool**> | Indicates if system can be used to execute jobs. | [optional]
**can_run_batch** | Option<**bool**> | Indicates if system supports running jobs using a batch scheduler. | [optional]
**enable_cmd_prefix** | Option<**bool**> |  | [optional][default to false]
**allow_children** | Option<**bool**> | Indicates if system allows for the creation of child systems. | [optional][default to false]
**parent_id** | Option<**String**> | Parent system associated with this child system. | [optional]
**mpi_cmd** | Option<**String**> |  | [optional]
**job_runtimes** | Option<[**Vec<models::JobRuntime>**](JobRuntime.md)> |  | [optional]
**job_working_dir** | Option<**String**> |  | [optional]
**job_env_variables** | Option<[**Vec<models::KeyValuePair>**](KeyValuePair.md)> |  | [optional]
**job_max_jobs** | Option<**i32**> |  | [optional][default to 2147483647]
**job_max_jobs_per_user** | Option<**i32**> |  | [optional][default to 2147483647]
**batch_scheduler** | Option<[**models::SchedulerTypeEnum**](SchedulerTypeEnum.md)> |  | [optional]
**batch_logical_queues** | Option<[**Vec<models::LogicalQueue>**](LogicalQueue.md)> |  | [optional]
**batch_default_logical_queue** | Option<**String**> |  | [optional]
**batch_scheduler_profile** | Option<**String**> |  | [optional]
**job_capabilities** | Option<[**Vec<models::Capability>**](Capability.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**notes** | Option<[**serde_json::Value**](.md)> |  | [optional]
**uuid** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**deleted** | Option<**bool**> |  | [optional]
**created** | Option<**String**> |  | [optional]
**updated** | Option<**String**> |  | [optional]
**has_credentials** | Option<**bool**> |  | [optional][default to false]
**is_public** | Option<**bool**> |  | [optional]
**is_dynamic_effective_user** | Option<**bool**> |  | [optional]
**shared_with_users** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



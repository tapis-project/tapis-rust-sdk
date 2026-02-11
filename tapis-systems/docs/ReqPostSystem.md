# ReqPostSystem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Short descriptive name for the system that is unique within the tenant. | 
**description** | Option<**String**> | Optional more verbose description. | [optional]
**system_type** | [**models::SystemTypeEnum**](SystemTypeEnum.md) |  | 
**owner** | Option<**String**> | A specific user set at system creation. | [optional]
**host** | **String** | FQDN, IP address, Globus endpoint ID or Globus collection ID. | 
**enabled** | Option<**bool**> | Indicates if system is currently considered active and available for use. | [optional][default to true]
**effective_user_id** | Option<**String**> | Username to use when accessing the system. A specific user (such as a service account) or the dynamic user ``${apiUserId}``. | [optional][default to ${apiUserId}]
**default_authn_method** | [**models::AuthnEnum**](AuthnEnum.md) |  | 
**authn_credential** | Option<[**models::ReqPostPutCredential**](ReqPostPutCredential.md)> |  | [optional]
**bucket_name** | Option<**String**> | Name of the bucket for an S3 type system. | [optional]
**root_dir** | Option<**String**> | Effective root directory to be used when listing files or moving files to and from the system. | [optional]
**port** | Option<**i32**> |  | [optional]
**use_proxy** | Option<**bool**> |  | [optional][default to false]
**proxy_host** | Option<**String**> |  | [optional]
**proxy_port** | Option<**i32**> |  | [optional]
**dtn_system_id** | Option<**String**> | An alternate system to use as a Data Transfer Node (DTN) during job execution. | [optional]
**can_exec** | **bool** | Indicates if system can be used to execute jobs. | 
**can_run_batch** | Option<**bool**> | Indicates if system supports running jobs using a batch scheduler. | [optional][default to false]
**enable_cmd_prefix** | Option<**bool**> | Indicates if system allows a job submission request to specify a *cmdPrefix*. | [optional][default to false]
**allow_children** | Option<**bool**> | Indicates if system allows for the creation of child systems. | [optional][default to false]
**mpi_cmd** | Option<**String**> |  | [optional]
**job_runtimes** | Option<[**Vec<models::JobRuntime>**](JobRuntime.md)> |  | [optional]
**job_working_dir** | Option<**String**> |  | [optional]
**job_env_variables** | Option<[**Vec<models::KeyValuePair>**](KeyValuePair.md)> |  | [optional]
**job_max_jobs** | Option<**i32**> |  | [optional]
**job_max_jobs_per_user** | Option<**i32**> |  | [optional]
**batch_scheduler** | Option<[**models::SchedulerTypeEnum**](SchedulerTypeEnum.md)> |  | [optional]
**batch_logical_queues** | Option<[**Vec<models::LogicalQueue>**](LogicalQueue.md)> |  | [optional]
**batch_default_logical_queue** | Option<**String**> |  | [optional]
**batch_scheduler_profile** | Option<**String**> |  | [optional]
**job_capabilities** | Option<[**Vec<models::Capability>**](Capability.md)> |  | [optional]
**tags** | Option<**Vec<String>**> | List of tags as simple strings. | [optional]
**notes** | Option<**serde_json::Value**> | Metadata in the form of a Json object. Not used by Tapis. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



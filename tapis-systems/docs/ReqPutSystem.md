# ReqPutSystem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Optional more verbose description. | [optional]
**host** | Option<**String**> | FQDN, IP address, Globus endpoint ID or Globus collection ID. | [optional]
**default_authn_method** | Option<[**models::AuthnEnum**](AuthnEnum.md)> |  | [optional]
**port** | Option<**i32**> |  | [optional]
**use_proxy** | Option<**bool**> |  | [optional][default to false]
**proxy_host** | Option<**String**> |  | [optional]
**proxy_port** | Option<**i32**> |  | [optional]
**dtn_system_id** | Option<**String**> | An alternate system to use as a Data Transfer Node (DTN) during job execution. | [optional]
**can_run_batch** | Option<**bool**> | Indicates if system supports running jobs using a batch scheduler. | [optional]
**enable_cmd_prefix** | Option<**bool**> |  | [optional][default to false]
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
**tags** | Option<**Vec<String>**> |  | [optional]
**notes** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



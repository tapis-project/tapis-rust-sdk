# JobListDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**uuid** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**owner** | Option<**String**> |  | [optional]
**app_id** | Option<**String**> |  | [optional]
**created** | Option<**String**> |  | [optional]
**status** | Option<**Status**> |  (enum: PENDING, PROCESSING_INPUTS, STAGING_INPUTS, STAGING_JOB, SUBMITTING_JOB, QUEUED, RUNNING, ARCHIVING, BLOCKED, PAUSED, FINISHED, CANCELLED, FAILED) | [optional]
**condition** | Option<**Condition**> |  (enum: CANCELLED_BY_USER, JOB_ARCHIVING_FAILED, JOB_DATABASE_ERROR, JOB_EXECUTION_MONITORING_ERROR, JOB_EXECUTION_MONITORING_ERROR_TIMEOUT, JOB_EXECUTION_MONITORING_TIMEOUT, JOB_FILES_SERVICE_ERROR, JOB_INTERNAL_ERROR, JOB_INVALID_DEFINITION, JOB_LAUNCH_FAILURE, JOB_QUEUE_MONITORING_ERROR, JOB_RECOVERY_FAILURE, JOB_RECOVERY_TIMEOUT, JOB_REMOTE_ACCESS_ERROR, JOB_REMOTE_OUTCOME_ERROR, JOB_UNABLE_TO_STAGE_INPUTS, JOB_UNABLE_TO_STAGE_JOB, JOB_TRANSFER_FAILED_OR_CANCELLED, JOB_TRANSFER_MONITORING_TIMEOUT, NORMAL_COMPLETION, SCHEDULER_CANCELLED, SCHEDULER_DEADLINE, SCHEDULER_OUT_OF_MEMORY, SCHEDULER_STOPPED, SCHEDULER_TIMEOUT, SCHEDULER_TERMINATED) | [optional]
**remote_started** | Option<**String**> |  | [optional]
**ended** | Option<**String**> |  | [optional]
**tenant** | Option<**String**> |  | [optional]
**exec_system_id** | Option<**String**> |  | [optional]
**archive_system_id** | Option<**String**> |  | [optional]
**app_version** | Option<**String**> |  | [optional]
**last_updated** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



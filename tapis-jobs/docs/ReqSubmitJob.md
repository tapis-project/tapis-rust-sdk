# ReqSubmitJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** |  | 
**owner** | Option<**String**> |  | [optional]
**tenant** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**app_id** | **String** |  | 
**app_version** | **String** |  | 
**job_type** | Option<**String**> |  | [optional]
**archive_on_app_error** | Option<**bool**> |  | [optional]
**archive_mode** | Option<[**models::ArchiveModeEnum**](ArchiveModeEnum.md)> |  | [optional]
**dynamic_exec_system** | Option<**bool**> |  | [optional]
**exec_system_id** | Option<**String**> |  | [optional]
**exec_system_exec_dir** | Option<**String**> |  | [optional]
**exec_system_input_dir** | Option<**String**> |  | [optional]
**exec_system_output_dir** | Option<**String**> |  | [optional]
**exec_system_logical_queue** | Option<**String**> |  | [optional]
**archive_system_id** | Option<**String**> |  | [optional]
**archive_system_dir** | Option<**String**> |  | [optional]
**dtn_system_input_dir** | Option<**String**> |  | [optional]
**dtn_system_output_dir** | Option<**String**> |  | [optional]
**node_count** | Option<**i32**> |  | [optional]
**cores_per_node** | Option<**i32**> |  | [optional]
**memory_mb** | Option<**i32**> |  | [optional]
**max_minutes** | Option<**i32**> |  | [optional]
**file_inputs** | Option<[**Vec<models::JobFileInput>**](JobFileInput.md)> |  | [optional]
**file_input_arrays** | Option<[**Vec<models::JobFileInputArray>**](JobFileInputArray.md)> |  | [optional]
**parameter_set** | Option<[**models::JobParameterSet**](JobParameterSet.md)> |  | [optional]
**exec_system_constraints** | Option<**Vec<String>**> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]
**subscriptions** | Option<[**Vec<models::ReqSubscribe>**](ReqSubscribe.md)> |  | [optional]
**is_mpi** | Option<**bool**> |  | [optional]
**mpi_cmd** | Option<**String**> |  | [optional]
**cmd_prefix** | Option<**String**> |  | [optional]
**notes** | Option<**serde_json::Value**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



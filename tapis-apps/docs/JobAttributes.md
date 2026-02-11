# JobAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**dynamic_exec_system** | Option<**bool**> |  | [optional][default to false]
**exec_system_constraints** | Option<**Vec<String>**> |  | [optional]
**exec_system_id** | Option<**String**> |  | [optional]
**exec_system_exec_dir** | Option<**String**> |  | [optional]
**exec_system_input_dir** | Option<**String**> |  | [optional]
**exec_system_output_dir** | Option<**String**> |  | [optional]
**dtn_system_input_dir** | Option<**String**> |  | [optional]
**dtn_system_output_dir** | Option<**String**> |  | [optional]
**exec_system_logical_queue** | Option<**String**> |  | [optional]
**archive_system_id** | Option<**String**> |  | [optional]
**archive_system_dir** | Option<**String**> |  | [optional]
**archive_on_app_error** | Option<**bool**> |  | [optional][default to true]
**archive_mode** | Option<[**models::ArchiveModeEnum**](ArchiveModeEnum.md)> |  | [optional]
**is_mpi** | Option<**bool**> |  | [optional][default to false]
**mpi_cmd** | Option<**String**> |  | [optional]
**cmd_prefix** | Option<**String**> |  | [optional]
**parameter_set** | Option<[**models::ParameterSet**](ParameterSet.md)> |  | [optional]
**file_inputs** | Option<[**Vec<models::AppFileInput>**](AppFileInput.md)> |  | [optional]
**file_input_arrays** | Option<[**Vec<models::AppFileInputArray>**](AppFileInputArray.md)> |  | [optional]
**node_count** | Option<**i32**> |  | [optional]
**cores_per_node** | Option<**i32**> |  | [optional]
**memory_mb** | Option<**i32**> |  | [optional]
**max_minutes** | Option<**i32**> |  | [optional]
**subscriptions** | Option<[**Vec<models::ReqSubscribe>**](ReqSubscribe.md)> |  | [optional]
**tags** | Option<**Vec<String>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



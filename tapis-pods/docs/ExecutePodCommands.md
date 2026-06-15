# ExecutePodCommands

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**commands** | [**models::Commands**](Commands.md) |  | 
**total_timeout** | Option<**i32**> | Total time (sec) to wait for all commands to finish. Default 300 seconds. | [optional]
**command_timeout** | Option<**i32**> | Time (sec) to wait for each command to finish. Default 60 seconds. | [optional]
**fail_on_non_success** | Option<**bool**> | If True, will fail if any command does not return 0. Default True. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



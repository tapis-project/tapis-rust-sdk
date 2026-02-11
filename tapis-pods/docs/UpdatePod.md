# UpdatePod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<**String**> | Which docker image to use, must be on allowlist, check /pods/images for list. | [optional][default to ]
**template** | Option<**String**> | Which pod template to use as base of pod fields. User set attributes will overwrite template fields. | [optional][default to ]
**description** | Option<**String**> |  | [optional]
**command** | Option<**Vec<String>**> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**status_requested** | Option<**String**> |  | [optional]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::VolumeMountsValue>**](VolumeMountsValue.md)> |  | [optional]
**time_to_stop_default** | Option<**i32**> |  | [optional]
**time_to_stop_instance** | Option<**i32**> |  | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::ModelsPodsNetworking>**](ModelsPodsNetworking.md)> |  | [optional]
**resources** | Option<[**models::ModelsPodsResources**](ModelsPodsResources.md)> |  | [optional]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



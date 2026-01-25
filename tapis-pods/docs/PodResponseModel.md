# PodResponseModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pod_id** | **String** | Name of this pod. | 
**image** | Option<**String**> | Which docker image to use, must be on allowlist, check /pods/images for list. | [optional][default to ]
**template** | Option<**String**> | Which pod template to use as base of pod fields. User set attributes will overwrite template fields. | [optional][default to ]
**description** | Option<**String**> | Description of this pod. | [optional][default to ]
**command** | Option<**Vec<String>**> | Command to run in pod. ex. `[\"sleep\", \"5000\"]` or `[\"/bin/bash\", \"-c\", \"(exec myscript.sh)\"]` | [optional]
**arguments** | Option<**Vec<String>**> | Arguments for the Pod's command. | [optional]
**environment_variables** | Option<[**serde_json::Value**](.md)> | Environment variables to inject into k8 pod; Only for custom pods. | [optional][default to {}]
**status_requested** | Option<**String**> | Status requested by user, `ON`, `OFF`, or `RESTART`. | [optional][default to ON]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::ModelsPodsVolumeMount>**](models_pods__VolumeMount.md)> | Key: Volume name. Value: List of strs specifying volume folders/files to mount in pod | [optional][default to {}]
**time_to_stop_default** | Option<**i32**> | Default time (sec) for pod to run from instance start. -1 for unlimited. 12 hour default. | [optional][default to 43200]
**time_to_stop_instance** | Option<**i32**> | Time (sec) for pod to run from instance start. Reset each time instance is started. -1 for unlimited. None uses default. | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::ModelsPodsNetworking>**](models_pods__Networking.md)> | Networking information. `{\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}}` | [optional][default to {default={protocol=http, port=5000}}]
**resources** | Option<[**models::ModelsPodsResources**](models_pods__Resources.md)> | Pod resource management `{\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpus\": 0}` | [optional][default to {}]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]
**time_to_stop_ts** | Option<**String**> | Time (UTC) that this pod is scheduled to be stopped. Change with time_to_stop_instance. | [optional]
**status** | Option<**String**> | Current status of pod. | [optional][default to STOPPED]
**status_container** | Option<[**serde_json::Value**](.md)> | Status of container if exists. Gives phase. | [optional][default to {}]
**creation_ts** | Option<**String**> | Time (UTC) that this pod was created. | [optional]
**update_ts** | Option<**String**> | Time (UTC) that this pod was updated. | [optional]
**start_instance_ts** | Option<**String**> | Time (UTC) that this pod instance was started. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



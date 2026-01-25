# UpdatePod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description of this pod. | [optional][default to ]
**command** | Option<**Vec<String>**> | Command to run in pod. ex. [\"sleep\", \"5000\"] or [\"/bin/bash\", \"-c\", \"(exec myscript.sh)\"] | [optional]
**environment_variables** | Option<[**serde_json::Value**](.md)> | Environment variables to inject into k8 pod; Only for custom pods. | [optional][default to {}]
**status_requested** | Option<**String**> | Status requested by user, `ON`, `OFF`, or `RESTART`. | [optional][default to ON]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::ModelsPodsVolumeMount>**](models_pods__VolumeMount.md)> | Key: Volume name. Value: List of strs specifying volume folders/files to mount in pod | [optional][default to {}]
**time_to_stop_default** | Option<**i32**> | Default time (sec) for pod to run from instance start. -1 for unlimited. 12 hour default. | [optional][default to 43200]
**time_to_stop_instance** | Option<**i32**> | Time (sec) for pod to run from instance start. Reset each time instance is started. -1 for unlimited. None uses default. | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::ModelsPodsNetworking>**](models_pods__Networking.md)> | Networking information. {\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}} | [optional][default to {default={protocol=http, port=5000}}]
**resources** | Option<[**models::ModelsPodsResources**](models_pods__Resources.md)> | Pod resource management {\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpu\": 0} | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



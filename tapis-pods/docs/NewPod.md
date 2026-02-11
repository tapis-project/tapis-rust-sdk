# NewPod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pod_id** | **String** | Name of this pod. | 
**image** | Option<**String**> | Which docker image to use, must be on allowlist, check /pods/images for list. | [optional][default to ]
**template** | Option<**String**> | Which pod template to use as base of pod fields. User set attributes will overwrite template fields. | [optional][default to ]
**description** | Option<**String**> | Description of this pod. | [optional][default to ]
**command** | Option<**Vec<String>**> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**environment_variables** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Environment variables to inject into k8 pod. Use `${pods:secrets:KEY}` to reference secret_map entries. | [optional][default to {}]
**secret_map** | Option<**std::collections::HashMap<String, String>**> | Map of keys to secret values. Syntax: ${secret:name} (user secret), ${secret:user:name} (explicit owner). Reference in environment_variables via ${pods:secrets:KEY}. Resolved at pod start. | [optional][default to {}]
**status_requested** | Option<**String**> | Status requested by user, `ON`, `OFF`, or `RESTART`. | [optional][default to ON]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::VolumeMountsValue>**](Volume_Mounts_value.md)> | Volume mounts keyed by mount_path. Values are VolumeMount objects (see schema) or null (to remove inherited mount). Ex: {\"/data\": {\"type\": \"tapisvolume\", \"source_id\": \"myvolume\"}, \"/etc/config.ini\": {\"type\": \"ephemeral\", \"config_content\": \"key=value\"}} | [optional][default to {}]
**time_to_stop_default** | Option<**i32**> | Default time (sec) for pod to run from instance start. -1 for unlimited. 12 hour default. | [optional][default to 43200]
**time_to_stop_instance** | Option<**i32**> |  | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::ModelsPodsNetworking>**](models_pods__Networking.md)> | Networking information. `{\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}}` | [optional][default to {default={protocol=http, port=5000}}]
**resources** | Option<[**models::ModelsPodsResources**](models_pods__Resources.md)> | Pod resource management `{\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpus\": 0}` | [optional][default to {}]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]
**template_overrides** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



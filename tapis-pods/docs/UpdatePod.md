# UpdatePod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<**String**> | Which docker image to use, must be on allowlist, check /pods/images for list. | [optional][default to ]
**template** | Option<**String**> | Which pod template to use as base of pod fields. User set attributes will overwrite template fields. | [optional][default to ]
**description** | Option<**String**> | Description of this pod. | [optional]
**command** | Option<**Vec<String>**> | Command to run in pod. ex. [\"sleep\", \"5000\"] or [\"/bin/bash\", \"-c\", \"(exec myscript.sh)\"] | [optional]
**arguments** | Option<**Vec<String>**> | Arguments for the Pod's command. | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Environment variables to inject into k8 pod. Use `${pods:secrets:KEY}` to reference secret_map entries. | [optional]
**secret_map** | Option<**std::collections::HashMap<String, String>**> | Map of keys to secret values. Syntax: ${secret:name} (user secret), ${secret:user:name} (explicit owner). Reference in environment_variables via ${pods:secrets:KEY}. Resolved at pod start. | [optional]
**status_requested** | Option<**String**> | Status requested by user, `ON`, `OFF`, or `RESTART`. | [optional]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::VolumeMountsValue>**](VolumeMountsValue.md)> | Volume mounts keyed by mount_path. Values are VolumeMount objects (see schema) or null (to remove inherited mount). Ex: {\"/data\": {\"type\": \"tapisvolume\", \"source_id\": \"myvolume\"}, \"/etc/config.ini\": {\"type\": \"ephemeral\", \"config_content\": \"key=value\"}} | [optional]
**time_to_stop_default** | Option<**i32**> | Default time (sec) for pod to run from instance start. -1 for unlimited. 12 hour default. | [optional]
**time_to_stop_instance** | Option<**i32**> | Time (sec) for pod to run from instance start. Reset each time instance is started. -1 for unlimited. None uses default. | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::Networking>**](Networking.md)> | Networking information. {\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}} | [optional]
**resources** | Option<[**models::ModelsPodsResources**](ModelsPodsResources.md)> | Pod resource management {\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpu\": 0} | [optional]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]
**template_overrides** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Partial overrides for template values. Override volume_mounts or secret_map values without rewriting full template field. Ex: {\"volume_mounts\": {\"/data\": {\"source_id\": \"my-vol\"}}, \"secret_map\": {\"DB_PASS\": \"${secret:mypass}\"}} | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



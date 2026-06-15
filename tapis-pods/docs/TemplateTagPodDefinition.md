# TemplateTagPodDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<**String**> | Which docker image to use, must be on allowlist, check /pods/images for list. | [optional]
**template** | Option<**String**> | Name of template to base this template off of. | [optional]
**description** | Option<**String**> | Description of this pod. | [optional]
**command** | Option<**Vec<String>**> | Command to run in pod. ex. `[\"sleep\", \"5000\"]` or `[\"/bin/bash\", \"-c\", \"(exec myscript.sh)\"]` | [optional]
**arguments** | Option<**Vec<String>**> | Arguments for the Pod's command. | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Environment variables to inject into pod. Use `${pods:secrets:KEY}` to reference secret_map entries. | [optional][default to {}]
**secret_map** | Option<**std::collections::HashMap<String, String>**> | Map of keys to secret references or placeholders. Use ${secret:name} for user secrets, ${pods:default:val:?desc} for placeholders with defaults, ${:?desc} for required placeholders. Secrets resolved at pod start. | [optional][default to {}]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::VolumeMountsValue>**](VolumeMountsValue.md)> | Volume mounts keyed by mount_path. For templates, tapisvolume/tapissnapshot MUST use placeholder source_id (e.g., \"${:?Description}\"). Ex: {\"/data\": {\"type\": \"tapisvolume\", \"source_id\": \"${:?User data volume}\"}, \"/etc/config.ini\": {\"type\": \"ephemeral\", \"config_content\": \"key=value\"}} | [optional][default to {}]
**time_to_stop_default** | Option<**i32**> | Default time (sec) for pod to run from instance start. -1 for unlimited. 12 hour default. | [optional]
**time_to_stop_instance** | Option<**i32**> | Time (sec) for pod to run from instance start. Reset each time instance is started. -1 for unlimited. None uses default. | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::Networking>**](Networking.md)> | Networking information. `{\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}}` | [optional][default to {}]
**resources** | Option<[**models::ModelsTemplatesTagsResources**](ModelsTemplatesTagsResources.md)> | Pod resource management `{\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpus\": 0}` | [optional][default to {}]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# TemplateTagPodDefinition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**image** | Option<**String**> |  | [optional]
**template** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**command** | Option<**Vec<String>**> |  | [optional]
**arguments** | Option<**Vec<String>**> |  | [optional]
**environment_variables** | Option<**std::collections::HashMap<String, serde_json::Value>**> | Environment variables to inject into pod. Use `${pods:secrets:KEY}` to reference secret_map entries. | [optional][default to {}]
**secret_map** | Option<**std::collections::HashMap<String, String>**> | Map of keys to secret references or placeholders. Use ${secret:name} for user secrets, ${pods:default:val:?desc} for placeholders with defaults, ${:?desc} for required placeholders. Secrets resolved at pod start. | [optional][default to {}]
**volume_mounts** | Option<[**std::collections::HashMap<String, models::VolumeMountsValue>**](VolumeMountsValue.md)> | Volume mounts keyed by mount_path. For templates, tapisvolume/tapissnapshot MUST use placeholder source_id (e.g., \"${:?Description}\"). Ex: {\"/data\": {\"type\": \"tapisvolume\", \"source_id\": \"${:?User data volume}\"}, \"/etc/config.ini\": {\"type\": \"ephemeral\", \"config_content\": \"key=value\"}} | [optional][default to {}]
**time_to_stop_default** | Option<**i32**> |  | [optional]
**time_to_stop_instance** | Option<**i32**> |  | [optional]
**networking** | Option<[**std::collections::HashMap<String, models::ModelsTemplatesTagsNetworking>**](ModelsTemplatesTagsNetworking.md)> | Networking information. `{\"url_suffix\": {\"protocol\": \"http\"  \"tcp\", \"port\": int}}` | [optional][default to {}]
**resources** | Option<[**models::ModelsTemplatesTagsResources**](ModelsTemplatesTagsResources.md)> | Pod resource management `{\"cpu_limit\": 3000, \"mem_limit\": 3000, \"cpu_request\": 500, \"mem_limit\": 500, \"gpus\": 0}` | [optional][default to {}]
**compute_queue** | Option<**String**> | Queue to run pod in. `default` is the default queue. | [optional][default to default]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



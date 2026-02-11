# VolumeMount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** | Type of mount: 'tapisvolume', 'tapissnapshot', 'ephemeral', or 'pvc'. | 
**source_id** | Option<**String**> |  | [optional]
**mounted_by** | Option<**String**> |  | [optional]
**sub_path** | Option<**String**> | Sub-path within the source volume/snapshot to mount. Not used for ephemeral. | [optional][default to ]
**read_only** | Option<**bool**> |  | [optional]
**config_content** | Option<**String**> |  | [optional]
**config_permissions** | Option<**String**> | Unix file permissions for config file (e.g., '0644', '0600'). | [optional][default to 0644]
**config_filename** | Option<**String**> |  | [optional]
**config_update_mode** | Option<**String**> | Config update behavior: 'always' recreates config on each pod start, 'once' only creates if file/ConfigMap doesn't exist. | [optional][default to always]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# VolumeMount

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** | Type of mount: 'tapisvolume', 'tapissnapshot', 'ephemeral', or 'pvc'. (enum: tapisvolume, tapissnapshot, ephemeral, pvc) | 
**source_id** | Option<**String**> | ID of the volume, snapshot, or PVC to mount. Required for tapisvolume/tapissnapshot/pvc. | [optional]
**mounted_by** | Option<**String**> | Service-managed: Username who mounted this volume. Set automatically when volume_mounts are created/updated. | [optional]
**sub_path** | Option<**String**> | Sub-path within the source volume/snapshot to mount. Not used for ephemeral. | [optional][default to ]
**read_only** | Option<**bool**> | If true, mount will be read-only. Default: False for volumes/pvc, True for snapshots/ephemeral. | [optional]
**config_content** | Option<**String**> | Config file content. For ephemeral: mounted as ConfigMap. For tapisvolume: written to NFS. Supports ${pods:secrets:KEY} interpolation. Max 1MB. | [optional]
**config_permissions** | Option<**String**> | Unix file permissions for config file (e.g., '0644', '0600'). | [optional][default to 0644]
**config_filename** | Option<**String**> | Filename for config file when using tapisvolume with config_content. Defaults to basename of mount_path. | [optional]
**config_update_mode** | Option<**String**> | Config update behavior: 'always' recreates config on each pod start, 'once' only creates if file/ConfigMap doesn't exist. | [optional][default to always]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# NewSnapshot

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**snapshot_id** | **String** | Name of this snapshot. | 
**source_volume_id** | **String** | The volume_id to use as source of snapshot. | 
**source_volume_path** | **String** | Path in source volume_id to make snapshot of | 
**destination_path** | Option<**String**> | Path to copy to. Snapshots of singular files require destination_path. | [optional][default to ]
**description** | Option<**String**> | Description of this snapshot. | [optional][default to ]
**size_limit** | Option<**i32**> | Size in MB to limit snapshot to. We'll start warning if you've gone past the limit. | [optional][default to 1024]
**cron** | Option<**String**> | cron bits | [optional][default to ]
**retention_policy** | Option<**String**> | retention_policy bits | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



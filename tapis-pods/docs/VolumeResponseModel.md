# VolumeResponseModel

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**volume_id** | **String** | Name of this volume. | 
**description** | Option<**String**> | Description of this volume. | [optional][default to ]
**size_limit** | Option<**i32**> | Size in MB to limit volume to. We'll start warning if you've gone past the limit. | [optional][default to 1024]
**size** | Option<**i32**> | Size of volume currently in MB | [optional][default to 0]
**status** | Option<**String**> | Current status of volume. | [optional][default to REQUESTED]
**creation_ts** | Option<**String**> |  | [optional]
**update_ts** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



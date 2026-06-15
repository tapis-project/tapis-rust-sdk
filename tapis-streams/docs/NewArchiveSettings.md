# NewArchiveSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**project_id** | Option<**String**> | ID of Project to archive | [optional]
**system_id** | Option<**String**> | ID os System ID to send archive too. | [optional]
**path** | Option<**String**> | The directory path to write archive to on selected System. | [optional]
**archive_format** | Option<**ArchiveFormat**> | The archive file format - zip is currently supported (enum: zip) | [optional]
**data_format** | Option<**DataFormat**> | The data measurement format - csv or JSON is currently supported (enum: csv, json) | [optional]
**frequency** | Option<**Frequency**> | How often should this archive occur - currently supports one-time_now (enum: one-time) | [optional]
**range** | Option<**Range**> | The range of data to archive_type - all or custom supported (enum: custom, all) | [optional]
**start_date** | Option<**String**> | A ISO8601 formatted datetime string for the starting time for archival data- only for custom range | [optional]
**end_date** | Option<**String**> | A ISO8601 formatted datetime string for the end time for archival data - only for custom range | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



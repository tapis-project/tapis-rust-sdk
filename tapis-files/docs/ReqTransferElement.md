# ReqTransferElement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**source_uri** | **String** |  | 
**destination_uri** | **String** |  | 
**optional** | Option<**bool**> | Allow the full transfer to succeed even if this element fails. | [optional][default to false]
**transfer_type** | Option<**TransferType**> | If this value is set to anything other than TRANSFER, both the source and target MUST, be on the same tapis system.  If the value is SERVICE_MOVE_DIRECTORY_CONTENTS, the  source URI is expected to be a directory, and the contents of that directory will be transfered to the target URI.  The target must be an existing directory or the operation will fail.  If the value is SERVICE_MOVE_FILE_OR_DIRECTORY the file or directory will be moved to the target URI.  If the target already exists, it will be overwritten.  (enum: TRANSFER, SERVICE_MOVE_DIRECTORY_CONTENTS, SERVICE_MOVE_FILE_OR_DIRECTORY) | [optional]
**src_shared_ctx** | Option<**String**> |  | [optional]
**dest_shared_ctx** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



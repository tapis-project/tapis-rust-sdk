# ReqPostChildSystem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | Short descriptive name for the system that is unique within the tenant. | [optional]
**effective_user_id** | **String** | Username to use when accessing the system. A specific user (such as a service account) or the dynamic user ``${apiUserId}``. | [default to ${apiUserId}]
**root_dir** | **String** | Effective root directory to be used when listing files or moving files to and from the system. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



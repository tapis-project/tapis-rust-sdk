# GetServerMetadata200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**version** | Option<**String**> | Version of the API | [optional]
**message** | Option<**String**> | Brief description of the response | [optional]
**status** | Option<**Status**> | Whether the request was a success or failure. (enum: success, failure) | [optional]
**metadata** | Option<**serde_json::Value**> | Metadata about the result object, including pagination information | [optional]
**result** | Option<[**models::OAuth2Metadata**](OAuth2Metadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



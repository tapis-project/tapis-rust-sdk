# TemplateTag

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | **String** | template_id this tag is linked to | 
**pod_definition** | Option<[**models::TemplateTagPodDefinition**](TemplateTagPodDefinition.md)> | Pod definition for this template. | [optional][default to {}]
**commit_message** | Option<**String**> | Commit message for this template tag. | [optional][default to ]
**tag** | Option<**String**> | Tag for this template. Default is 'latest'. | [optional][default to latest]
**description** | Option<**String**> | Description of template tag. | [optional][default to ]
**archive_message** | Option<**String**> | If set, metadata message to give users of this template tag. | [optional][default to ]
**tag_timestamp** | Option<**String**> | tag@timestamp for this template tag. | [optional][default to ]
**added_by** | Option<**String**> | User who added this template tag. | [optional][default to ]
**creation_ts** | Option<**String**> |  | [optional]
**tenant_id** | Option<**String**> | Tapis tenant used during creation of this template tag. | [optional][default to ]
**site_id** | Option<**String**> | Tapis site used during creation of this template tag. | [optional][default to ]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



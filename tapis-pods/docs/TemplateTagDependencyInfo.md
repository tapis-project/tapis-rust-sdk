# TemplateTagDependencyInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_id** | Option<**String**> |  | [optional]
**tag_timestamp** | Option<**String**> | Tag timestamp identifier. | [optional][default to ]
**full_tag** | Option<**String**> | Full tag reference (template_id:tag@timestamp). | [optional][default to ]
**dependant_pods** | Option<**Vec<String>**> | List of pod IDs that depend on this template tag. | [optional][default to []]
**dependant_pod_count** | Option<**i32**> | Number of pods that depend on this template tag. | [optional][default to 0]
**dependant_tags** | Option<**Vec<String>**> | List of template tag timestamps that depend on this template tag. | [optional][default to []]
**dependant_tags_count** | Option<**i32**> | Number of template tags that depend on this template tag. | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



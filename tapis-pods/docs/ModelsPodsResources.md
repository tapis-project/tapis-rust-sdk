# ModelsPodsResources

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_request** | Option<**i32**> | CPU allocation pod requests at startup. In millicpus (m). 1000 = 1 cpu. | [optional][default to 250]
**cpu_limit** | Option<**i32**> | CPU allocation pod is allowed to use. In millicpus (m). 1000 = 1 cpu. | [optional][default to 2000]
**mem_request** | Option<**i32**> | Memory allocation pod requests at startup. In megabytes (Mi) | [optional][default to 256]
**mem_limit** | Option<**i32**> | Memory allocation pod is allowed to use. In megabytes (Mi) | [optional][default to 3072]
**gpus** | Option<**i32**> | GPU allocation pod is allowed to use. In integers of GPUs. (we only have 1 currently ;) ) | [optional][default to 0]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



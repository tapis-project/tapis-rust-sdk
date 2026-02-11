# \JobsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobsApi.md#cancel_job) | **POST** /jobs/{jobUuid}/cancel | 
[**get_job**](JobsApi.md#get_job) | **GET** /jobs/{jobUuid} | 
[**get_job_history**](JobsApi.md#get_job_history) | **GET** /jobs/{jobUuid}/history | 
[**get_job_list**](JobsApi.md#get_job_list) | **GET** /jobs/list | 
[**get_job_output_download**](JobsApi.md#get_job_output_download) | **GET** /jobs/{jobUuid}/output/download/{outputPath} | 
[**get_job_output_list**](JobsApi.md#get_job_output_list) | **GET** /jobs/{jobUuid}/output/list/{outputPath} | 
[**get_job_search_list**](JobsApi.md#get_job_search_list) | **GET** /jobs/search | 
[**get_job_search_list_by_post_sql_str**](JobsApi.md#get_job_search_list_by_post_sql_str) | **POST** /jobs/search | 
[**get_job_status**](JobsApi.md#get_job_status) | **GET** /jobs/{jobUuid}/status | 
[**get_resubmit_request_json**](JobsApi.md#get_resubmit_request_json) | **GET** /jobs/{jobUuid}/resubmit_request | 
[**hide_job**](JobsApi.md#hide_job) | **POST** /jobs/{jobUuid}/hide | 
[**patch_job_annotations**](JobsApi.md#patch_job_annotations) | **PATCH** /jobs/{jobUuid}/annotations | 
[**put_job_annotations**](JobsApi.md#put_job_annotations) | **PUT** /jobs/{jobUuid}/annotations | 
[**resubmit_job**](JobsApi.md#resubmit_job) | **POST** /jobs/{jobUuid}/resubmit | 
[**send_event**](JobsApi.md#send_event) | **POST** /jobs/{jobUuid}/sendEvent | 
[**submit_job**](JobsApi.md#submit_job) | **POST** /jobs/submit | 
[**unhide_job**](JobsApi.md#unhide_job) | **POST** /jobs/{jobUuid}/unhide | 



## cancel_job

> models::RespCancelJob cancel_job(job_uuid, body)


Cancel a previously submitted job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RespCancelJob**](RespCancelJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::RespGetJob get_job(job_uuid)


Retrieve a previously submitted job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |

### Return type

[**models::RespGetJob**](RespGetJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_history

> models::RespJobHistory get_job_history(job_uuid, limit, skip)


Retrieve history of a previously submitted job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |

### Return type

[**models::RespJobHistory**](RespJobHistory.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_list

> models::RespGetJobList get_job_list(limit, skip, start_after, order_by, compute_total, list_type)


Retrieve list of jobs for which the user is the job owner, creator or a tenant administrator.  Also list the jobs that are shared with the user.  listType allowed are: MY_JOBS, SHARED_JOBS, ALL_JOBS

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**start_after** | Option<**i32**> |  |  |
**order_by** | Option<**String**> |  |  |
**compute_total** | Option<**bool**> |  |  |
**list_type** | Option<**String**> |  |  |[default to MY_JOBS]

### Return type

[**models::RespGetJobList**](RespGetJobList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_output_download

> std::path::PathBuf get_job_output_download(job_uuid, output_path, compress, format, allow_if_running)


Download a job's output files using the job's UUID. By default, the job must be in a terminal state (FINISHED or FAILED or CANCELLED) for this command to execute. To execute when a job is not in a terminal state--and possibly receive incomplete results--set _allowIfRunning=true_.    The caller must be the job owner, creator or a tenant administrator. The _outputPath_ is always relative to the job output directory and must end with a '/'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**output_path** | **String** |  | [required] |[default to ]
**compress** | Option<**bool**> |  |  |[default to false]
**format** | Option<**String**> |  |  |[default to zip]
**allow_if_running** | Option<**bool**> |  |  |[default to false]

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_output_list

> models::RespGetJobOutputList get_job_output_list(job_uuid, output_path, limit, skip, allow_if_running)


Retrieve a job's output file listing using the job's UUID. By default, the job must be in a terminal state (FINISHED or FAILED or CANCELLED) for this command to execute. To execute when a job is not in a terminal state--and possibly receive incomplete results--set _allowIfRunning=true_.    The caller must be the job owner, creator or a tenant administrator. The _outputPath_ is always relative to the job output directory and must end with a '/'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**output_path** | **String** |  | [required] |[default to ]
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**allow_if_running** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::RespGetJobOutputList**](RespGetJobOutputList.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_search_list

> models::RespJobSearchAllAttributes get_job_search_list(limit, skip, start_after, order_by, compute_total, select, list_type)


Retrieve list of jobs for the user based on search conditions in the query paramter on the dedicated search end-point.  The caller must be the job owner, creator or a tenant administrator.   List of Jobs shared with the user can also be searched

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**start_after** | Option<**i32**> |  |  |
**order_by** | Option<**String**> |  |  |
**compute_total** | Option<**bool**> |  |  |
**select** | Option<**String**> |  |  |
**list_type** | Option<**String**> |  |  |[default to MY_JOBS]

### Return type

[**models::RespJobSearchAllAttributes**](RespJobSearchAllAttributes.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_search_list_by_post_sql_str

> models::RespJobSearchAllAttributes get_job_search_list_by_post_sql_str(limit, skip, start_after, order_by, compute_total, select, list_type, body)


Retrieve list of jobs for the user based on search conditions in the request body and pagination information from the query paramter on the dedicated search end-point.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> |  |  |
**skip** | Option<**i32**> |  |  |
**start_after** | Option<**i32**> |  |  |
**order_by** | Option<**String**> |  |  |
**compute_total** | Option<**bool**> |  |  |
**select** | Option<**String**> |  |  |
**list_type** | Option<**String**> |  |  |[default to MY_JOBS]
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RespJobSearchAllAttributes**](RespJobSearchAllAttributes.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_status

> models::RespGetJobStatus get_job_status(job_uuid)


Retrieve status of a previously submitted job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |

### Return type

[**models::RespGetJobStatus**](RespGetJobStatus.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_resubmit_request_json

> models::RespGetResubmit get_resubmit_request_json(job_uuid)


Get Resubmit request for of a job in JSON format.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |

### Return type

[**models::RespGetResubmit**](RespGetResubmit.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hide_job

> models::RespHideJob hide_job(job_uuid, body)


Hide a job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RespHideJob**](RespHideJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_job_annotations

> models::RespJobAnnotations patch_job_annotations(job_uuid, req_job_annotation)


Adding tags or notes with the following behavior: ## Behavior   - **Tags**: Repetitive tags will be ignored (case-sensitive merging)   - **Notes**: Old notes will be updated with new values if the key is the same   - If any of the tags/notes field is missing in the request body, the corresponding field in jobs will not be touched.  ## Limits   - Total number of bytes of tags/notes: **128KB maximum**.    - Depending on the byte-length of the characters, tags and notes can have 32768 (4-byte) - 131072 (ASCII) UTF-8 characters.   - Overall, there is no guarantee of success if tags/notes exceed 32768 UTF-8 characters.  ## Authorization    - Caller must be the job owner, creator, or tenant administrator  **Note**: This operation merges new data with existing annotations rather than replacing them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**req_job_annotation** | [**ReqJobAnnotation**](ReqJobAnnotation.md) |  | [required] |

### Return type

[**models::RespJobAnnotations**](RespJobAnnotations.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_job_annotations

> models::RespJobAnnotations put_job_annotations(job_uuid, req_job_annotation)


Replace tags or notes with the provided values.  ## Behavior   - **Tags**: Completely replaces existing tags with the provided array   - **Notes**: Completely replaces existing notes with the provided JSON object   - **Clearing Data**:      - Provide empty array `[]` for tags to remove all tags     - Provide empty object `{}` for notes to remove all notes   - If any of the tags/notes field is missing in the request body, the corresponding field in jobs will not be touched.  ## Limits   - Total number of bytes of tags/notes: **128KB maximum**.    - Depending on the byte-length of the characters, tags and notes can have 32768 (4-byte) - 131072 (ASCII) UTF-8 characters.   - Overall, there is no guarantee of success if tags/notes exceed 32768 UTF-8 characters.  ## Authorization   - The caller must be the job owner, creator, or a tenant administrator.  **Note**: This operation completely replaces existing annotations rather than merging them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**req_job_annotation** | [**ReqJobAnnotation**](ReqJobAnnotation.md) |  | [required] |

### Return type

[**models::RespJobAnnotations**](RespJobAnnotations.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resubmit_job

> models::RespSubmitJob resubmit_job(job_uuid, body)


Resubmit a job for execution using the job's original parameters.  The main phases of job execution are:    - validate input   - check resource availability   - stage input files   - stage application code   - launch application   - monitor application   - archive application output  When a job is submitted its request payload is captured and available for resubmission using this API. The resubmitted job is assigned a new UUID and does not reference or have any special access to the original job's information once the orginal job's request is copied. The resubmitted job's execution can differ from the original job's if the application, system or other aspects of the execution environment have changed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RespSubmitJob**](RespSubmitJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_event

> models::RespBasic send_event(job_uuid, req_user_event)


Send a user event to an active job. The job must be in the same tenant as the caller, but no other authorization is needed. If the job has terminated the request will be rejected. The caller must specify a payload of non-empty string data in the *eventData* field. The *eventDetail* field can be set to further qualify the type of user event, which is useful when filtering events. If not provided the *eventDetail* defaults to \"DEFAULT\".  Subscribers that register interest in events of type JOB_USER_EVENT will receive a notification as a result of this call.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**req_user_event** | [**ReqUserEvent**](ReqUserEvent.md) |  | [required] |

### Return type

[**models::RespBasic**](RespBasic.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_job

> models::RespSubmitJob submit_job(req_submit_job)


Submit a job for execution.  The main phases of job execution are:    - validate input   - check resource availability   - stage input files   - stage application code   - launch application   - monitor application   - archive application output  At a minimum, the job name, application ID and application version must be specified in the request payload. The optional parameters available in a job request provide great flexibility but must be considered in the context of the application and system definitions. The actual values used during job execution are a combination of the values in this request and those specified in the job's application and system definitions. It's often desirable to keep the submission request simple by specifying common values in these other two definitions. See the [Job Submission Request](https://tapis.readthedocs.io/en/latest/technical/jobs.html#the-job-submission-request) documentation for details. The total number of tags of a job is limited to be 128 and the total size of tags/notes is limited to be 128K bytes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**req_submit_job** | [**ReqSubmitJob**](ReqSubmitJob.md) |  | [required] |

### Return type

[**models::RespSubmitJob**](RespSubmitJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unhide_job

> models::RespHideJob unhide_job(job_uuid, body)


Un-hide a job by its UUID.  The caller must be the job owner, creator or a tenant administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_uuid** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

[**models::RespHideJob**](RespHideJob.md)

### Authorization

[TapisJWT](../README.md#TapisJWT)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


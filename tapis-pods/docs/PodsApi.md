# \PodsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_pod**](PodsApi.md#create_pod) | **POST** /pods | create_pod
[**delete_pod**](PodsApi.md#delete_pod) | **DELETE** /pods/{pod_id} | delete_pod
[**download_from_pod**](PodsApi.md#download_from_pod) | **GET** /pods/{pod_id}/download_from_pod{url_path} | Download a file from the pod's filesystem
[**exec_pod_commands**](PodsApi.md#exec_pod_commands) | **POST** /pods/{pod_id}/exec | exec_pod_commands
[**get_derived_pod**](PodsApi.md#get_derived_pod) | **GET** /pods/{pod_id}/derived | get_derived_pod
[**get_pod**](PodsApi.md#get_pod) | **GET** /pods/{pod_id} | get_pod
[**get_pod_credentials**](PodsApi.md#get_pod_credentials) | **GET** /pods/{pod_id}/credentials | get_pod_credentials
[**get_pod_logs**](PodsApi.md#get_pod_logs) | **GET** /pods/{pod_id}/logs | get_pod_logs
[**list_files_in_pod**](PodsApi.md#list_files_in_pod) | **GET** /pods/{pod_id}/list_files{url_path} | List files in the pod's filesystem
[**list_pods**](PodsApi.md#list_pods) | **GET** /pods | list_pods
[**pod_auth**](PodsApi.md#pod_auth) | **GET** /pods/{pod_id_net}/auth | pod_auth
[**pod_auth_callback**](PodsApi.md#pod_auth_callback) | **GET** /pods/{pod_id_net}/auth/callback | pod_auth_callback
[**restart_pod**](PodsApi.md#restart_pod) | **GET** /pods/{pod_id}/restart | restart_pod
[**save_pod_as_template_tag**](PodsApi.md#save_pod_as_template_tag) | **POST** /pods/{pod_id_net}/save_pod_as_template_tag | save_pod_as_template_tag
[**start_pod**](PodsApi.md#start_pod) | **GET** /pods/{pod_id}/start | start_pod
[**stop_pod**](PodsApi.md#stop_pod) | **GET** /pods/{pod_id}/stop | stop_pod
[**update_pod**](PodsApi.md#update_pod) | **PUT** /pods/{pod_id} | update_pod
[**upload_to_pod**](PodsApi.md#upload_to_pod) | **POST** /pods/{pod_id}/upload_to_pod | Upload a file directly into the pod's filesystem



## create_pod

> models::PodResponse create_pod(new_pod)
create_pod

Create a pod with inputted information.  Notes: - Author will be given ADMIN level permissions to the pod. - status_requested defaults to \"ON\". So pod will immediately begin creation.  Returns new pod object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_pod** | [**NewPod**](NewPod.md) |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_pod

> models::PodDeleteResponse delete_pod(pod_id)
delete_pod

Delete a pod.  Returns \"\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodDeleteResponse**](PodDeleteResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_from_pod

> serde_json::Value download_from_pod(pod_id, url_path, path)
Download a file from the pod's filesystem

Download a file from a specific path inside the pod using Kubernetes exec.  Path options (use one, not both): - URL path: Relative paths only (e.g., /download_from_pod/myfile.txt -> \"myfile.txt\") - Query parameter: Absolute paths allowed (e.g., ?path=/tmp/myfile.txt -> \"/tmp/myfile.txt\")  Notes: - Pod must have /bin/sh and base64 available (most standard images include these) - Distroless or minimal images without a shell or base64 will not work with this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** | Unique identifier for the pod. | [required] |
**url_path** | **String** | Path to the file inside the pod to download. | [required] |
**path** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## exec_pod_commands

> serde_json::Value exec_pod_commands(pod_id, execute_pod_commands)
exec_pod_commands

Execute one or more commands in a pod.  Accepts either: - Single command: [\"sleep\", \"5\"] - Multiple commands: [[\"sleep\", \"5\"], [\"echo\", \"hello\"]]  Executes commands synchronously in the pod: - Each command runs sequentially - Total request time = sum of all command execution times - Request remains open until all commands complete - Returns consolidated results for all commands  Response includes: - Individual command outputs - Success/failure status - Execution duration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**execute_pod_commands** | [**ExecutePodCommands**](ExecutePodCommands.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_derived_pod

> models::PodResponse get_derived_pod(pod_id, include_configs, resolve_secrets)
get_derived_pod

Derive a pod's final definition if templates are used.  Returns final pod definition to be used for pod creation.  Use resolve_secrets=true (admin only) to preview how secrets will be interpolated into environment_variables and config_content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**include_configs** | Option<**bool**> | Include full config_content for volume mounts using field. Default: false (shows placeholder with size) |  |[default to false]
**resolve_secrets** | Option<**bool**> | Resolve and show actual secret values (admin only). Default: false. Use to preview how secrets will be interpolated. |  |[default to false]

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pod

> models::PodResponse get_pod(pod_id, include_configs, check_unresolved)
get_pod

Get a pod.  Returns retrieved pod object.  Use check_unresolved=true to detect any ${...} patterns that haven't been resolved.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**include_configs** | Option<**bool**> | Include full config_content for volume mounts using field. Default: false (shows placeholder with size) |  |[default to false]
**check_unresolved** | Option<**bool**> | Check for unresolved ${...} patterns and include in metadata. Default: True |  |[default to true]

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pod_credentials

> models::PodCredentialsResponse get_pod_credentials(pod_id)
get_pod_credentials

Get the credentials created for a pod.  Note: - These credentials are used in the case of templated pods, but for custom pods they're not.  Returns user accessible credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodCredentialsResponse**](PodCredentialsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pod_logs

> models::PodLogsResponse get_pod_logs(pod_id)
get_pod_logs

Get a pods stdout logs and action_logs.  Note: - Pod logs are only retrieved while pod is running. - If a pod is restarted or turned off and then on, the logs will be reset. - Action logs are detailed logs of actions taken on the pod.  Returns pod stdout logs and action logs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodLogsResponse**](PodLogsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_files_in_pod

> serde_json::Value list_files_in_pod(pod_id, url_path, path)
List files in the pod's filesystem

List files and directories at a specific path inside the pod using Kubernetes exec.  Path options (use one, not both): - URL path: Relative paths only (e.g., /list_files/mydir -> \"mydir\") - Query parameter: Absolute paths allowed (e.g., ?path=/tmp -> \"/tmp\")  Notes: - Pod must have /bin/sh and ls available (most standard images include these) - Distroless or minimal images without a shell will return a 500 error.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** | Unique identifier for the pod. | [required] |
**url_path** | **String** | Path to list files from inside the pod. | [required] |
**path** | Option<**String**> | Alternative query parameter for path. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_pods

> models::PodsResponse list_pods()
list_pods

Get all pods in your respective tenant and site that you have READ or higher access to.  Returns a list of pods.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PodsResponse**](PodsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_auth

> models::PodResponse pod_auth(pod_id_net)
pod_auth

Auth endpoint for each pod. When a networking object defines tapis_auth=True, this endpoint manages auth.  Traefik has a forwardAuth middleware for http routes. This redirects users, auth happens, if traefik gets 200 then traefik allows user to endpoint. Auth flow for a user getting to \"fastapi hello world\" pod at https://fastapi.pods.tacc.tapis.io.   1) User navigates to https://fastapi.pods.tacc.tapis.io   2) Traefik redirects user to https://tacc.tapis.io/v3/pods/fastapi/auth   3) Check if logged in via cookies, if logged in, respond 200 + set user defined headers. Otherwise...   4) Pods service creates client in correct tenant for user or updates client if it already exists. (we expect only one client in use at a time)   5) With client the /auth endpoint redirects users to https://tacc.tapis.io/v3/oauth2/authorize?client_id={client_id}&redirect_uri={callback_url}&response_type=code   6) User logs in via browser, authorizes client, redirects to callback_url at https://tacc.tapis.io/v3/pods/fastapi/auth/callback?code=CodeHere   7) Callback url exchanges code for token, gets username from token, sets X-Tapis-Token cookies, sets response headers according to tapis_auth_response_headers   8) User gets redirected back to https://fastapi.pods.tacc.tapis.io/{tapis_auth_return_path}, Traefik starts forwardAuth, user at this point should be authenticated   9) Auth endpoint responds with 200, sets headers specified by networking stanza, and users gets to fastapi hello world response.  users can specify:  - tapis_auth=True/False - Turns on auth  - tapis_auth_response_headers - dict[str] - headers to set on response and their values  - tapis_auth_allowed_users - list[str] - list of tapis users allowed to access pod  - tapis_auth_return_path - str - uri to return to after auth, default is \"passthrough\", which we save in cookies(?) and return to. x-forwarded-host?   - response headers need to be slightly modifiable to allow for different application requirements  - for example we have to pass username, but many apps require @email.bit, so user must be able to append to user.  - tapis_auth_response_headers: {\"X-Tapis-Username\": \"<<tapisusername>>@tapis.io\", \"FROM\": \"pods auth endpoint from <<tenant>>.<<site>>\", \"OAUTH2_USERNAME_KEY\": \"username\"}   - tapis_auth_allowed_users, checks username against .lower() of username list to make sure it's in list. otherwise deny

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id_net** | **String** |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pod_auth_callback

> models::PodResponse pod_auth_callback(pod_id_net)
pod_auth_callback

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id_net** | **String** |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## restart_pod

> models::PodResponse restart_pod(pod_id, grab_latest_template_tag)
restart_pod

Restart a pod.  Note: - Sets status_requested to RESTART. If pod status gets to STOPPED, status_requested will be flipped to ON. Health should then create new pod. - If grab_latest_template_tag is True, attempts to grab the latest version of the template tag if the pod has a template.  Returns updated pod object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**grab_latest_template_tag** | Option<**bool**> |  |  |[default to false]

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## save_pod_as_template_tag

> models::TemplateTagResponse save_pod_as_template_tag(pod_id_net, new_template_tag_from_pod)
save_pod_as_template_tag

Endpoint takes pod_id and derives a pod_definition to create a template tag from it. Allows users to save the configuration of a particular pod as a template tag.  POST data contains location to save the tag and tag creation data  Return the template tag object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id_net** | **String** |  | [required] |
**new_template_tag_from_pod** | [**NewTemplateTagFromPod**](NewTemplateTagFromPod.md) |  | [required] |

### Return type

[**models::TemplateTagResponse**](TemplateTagResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_pod

> models::PodResponse start_pod(pod_id)
start_pod

Start a pod.  Note: - Sets status_requested to ON. Pod will attempt to deploy.  Returns updated pod object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_pod

> models::PodResponse stop_pod(pod_id)
stop_pod

Stop a pod.  Note: - Sets status_requested to OFF. Pod will attempt to get to STOPPED status unless start_pod is ran.  Returns updated pod object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_pod

> models::PodResponse update_pod(pod_id, update_pod)
update_pod

Update a pod.  Note: - Pod will not be restarted, you must restart the pod for any pod-related changes to proliferate.  Returns updated pod object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**update_pod** | [**UpdatePod**](UpdatePod.md) |  | [required] |

### Return type

[**models::PodResponse**](PodResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_to_pod

> serde_json::Value upload_to_pod(pod_id, file, dest_path)
Upload a file directly into the pod's filesystem

Upload a file to a specific path inside the pod using Kubernetes exec.  Notes: - Pod must have /bin/sh available (most standard images include this) - Distroless or minimal images without a shell will not work with this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pod_id** | **String** |  | [required] |
**file** | **std::path::PathBuf** |  | [required] |
**dest_path** | **String** |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


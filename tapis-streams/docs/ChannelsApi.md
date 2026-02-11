# \ChannelsApi

All URIs are relative to *http://localhost:5000*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_channels**](ChannelsApi.md#create_channels) | **POST** /v3/streams/channels | Create channels.
[**delete_channel**](ChannelsApi.md#delete_channel) | **DELETE** /v3/streams/channels/{channel_id} | Delete a channel
[**get_channel**](ChannelsApi.md#get_channel) | **GET** /v3/streams/channels/{channel_id} | Get channels details
[**list_alerts**](ChannelsApi.md#list_alerts) | **GET** /v3/streams/channels/{channel_id}/alerts | List alerts for given channel id
[**list_channels**](ChannelsApi.md#list_channels) | **GET** /v3/streams/channels | List channels.
[**update_channel**](ChannelsApi.md#update_channel) | **PUT** /v3/streams/channels/{channel_id} | Update a channel
[**update_status**](ChannelsApi.md#update_status) | **POST** /v3/streams/channels/{channel_id} | Update channel status



## create_channels

> models::ListChannels200Response create_channels(new_channel)
Create channels.

Create channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_channel** | [**NewChannel**](NewChannel.md) |  | [required] |

### Return type

[**models::ListChannels200Response**](list_channels_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_channel

> models::GetChannel200Response delete_channel(channel_id)
Delete a channel

Delete a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Unique channel id | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> models::GetChannel200Response get_channel(channel_id)
Get channels details

Get details of a specific channel by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Unique channel id | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_alerts

> models::ListAlerts200Response list_alerts(channel_id)
List alerts for given channel id

Get details of a specific channel by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Unique channel id | [required] |

### Return type

[**models::ListAlerts200Response**](list_alerts_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channels

> models::ListChannels200Response list_channels(query, limit, skip)
List channels.

List channels.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | Option<**String**> | a formated query string for channel. |  |
**limit** | Option<**i32**> | limit the number of records returned. |  |
**skip** | Option<**i32**> | index (skip) to start list. |  |

### Return type

[**models::ListChannels200Response**](list_channels_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel

> models::GetChannel200Response update_channel(channel_id, update_channel)
Update a channel

Update a channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Unique channel id | [required] |
**update_channel** | [**UpdateChannel**](UpdateChannel.md) |  | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_status

> models::GetChannel200Response update_status(channel_id, update_channel_status)
Update channel status

Update channel status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**channel_id** | **String** | Unique channel id | [required] |
**update_channel_status** | [**UpdateChannelStatus**](UpdateChannelStatus.md) |  | [required] |

### Return type

[**models::GetChannel200Response**](get_channel_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


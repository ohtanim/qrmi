# \AnnouncementsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_announcement**](AnnouncementsApi.md#get_announcement) | **GET** /announcements/{announcement_id} | Get selected announcement
[**get_announcements_list**](AnnouncementsApi.md#get_announcements_list) | **GET** /announcements | Get announcements list from backend



## get_announcement

> models::AnnouncementsGetAnnouncementResponse get_announcement(announcement_id)
Get selected announcement

Get selected announcement

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**announcement_id** | **i32** | announcement ID | [required] |

### Return type

[**models::AnnouncementsGetAnnouncementResponse**](announcements.GetAnnouncementResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_announcements_list

> models::AnnouncementsGetAnnouncementsListResponse get_announcements_list(offset, limit, order, current_time)
Get announcements list from backend

Get announcements list from backend

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**offset** | Option<**String**> | offset information |  |
**limit** | Option<**String**> | Limit information |  |
**order** | Option<**String**> | Specify order according to start time |  |[default to ASC]
**current_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Allows to filter the list of announcements to fetch by provided time. If specified only announcements with start_time <= current_time and end_time >= current_time are returned. |  |

### Return type

[**models::AnnouncementsGetAnnouncementsListResponse**](announcements.GetAnnouncementsListResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


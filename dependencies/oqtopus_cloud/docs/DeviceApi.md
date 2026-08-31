# \DeviceApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_device**](DeviceApi.md#get_device) | **GET** /devices/{device_id} | Get specified device details
[**list_devices**](DeviceApi.md#list_devices) | **GET** /devices | List available devices



## get_device

> models::DevicesDeviceInfo get_device(device_id)
Get specified device details

get device

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device_id** | **String** | Device identifier | [required] |

### Return type

[**models::DevicesDeviceInfo**](devices.DeviceInfo.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_devices

> Vec<models::DevicesDeviceInfo> list_devices()
List available devices

List available devices

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DevicesDeviceInfo>**](devices.DeviceInfo.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


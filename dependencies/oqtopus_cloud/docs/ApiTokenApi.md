# \ApiTokenApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_api_token**](ApiTokenApi.md#create_api_token) | **POST** /api-token | create api token
[**delete_api_token**](ApiTokenApi.md#delete_api_token) | **DELETE** /api-token | delete api token
[**get_api_token_status**](ApiTokenApi.md#get_api_token_status) | **GET** /api-token/status | get api token status



## create_api_token

> models::ApiTokenApiToken create_api_token()
create api token

Create api token

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiTokenApiToken**](api-token.ApiToken.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_api_token

> delete_api_token()
delete api token

Delete api token

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_token_status

> models::ApiTokenApiTokenStatus get_api_token_status()
get api token status

Get api token status

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ApiTokenApiTokenStatus**](api-token.ApiTokenStatus.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


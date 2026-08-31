# \UsersApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_current_user**](UsersApi.md#delete_current_user) | **DELETE** /users/me | Delete current user
[**get_current_user**](UsersApi.md#get_current_user) | **GET** /users/me | Get current user
[**update_current_user**](UsersApi.md#update_current_user) | **PATCH** /users/me | Update current user



## delete_current_user

> delete_current_user()
Delete current user

Delete current user

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


## get_current_user

> models::UsersGetOneUserResponse get_current_user()
Get current user

Get current user

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::UsersGetOneUserResponse**](users.GetOneUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_current_user

> models::UsersGetOneUserResponse update_current_user(users_update_user_request)
Update current user

Update current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**users_update_user_request** | Option<[**UsersUpdateUserRequest**](UsersUpdateUserRequest.md)> |  |  |

### Return type

[**models::UsersGetOneUserResponse**](users.GetOneUserResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


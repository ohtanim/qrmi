# \JobApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_job**](JobApi.md#cancel_job) | **POST** /jobs/{job_id}/cancel | Cancel job
[**delete_job**](JobApi.md#delete_job) | **DELETE** /jobs/{job_id} | Delete job
[**get_job**](JobApi.md#get_job) | **GET** /jobs/{job_id} | Get selected job
[**get_job_status**](JobApi.md#get_job_status) | **GET** /jobs/{job_id}/status | Get selected job's status
[**list_jobs**](JobApi.md#list_jobs) | **GET** /jobs | List all quantum jobs
[**register_job_id**](JobApi.md#register_job_id) | **POST** /jobs | Register new job
[**submit_job**](JobApi.md#submit_job) | **POST** /jobs/{job_id}/submit | Complete submission of a quantum job



## cancel_job

> models::SuccessSuccessResponse cancel_job(job_id)
Cancel job

Start a procedure to cancel quantum job.<br/><br/> Operation is valid only for job with status: submitted, ready or running.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job identifier | [required] |

### Return type

[**models::SuccessSuccessResponse**](success.SuccessResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_job

> models::SuccessSuccessResponse delete_job(job_id)
Delete job

Deletes quantum job and related result<br/><br/>Operation is valid only for job with status: succeeded, failed and cancelled. submitted, ready and running jobs must be cancelled before deletion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job identifier | [required] |

### Return type

[**models::SuccessSuccessResponse**](success.SuccessResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::JobsJob get_job(job_id)
Get selected job

Get selected job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job identifier | [required] |

### Return type

[**models::JobsJob**](jobs.Job.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job_status

> models::JobsGetJobStatusResponse get_job_status(job_id)
Get selected job's status

Get selected job's status

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job identifier | [required] |

### Return type

[**models::JobsGetJobStatusResponse**](jobs.GetJobStatusResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_jobs

> Vec<models::JobsJob> list_jobs(fields, start_time, end_time, status, q, page, size, order)
List all quantum jobs

By default, all available job's properties are returned. Use 'fields' parameter to specify exact list of properties to get for each job.  List of jobs can be filtered by submission time, status or search text with 'start_time', 'end_time', 'status' and 'q' parameters.  Jobs are fetched with the pagination mechanism. This can be configured with 'page' and 'perPage' parameters. Check response's 'Link' header for pagination details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<**String**> | Allows to specify an exact list of job properties to fetch for a single job. Each element of the list must be a valid name of job property.  If parameter is specified and requested job field is not defined for a job null is returned.  If parameter is omitted all available job properties are returned. Undefined job properties (null properties) are not included in the response. |  |
**start_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Allows to filter the list of jobs to fetch by submission time. If specified only jobs with submission time (submitted_at property) >= start_time are returned. |  |
**end_time** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Allows to filter the list of jobs to fetch by to submission time. If specified only jobs with submission time (submitted_at property) <= end_time are returned. |  |
**status** | Option<[**JobsJobStatus**](JobsJobStatus.md)> | Allows to filter the list of jobs to fetch by job's status. If specified only jobs which status is equal to provided status are returned. |  |
**q** | Option<**String**> | Allows to filter the list of jobs to fetch by job's id, name and description. If specified only jobs which id, name or description contains specified search string are returned. |  |
**page** | Option<**i32**> | Set jobs list page number to fetch. If requested page number exceeds number of all pages last page is returned. |  |[default to 1]
**size** | Option<**i32**> | Configure number of jobs per page |  |[default to 10]
**order** | Option<**String**> | Specify jobs order according to creation time (createdAt property) |  |[default to ASC]

### Return type

[**Vec<models::JobsJob>**](jobs.Job.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_job_id

> models::JobsRegisterJobResponse register_job_id()
Register new job

Register new job and generate a presigned URL to upload job information (`jobs.S3SubmitJobInfo`) to OQTOPUS cloud.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::JobsRegisterJobResponse**](jobs.RegisterJobResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submit_job

> models::SuccessSuccessResponse submit_job(job_id, jobs_submit_job_request)
Complete submission of a quantum job

Complete submission of a previously registered quantum job.  job_id must be created via 'POST /jobs' request.  Submit job information (`jobs.S3SubmitJobInfo`) must be formerly uploaded to OQTOPUS cloud using presigned URL received in 'POST /jobs' response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**job_id** | **String** | Job identifier | [required] |
**jobs_submit_job_request** | Option<[**JobsSubmitJobRequest**](JobsSubmitJobRequest.md)> | Quantum job to be submitted |  |

### Return type

[**models::SuccessSuccessResponse**](success.SuccessResponse.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


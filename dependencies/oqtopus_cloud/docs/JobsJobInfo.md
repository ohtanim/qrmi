# JobsJobInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**input** | **String** | Content of the file will match `jobs.S3SubmitJobInfo` schema. | 
**combined_program** | Option<**String**> | For multiprogramming jobs, this file contains the combined circuit. | [optional]
**result** | Option<**String**> | Content of the file will match `jobs.S3JobResult` schema. | [optional]
**transpile_result** | Option<**String**> | Content of the file will match `jobs.S3TranspileResult` schema. | [optional]
**sse_log** | Option<**String**> | File available only for sse jobs. | [optional]
**message** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



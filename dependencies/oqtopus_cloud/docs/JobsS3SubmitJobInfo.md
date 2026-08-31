# JobsS3SubmitJobInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**program** | Option<**Vec<String>**> | A list of OPENQASM3 program. Required for sampling, estimation and multiprogramming jobs. For non-multiprogramming jobs, this field is assumed to contain exactly one program. Otherwise, those programs are combined according to the multiprogramming machinery. | [optional]
**operator** | Option<[**Vec<models::JobsS3OperatorItem>**](JobsS3OperatorItem.md)> | Estimation operator. Required for estimation jobs. | [optional]
**sse_program** | Option<**String**> | SSE user program. Required for SSE jobs. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# JobsJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**description** | Option<**String**> |  | [optional]
**job_type** | Option<[**models::JobsJobType**](JobsJobType.md)> |  | [optional]
**status** | Option<[**models::JobsJobStatus**](JobsJobStatus.md)> |  | [optional]
**device_id** | Option<**String**> |  | [optional]
**shots** | Option<**i32**> |  | [optional]
**job_info** | Option<[**models::JobsJobInfo**](JobsJobInfo.md)> |  | [optional]
**transpiler_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**simulator_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**mitigation_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**execution_time** | Option<**f64**> |  | [optional]
**submitted_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**ready_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**running_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**ended_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



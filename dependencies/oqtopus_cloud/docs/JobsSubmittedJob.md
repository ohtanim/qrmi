# JobsSubmittedJob

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**job_id** | **String** |  | 
**name** | **String** |  | 
**description** | Option<**String**> |  | [optional]
**job_type** | [**models::JobsJobType**](JobsJobType.md) |  | 
**status** | [**models::JobsJobStatus**](JobsJobStatus.md) |  | 
**device_id** | **String** |  | 
**shots** | **i32** |  | 
**job_info** | [**models::JobsJobInfo**](JobsJobInfo.md) |  | 
**transpiler_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**simulator_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**mitigation_info** | Option<**std::collections::HashMap<String, serde_json::Value>**> |  | [optional]
**execution_time** | Option<**f64**> |  | [optional]
**submitted_at** | **chrono::DateTime<chrono::FixedOffset>** |  | 
**ready_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**running_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]
**ended_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



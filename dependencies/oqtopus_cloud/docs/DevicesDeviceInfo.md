# DevicesDeviceInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**device_id** | **String** |  | 
**device_type** | **DeviceType** |  (enum: QPU, simulator) | 
**status** | **Status** |  (enum: available, unavailable) | 
**available_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Parameter mandatory and valid for 'unavailable' devices | [optional]
**n_pending_jobs** | **i32** |  | 
**n_qubits** | Option<**i32**> |  | [optional]
**basis_gates** | **Vec<String>** |  | 
**supported_instructions** | **Vec<String>** |  | 
**device_info** | Option<**String**> | json format calibration_data and n_nodes etc | [optional]
**calibrated_at** | Option<**chrono::DateTime<chrono::FixedOffset>**> | Parameter available only for `QPU` devices with available calibration data | [optional]
**description** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



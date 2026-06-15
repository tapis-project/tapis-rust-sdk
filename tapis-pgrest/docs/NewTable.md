# NewTable

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**table_name** | **String** | The name to be used for the table. | 
**root_url** | **String** | The url path to use for the root_url associated with the records in the table. | 
**columns** | [**models::TableColumns**](TableColumns.md) |  | 
**comments** | Option<**String**> | Text area to describe table. Returned when calling manage endpoints. | [optional]
**constraints** | Option<[**models::TableConstraints**](TableConstraints.md)> |  | [optional]
**enums** | Option<**serde_json::Value**> | Definition for enum to create. {enum_name\":\" [enum_val_1, ...]} | [optional]
**create** | Option<**bool**> | Whether the \"created\" endpoint (HTTP verb \"POST\") for creating new rows is available. | [optional][default to true]
**delete** | Option<**bool**> | Whether the \"delete\" endpoint (HTTP verb \"delete\") for deleting a specific row is available. | [optional][default to true]
**list_all** | Option<**bool**> | Whether the \"list all\" endpoint (HTTP verb \"GET\") for listing all rows in the table is available. | [optional][default to true]
**list_single** | Option<**bool**> | Whether the \"get row\" endpoint (HTTP verb \"GET\") for retrieving a specifc row in the table is available. | [optional][default to true]
**update** | Option<**bool**> | Whether the \"update\" endpoint (HTTP verb \"PUT\") for updating a specific row is available. | [optional][default to true]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


